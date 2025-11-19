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

//! HTTP client for Bitget REST API.

use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

use chrono::Utc;
use nautilus_network::http::{HttpClient, HttpResponse};
use reqwest::Method;
use serde::{Serialize, de::DeserializeOwned};
use tokio_util::sync::CancellationToken;

use super::error::BitgetHttpError;
use crate::common::{credential::Credential, models::BitgetApiResponse};

const BITGET_SUCCESS_CODE: &str = "00000";

/// Provides a raw HTTP client for interacting with the Bitget REST API.
#[derive(Clone)]
pub struct BitgetHttpClient {
    base_url: String,
    client: HttpClient,
    credential: Option<Credential>,
    retry_manager: RetryManager<BitgetHttpError>,
    cancellation_token: CancellationToken,
}

impl Debug for BitgetHttpClient {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let credential = self.credential.as_ref().map(|_| "<redacted>");
        f.debug_struct(stringify!(BitgetHttpClient))
            .field("base_url", &self.base_url)
            .field("credential", &credential)
            .finish_non_exhaustive()
    }
}

impl BitgetHttpClient {
    /// Creates a new [`BitgetHttpClient`] instance.
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP client cannot be created.
    pub fn new(
        base_url: String,
        api_key: Option<String>,
        api_secret: Option<String>,
        api_passphrase: Option<String>,
        timeout_secs: Option<u64>,
    ) -> anyhow::Result<Self> {
        let credential = match (api_key, api_secret, api_passphrase) {
            (Some(key), Some(secret), Some(passphrase)) => {
                Some(Credential::new(key, secret, passphrase))
            }
            _ => None,
        };

        let client = HttpClient::new(timeout_secs, None, None, None)?;
        let retry_manager = RetryManager::new(None, None, None);
        let cancellation_token = CancellationToken::new();

        Ok(Self {
            base_url,
            client,
            credential,
            retry_manager,
            cancellation_token,
        })
    }

    /// Sends a GET request to the specified endpoint.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn get<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        authenticated: bool,
    ) -> Result<T, BitgetHttpError> {
        self.send_request(Method::GET, endpoint, None::<&()>, authenticated)
            .await
    }

    /// Sends a POST request to the specified endpoint.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn post<T: DeserializeOwned, B: Serialize>(
        &self,
        endpoint: &str,
        body: Option<&B>,
        authenticated: bool,
    ) -> Result<T, BitgetHttpError> {
        self.send_request(Method::POST, endpoint, body, authenticated)
            .await
    }

    async fn send_request<T: DeserializeOwned, B: Serialize>(
        &self,
        method: Method,
        endpoint: &str,
        body: Option<&B>,
        authenticated: bool,
    ) -> Result<T, BitgetHttpError> {
        let url = format!("{}{}", self.base_url, endpoint);
        let timestamp = Utc::now().timestamp_millis().to_string();

        let body_bytes = if let Some(b) = body {
            serde_json::to_vec(b).map_err(|e| {
                BitgetHttpError::InvalidParameters(format!("Failed to serialize body: {e}"))
            })?
        } else {
            Vec::new()
        };

        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("nautilus-bitget/0.52.0"));
        headers.insert(
            HeaderName::from_static("content-type"),
            HeaderValue::from_static("application/json"),
        );

        if authenticated {
            let credential = self.credential.as_ref().ok_or_else(|| {
                BitgetHttpError::AuthenticationError("No credentials provided".to_string())
            })?;

            let signature = credential.sign(
                &timestamp,
                method.as_str(),
                endpoint,
                if body_bytes.is_empty() { None } else { Some(&body_bytes) },
            );

            headers.insert(
                HeaderName::from_static("access-key"),
                HeaderValue::from_str(&credential.api_key.to_string())
                    .map_err(|e| BitgetHttpError::AuthenticationError(e.to_string()))?,
            );
            headers.insert(
                HeaderName::from_static("access-sign"),
                HeaderValue::from_str(&signature)
                    .map_err(|e| BitgetHttpError::AuthenticationError(e.to_string()))?,
            );
            headers.insert(
                HeaderName::from_static("access-timestamp"),
                HeaderValue::from_str(&timestamp)
                    .map_err(|e| BitgetHttpError::AuthenticationError(e.to_string()))?,
            );
            headers.insert(
                HeaderName::from_static("access-passphrase"),
                HeaderValue::from_str(&credential.api_passphrase)
                    .map_err(|e| BitgetHttpError::AuthenticationError(e.to_string()))?,
            );
        }

        let response = self
            .client
            .send_request(
                method.clone(),
                url.clone(),
                headers,
                if body_bytes.is_empty() { None } else { Some(body_bytes) },
            )
            .await
            .map_err(|e| BitgetHttpError::RequestFailed(e.to_string()))?;

        if !response.status.is_success() {
            return Err(BitgetHttpError::RequestFailed(format!(
                "HTTP {}: {}",
                response.status,
                String::from_utf8_lossy(&response.body)
            )));
        }

        let api_response: BitgetApiResponse<T> = serde_json::from_slice(&response.body)
            .map_err(|e| BitgetHttpError::ParseError(format!("Failed to parse response: {e}")))?;

        if api_response.code != BITGET_SUCCESS_CODE {
            return Err(BitgetHttpError::ApiError {
                code: api_response.code,
                msg: api_response.msg,
            });
        }

        Ok(api_response.data)
    }

    /// Fetches spot instruments from Bitget.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails.
    pub async fn get_spot_instruments(&self) -> Result<Vec<crate::common::models::BitgetInstrument>, BitgetHttpError> {
        self.get("/api/v2/spot/public/symbols", false).await
    }

    /// Fetches futures contracts from Bitget.
    ///
    /// # Parameters
    ///
    /// * `product_type` - The product type (e.g., "USDT-FUTURES", "COIN-FUTURES")
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails.
    pub async fn get_futures_contracts(
        &self,
        product_type: &str,
    ) -> Result<Vec<crate::common::models::BitgetFuturesContract>, BitgetHttpError> {
        let endpoint = format!("/api/v2/mix/market/contracts?productType={}", product_type);
        self.get(&endpoint, false).await
    }

    /// Fetches futures positions.
    ///
    /// # Parameters
    ///
    /// * `product_type` - The product type (e.g., "USDT-FUTURES")
    /// * `symbol` - Optional specific symbol filter
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails.
    pub async fn get_futures_positions(
        &self,
        product_type: &str,
        symbol: Option<&str>,
    ) -> Result<Vec<crate::common::models::BitgetPosition>, BitgetHttpError> {
        let mut endpoint = format!("/api/v2/mix/position/all-position?productType={}", product_type);
        if let Some(sym) = symbol {
            endpoint.push_str(&format!("&symbol={}", sym));
        }
        self.get(&endpoint, true).await
    }

    /// Fetches futures account information.
    ///
    /// # Parameters
    ///
    /// * `product_type` - The product type (e.g., "USDT-FUTURES")
    /// * `symbol` - Symbol for the account
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails.
    pub async fn get_futures_account(
        &self,
        product_type: &str,
        symbol: &str,
    ) -> Result<crate::common::models::BitgetFuturesAccount, BitgetHttpError> {
        let endpoint = format!(
            "/api/v2/mix/account/account?productType={}&symbol={}",
            product_type, symbol
        );
        self.get(&endpoint, true).await
    }

    /// Fetches current funding rate.
    ///
    /// # Parameters
    ///
    /// * `product_type` - The product type (e.g., "USDT-FUTURES")
    /// * `symbol` - Symbol for funding rate
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails.
    pub async fn get_funding_rate(
        &self,
        product_type: &str,
        symbol: &str,
    ) -> Result<crate::common::models::BitgetFundingRate, BitgetHttpError> {
        let endpoint = format!(
            "/api/v2/mix/market/current-fund-rate?productType={}&symbol={}",
            product_type, symbol
        );
        self.get(&endpoint, false).await
    }

    /// Sets leverage for a futures contract.
    ///
    /// # Parameters
    ///
    /// * `product_type` - The product type
    /// * `symbol` - The symbol
    /// * `leverage` - The leverage value
    /// * `margin_coin` - The margin coin
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails.
    pub async fn set_leverage(
        &self,
        product_type: &str,
        symbol: &str,
        leverage: &str,
        margin_coin: &str,
    ) -> Result<serde_json::Value, BitgetHttpError> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct SetLeverageRequest<'a> {
            product_type: &'a str,
            symbol: &'a str,
            margin_coin: &'a str,
            leverage: &'a str,
        }

        let body = SetLeverageRequest {
            product_type,
            symbol,
            margin_coin,
            leverage,
        };

        self.post("/api/v2/mix/account/set-leverage", Some(&body), true)
            .await
    }

    /// Sets margin mode for a futures contract.
    ///
    /// # Parameters
    ///
    /// * `product_type` - The product type
    /// * `symbol` - The symbol
    /// * `margin_mode` - The margin mode ("crossed" or "isolated")
    /// * `margin_coin` - The margin coin
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails.
    pub async fn set_margin_mode(
        &self,
        product_type: &str,
        symbol: &str,
        margin_mode: &str,
        margin_coin: &str,
    ) -> Result<serde_json::Value, BitgetHttpError> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct SetMarginModeRequest<'a> {
            product_type: &'a str,
            symbol: &'a str,
            margin_coin: &'a str,
            margin_mode: &'a str,
        }

        let body = SetMarginModeRequest {
            product_type,
            symbol,
            margin_coin,
            margin_mode,
        };

        self.post("/api/v2/mix/account/set-margin-mode", Some(&body), true)
            .await
    }
}
