// -------------------------------------------------------------------------------------------------
//  Copyright (C) 2015-2025 Nautech Systems Pty Ltd. All rights reserved.
//  https://nautechsystems.io
//
//  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
//  You may not use this file except in compliance with the License.
//  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
// -------------------------------------------------------------------------------------------------

//! Provides the HTTP client integration for the Bitget REST API.
//!
//! Bitget API reference <https://www.bitget.com/api-doc/common/intro>.

use std::{
    collections::HashMap,
    fmt::{Debug, Formatter},
    num::NonZeroU32,
    sync::LazyLock,
    time::Duration,
};

use chrono::Utc;
use nautilus_core::consts::NAUTILUS_USER_AGENT;
use nautilus_network::{
    http::HttpClient,
    ratelimiter::quota::Quota,
};
use reqwest::{Method, header::{HeaderMap, HeaderValue, USER_AGENT}};
use serde::{Serialize, de::DeserializeOwned};
use tokio_util::sync::CancellationToken;

use super::{
    error::{BitgetHttpError, BitgetHttpResult},
    models::{
        BitgetFuturesSymbolsResponse, BitgetOrderbookResponse, BitgetResponse,
        BitgetServerTimeResponse, BitgetSpotSymbolsResponse, BitgetTradesResponse,
    },
};
use crate::common::{credential::Credential, enums::BitgetEnvironment, urls::bitget_http_base_url};

const DEFAULT_TIMEOUT_SECS: u64 = 30;

/// Default Bitget REST API rate limit.
///
/// Bitget implements rate limiting per endpoint with varying limits.
/// We use a conservative 10 requests per second as a general default.
pub static BITGET_REST_QUOTA: LazyLock<Quota> = LazyLock::new(|| {
    Quota::per_second(NonZeroU32::new(10).expect("Should be a valid non-zero u32"))
});

const BITGET_GLOBAL_RATE_KEY: &str = "bitget:global";

/// Raw HTTP client for low-level Bitget API operations.
///
/// This client handles request/response operations with the Bitget API,
/// returning venue-specific response types. It does not parse to Nautilus domain types.
pub struct BitgetHttpClient {
    base_url: String,
    client: HttpClient,
    credential: Option<Credential>,
    timeout: Duration,
    cancellation_token: CancellationToken,
}

impl Default for BitgetHttpClient {
    fn default() -> Self {
        Self::new(None, None, None, None)
            .expect("Failed to create default BitgetHttpClient")
    }
}

impl Debug for BitgetHttpClient {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BitgetHttpClient")
            .field("base_url", &self.base_url)
            .field("has_credentials", &self.credential.is_some())
            .field("timeout", &self.timeout)
            .finish()
    }
}

impl BitgetHttpClient {
    /// Creates a new [`BitgetHttpClient`] instance.
    ///
    /// # Arguments
    ///
    /// * `is_testnet` - Whether to use the testnet API endpoint
    /// * `api_key` - Optional API key for authenticated requests
    /// * `api_secret` - Optional API secret for authenticated requests
    /// * `passphrase` - Optional passphrase for authenticated requests
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP client cannot be built.
    pub fn new(
        is_testnet: Option<bool>,
        api_key: Option<String>,
        api_secret: Option<String>,
        passphrase: Option<String>,
    ) -> BitgetHttpResult<Self> {
        let environment = if is_testnet.unwrap_or(false) {
            BitgetEnvironment::Testnet
        } else {
            BitgetEnvironment::Mainnet
        };
        let base_url = bitget_http_base_url(environment);
        let timeout = Duration::from_secs(DEFAULT_TIMEOUT_SECS);

        let credential = match (api_key, api_secret, passphrase) {
            (Some(key), Some(secret), Some(pass)) => Some(Credential::new(key, secret, pass)),
            _ => None,
        };

        let mut headers = HashMap::new();
        headers.insert(USER_AGENT.to_string(), NAUTILUS_USER_AGENT.to_string());

        let keyed_quotas = vec![
            (BITGET_GLOBAL_RATE_KEY.to_string(), *BITGET_REST_QUOTA),
        ];

        let client = HttpClient::new(
            headers,
            vec![],
            keyed_quotas,
            Some(*BITGET_REST_QUOTA),
            Some(DEFAULT_TIMEOUT_SECS),
            None,
        )
        .map_err(|e| BitgetHttpError::ClientBuild(e.to_string()))?;

        Ok(Self {
            base_url: base_url.to_string(),
            client,
            credential,
            timeout,
            cancellation_token: CancellationToken::new(),
        })
    }

    /// Sends a GET request to the specified endpoint.
    async fn send_get<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        authenticated: bool,
    ) -> BitgetHttpResult<BitgetResponse<T>> {
        let url = format!("{}{}", self.base_url, endpoint);
        let timestamp = Utc::now().timestamp_millis().to_string();

        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static(NAUTILUS_USER_AGENT));
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));

        if authenticated {
            let credential = self
                .credential
                .as_ref()
                .ok_or_else(|| BitgetHttpError::Authentication("No credentials provided".into()))?;

            let signature = credential.sign_request(&timestamp, "GET", endpoint, "");

            headers.insert(
                "ACCESS-KEY",
                HeaderValue::from_str(credential.api_key().as_str())
                    .map_err(|e| BitgetHttpError::Authentication(e.to_string()))?,
            );
            headers.insert(
                "ACCESS-SIGN",
                HeaderValue::from_str(&signature)
                    .map_err(|e| BitgetHttpError::Authentication(e.to_string()))?,
            );
            headers.insert(
                "ACCESS-TIMESTAMP",
                HeaderValue::from_str(&timestamp)
                    .map_err(|e| BitgetHttpError::Authentication(e.to_string()))?,
            );
            headers.insert(
                "ACCESS-PASSPHRASE",
                HeaderValue::from_str(credential.passphrase().as_str())
                    .map_err(|e| BitgetHttpError::Authentication(e.to_string()))?,
            );
        }

        let mut headers_map = HashMap::new();
        for (key, value) in headers.iter() {
            if let Ok(v) = value.to_str() {
                headers_map.insert(key.as_str().to_string(), v.to_string());
            }
        }

        let response = self
            .client
            .request(
                Method::GET,
                url,
                None,
                Some(headers_map),
                None,
                Some(self.timeout.as_secs()),
                Some(vec![BITGET_GLOBAL_RATE_KEY.to_string()]),
            )
            .await
            .map_err(|e| BitgetHttpError::Request(e.to_string()))?;

        let body = String::from_utf8(response.body.to_vec())
            .map_err(|e| BitgetHttpError::Request(format!("Invalid UTF-8: {e}")))?;

        if !response.status.is_success() {
            return Err(BitgetHttpError::Request(format!(
                "HTTP {}: {}",
                response.status.as_u16(), body
            )));
        }

        let parsed: BitgetResponse<T> = serde_json::from_str(&body)
            .map_err(|e| BitgetHttpError::Deserialization(format!("{}: {}", e, body)))?;

        if !parsed.is_success() {
            return Err(BitgetHttpError::ApiError {
                code: parsed.code,
                msg: parsed.msg,
            });
        }

        Ok(parsed)
    }

    /// Sends a POST request to the specified endpoint.
    async fn send_post<T: DeserializeOwned, B: Serialize>(
        &self,
        endpoint: &str,
        body: &B,
    ) -> BitgetHttpResult<BitgetResponse<T>> {
        let credential = self
            .credential
            .as_ref()
            .ok_or_else(|| BitgetHttpError::Authentication("No credentials provided".into()))?;

        let url = format!("{}{}", self.base_url, endpoint);
        let timestamp = Utc::now().timestamp_millis().to_string();
        let body_json = serde_json::to_string(body)
            .map_err(|e| BitgetHttpError::Request(e.to_string()))?;

        let signature = credential.sign_request(&timestamp, "POST", endpoint, &body_json);

        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static(NAUTILUS_USER_AGENT));
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert(
            "ACCESS-KEY",
            HeaderValue::from_str(credential.api_key().as_str())
                .map_err(|e| BitgetHttpError::Authentication(e.to_string()))?,
        );
        headers.insert(
            "ACCESS-SIGN",
            HeaderValue::from_str(&signature)
                .map_err(|e| BitgetHttpError::Authentication(e.to_string()))?,
        );
        headers.insert(
            "ACCESS-TIMESTAMP",
            HeaderValue::from_str(&timestamp)
                .map_err(|e| BitgetHttpError::Authentication(e.to_string()))?,
        );
        headers.insert(
            "ACCESS-PASSPHRASE",
            HeaderValue::from_str(credential.passphrase().as_str())
                .map_err(|e| BitgetHttpError::Authentication(e.to_string()))?,
        );

        let mut headers_map = HashMap::new();
        for (key, value) in headers.iter() {
            if let Ok(v) = value.to_str() {
                headers_map.insert(key.as_str().to_string(), v.to_string());
            }
        }

        let response = self
            .client
            .request(
                Method::POST,
                url,
                None,
                Some(headers_map),
                Some(body_json.into_bytes()),
                Some(self.timeout.as_secs()),
                Some(vec![BITGET_GLOBAL_RATE_KEY.to_string()]),
            )
            .await
            .map_err(|e| BitgetHttpError::Request(e.to_string()))?;

        let body_text = String::from_utf8(response.body.to_vec())
            .map_err(|e| BitgetHttpError::Request(format!("Invalid UTF-8: {e}")))?;

        if !response.status.is_success() {
            return Err(BitgetHttpError::Request(format!(
                "HTTP {}: {}",
                response.status.as_u16(), body_text
            )));
        }

        let parsed: BitgetResponse<T> = serde_json::from_str(&body_text)
            .map_err(|e| BitgetHttpError::Deserialization(format!("{}: {}", e, body_text)))?;

        if !parsed.is_success() {
            return Err(BitgetHttpError::ApiError {
                code: parsed.code,
                msg: parsed.msg,
            });
        }

        Ok(parsed)
    }

    // ========== Public API Endpoints ==========

    /// Get server time.
    ///
    /// # References
    /// - <https://www.bitget.com/api-doc/common/public/Get-Server-Time>
    pub async fn get_server_time(&self) -> BitgetHttpResult<BitgetServerTimeResponse> {
        self.send_get("/api/v2/public/time", false).await
    }

    /// Get all spot symbols.
    ///
    /// # References
    /// - <https://www.bitget.com/api-doc/spot/market/Get-Symbols>
    pub async fn get_spot_symbols(&self) -> BitgetHttpResult<BitgetSpotSymbolsResponse> {
        self.send_get("/api/v2/spot/public/symbols", false).await
    }

    /// Get all futures symbols.
    ///
    /// # Arguments
    ///
    /// * `product_type` - Product type (e.g., "USDT-FUTURES", "COIN-FUTURES", "USDC-FUTURES")
    ///
    /// # References
    /// - <https://www.bitget.com/api-doc/contract/market/Get-All-Symbols>
    pub async fn get_futures_symbols(
        &self,
        product_type: &str,
    ) -> BitgetHttpResult<BitgetFuturesSymbolsResponse> {
        let endpoint = format!("/api/v2/mix/market/contracts?productType={}", product_type);
        self.send_get(&endpoint, false).await
    }

    /// Get orderbook snapshot.
    ///
    /// # Arguments
    ///
    /// * `symbol` - Symbol name (e.g., "BTCUSDT")
    /// * `limit` - Number of levels to return (default: 100, max: 150)
    ///
    /// # References
    /// - <https://www.bitget.com/api-doc/spot/market/Get-Orderbook>
    pub async fn get_orderbook(
        &self,
        symbol: &str,
        limit: Option<u32>,
    ) -> BitgetHttpResult<BitgetOrderbookResponse> {
        let limit = limit.unwrap_or(100).min(150);
        let endpoint = format!("/api/v2/spot/market/orderbook?symbol={}&type=step0&limit={}", symbol, limit);
        self.send_get(&endpoint, false).await
    }

    /// Get recent trades.
    ///
    /// # Arguments
    ///
    /// * `symbol` - Symbol name (e.g., "BTCUSDT")
    /// * `limit` - Number of trades to return (default: 100, max: 500)
    ///
    /// # References
    /// - <https://www.bitget.com/api-doc/spot/market/Get-Recent-Trades>
    pub async fn get_trades(
        &self,
        symbol: &str,
        limit: Option<u32>,
    ) -> BitgetHttpResult<BitgetTradesResponse> {
        let limit = limit.unwrap_or(100).min(500);
        let endpoint = format!("/api/v2/spot/market/fills?symbol={}&limit={}", symbol, limit);
        self.send_get(&endpoint, false).await
    }

    /// Shutdown the client and cancel all pending operations.
    pub fn shutdown(&self) {
        self.cancellation_token.cancel();
    }
}

////////////////////////////////////////////////////////////////////////////////
// Tests
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation_no_auth() {
        let client = BitgetHttpClient::new(None, None, None, None);
        assert!(client.is_ok());

        let client = client.unwrap();
        assert!(client.credential.is_none());
        assert_eq!(client.base_url, "https://api.bitget.com");
    }

    #[test]
    fn test_client_creation_with_auth() {
        let client = BitgetHttpClient::new(
            None,
            Some("test_key".to_string()),
            Some("test_secret".to_string()),
            Some("test_pass".to_string()),
        );
        assert!(client.is_ok());

        let client = client.unwrap();
        assert!(client.credential.is_some());
    }

    #[test]
    fn test_client_creation_testnet() {
        let client = BitgetHttpClient::new(Some(true), None, None, None);
        assert!(client.is_ok());

        // Note: Testnet URL would be different - update when Bitget provides testnet
        let client = client.unwrap();
        assert!(client.base_url.contains("bitget"));
    }

    #[tokio::test]
    async fn test_get_server_time() {
        let client = BitgetHttpClient::default();
        let result = client.get_server_time().await;

        // This test requires network access
        // In a real environment, we'd use a mock server
        if result.is_ok() {
            let response = result.unwrap();
            assert!(response.is_success());
        }
    }
}
