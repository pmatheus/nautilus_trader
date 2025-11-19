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

//! Provides an ergonomic wrapper around the **Kucoin REST API**.
//!
//! The core type exported by this module is [`KucoinHttpClient`]. It offers an
//! interface to all exchange endpoints currently required by NautilusTrader.
//!
//! **CRITICAL FEATURE**: WebSocket Token Acquisition
//!
//! Kucoin uses a unique token-based WebSocket connection system where you must
//! first request a connection token via REST API before establishing WebSocket
//! connections. This client provides methods to acquire both public and private
//! WebSocket tokens.

use std::{
    fmt::{Debug, Formatter},
    sync::Arc,
    time::Duration,
};

use chrono::Utc;
use nautilus_core::consts::NAUTILUS_USER_AGENT;
use nautilus_network::http::HttpClient;
use reqwest::{Method, header::USER_AGENT};
use serde::{Deserialize, de::DeserializeOwned};
use tokio_util::sync::CancellationToken;

use super::{error::KucoinHttpError, models::{KucoinResponse, KucoinServerTime}};
use crate::common::{
    consts::KUCOIN_SUCCESS_CODE,
    credential::Credential,
    enums::KucoinProductType,
    models::{KucoinFuturesContract, KucoinInstrument, KucoinWsTokenResponse},
};

/// Provides a raw HTTP client for interacting with the [Kucoin](https://kucoin.com) REST API.
///
/// This client wraps the underlying [`HttpClient`] to handle functionality
/// specific to Kucoin, such as request signing (for authenticated endpoints),
/// forming request URLs, and deserializing responses into Kucoin specific data models.
pub struct KucoinRawHttpClient {
    base_url: String,
    client: HttpClient,
    credential: Option<Arc<Credential>>,
    product_type: KucoinProductType,
}

impl Debug for KucoinRawHttpClient {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KucoinRawHttpClient")
            .field("base_url", &self.base_url)
            .field("product_type", &self.product_type)
            .field(
                "credential",
                &self.credential.as_ref().map(|_| "<redacted>"),
            )
            .finish()
    }
}

impl KucoinRawHttpClient {
    /// Creates a new [`KucoinRawHttpClient`] instance.
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP client cannot be created.
    pub fn new(
        base_url: Option<String>,
        product_type: KucoinProductType,
        credential: Option<Arc<Credential>>,
        timeout_secs: Option<u64>,
        cancel_token: CancellationToken,
    ) -> anyhow::Result<Self> {
        let timeout = timeout_secs.map(Duration::from_secs);

        let client = HttpClient::new(
            USER_AGENT,
            NAUTILUS_USER_AGENT.to_string(),
            timeout,
            None, // No rate limiter at this level
            cancel_token,
        )?;

        Ok(Self {
            base_url: base_url.unwrap_or_else(|| {
                // Default base URL will be set later based on product_type and is_sandbox
                String::new()
            }),
            client,
            credential,
            product_type,
        })
    }

    /// Makes an authenticated request to the Kucoin API.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - No credentials are configured
    /// - The request fails
    /// - The response cannot be parsed
    async fn send_authenticated<T: DeserializeOwned>(
        &self,
        method: Method,
        endpoint: &str,
        body: Option<String>,
    ) -> Result<T, KucoinHttpError> {
        let credential = self
            .credential
            .as_ref()
            .ok_or_else(|| KucoinHttpError::AuthError("No credentials configured".to_string()))?;

        // Get current timestamp in milliseconds
        let timestamp = Utc::now().timestamp_millis().to_string();

        // Prepare body for signing
        let body_str = body.as_deref().unwrap_or("");

        // Sign the request
        let signature = credential.sign(&timestamp, method.as_str(), endpoint, body_str);
        let passphrase = credential.sign_passphrase();

        // Build URL
        let url = format!("{}{}", self.base_url, endpoint);

        // Make request
        let mut request = self.client.request(method.clone(), &url);

        // Add authentication headers
        request = request
            .header("KC-API-KEY", credential.api_key.as_str())
            .header("KC-API-SIGN", signature)
            .header("KC-API-TIMESTAMP", timestamp)
            .header("KC-API-PASSPHRASE", passphrase)
            .header("KC-API-KEY-VERSION", "2"); // Version 2 uses encrypted passphrase

        // Add body if present
        if let Some(body_content) = body {
            request = request
                .header("Content-Type", "application/json")
                .body(body_content);
        }

        // Send request
        let response = request
            .send()
            .await
            .map_err(|e| KucoinHttpError::RequestFailed(e.to_string()))?;

        // Parse response
        let text = response
            .text()
            .await
            .map_err(|e| KucoinHttpError::ParseError(e.to_string()))?;

        let api_response: KucoinResponse<T> = serde_json::from_str(&text)
            .map_err(|e| KucoinHttpError::ParseError(format!("{e}: {text}")))?;

        // Check for API errors
        if api_response.code != KUCOIN_SUCCESS_CODE {
            return Err(KucoinHttpError::ApiError {
                code: api_response.code,
                msg: "API error".to_string(),
            });
        }

        Ok(api_response.data)
    }

    /// Makes an unauthenticated request to the Kucoin API.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    async fn send_public<T: DeserializeOwned>(
        &self,
        method: Method,
        endpoint: &str,
    ) -> Result<T, KucoinHttpError> {
        let url = format!("{}{}", self.base_url, endpoint);

        let response = self
            .client
            .request(method, &url)
            .send()
            .await
            .map_err(|e| KucoinHttpError::RequestFailed(e.to_string()))?;

        let text = response
            .text()
            .await
            .map_err(|e| KucoinHttpError::ParseError(e.to_string()))?;

        let api_response: KucoinResponse<T> = serde_json::from_str(&text)
            .map_err(|e| KucoinHttpError::ParseError(format!("{e}: {text}")))?;

        if api_response.code != KUCOIN_SUCCESS_CODE {
            return Err(KucoinHttpError::ApiError {
                code: api_response.code,
                msg: "API error".to_string(),
            });
        }

        Ok(api_response.data)
    }

    /// **CRITICAL**: Requests a public WebSocket token from Kucoin.
    ///
    /// This is the first step in establishing a WebSocket connection.
    /// The response contains:
    /// - A token valid for 24 hours
    /// - WebSocket server endpoints
    /// - Ping/pong configuration
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    ///
    /// # Kucoin Documentation
    ///
    /// <https://docs.kucoin.com/#apply-connect-token>
    pub async fn get_public_ws_token(&self) -> Result<KucoinWsTokenResponse, KucoinHttpError> {
        self.send_public(Method::POST, "/api/v1/bullet-public")
            .await
    }

    /// **CRITICAL**: Requests a private WebSocket token from Kucoin.
    ///
    /// This is required for private WebSocket channels (account updates, orders, etc.).
    /// Requires authentication. The response contains:
    /// - A token valid for 24 hours
    /// - WebSocket server endpoints
    /// - Ping/pong configuration
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - No credentials are configured
    /// - The request fails
    /// - The response cannot be parsed
    ///
    /// # Kucoin Documentation
    ///
    /// <https://docs.kucoin.com/#apply-connect-token>
    pub async fn get_private_ws_token(&self) -> Result<KucoinWsTokenResponse, KucoinHttpError> {
        self.send_authenticated(Method::POST, "/api/v1/bullet-private", None)
            .await
    }

    /// Gets the server time.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn get_server_time(&self) -> Result<KucoinServerTime, KucoinHttpError> {
        self.send_public(Method::GET, "/api/v1/timestamp").await
    }

    /// Gets all SPOT instruments (symbols).
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn get_instruments(&self) -> Result<Vec<KucoinInstrument>, KucoinHttpError> {
        self.send_public(Method::GET, "/api/v1/symbols").await
    }

    /// Gets all FUTURES contracts (instruments).
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    ///
    /// # Kucoin Futures Documentation
    ///
    /// <https://docs.kucoin.com/futures/#get-open-contract-list>
    pub async fn get_futures_contracts(
        &self,
    ) -> Result<Vec<KucoinFuturesContract>, KucoinHttpError> {
        self.send_public(Method::GET, "/api/v1/contracts/active")
            .await
    }
}

/// High-level Kucoin HTTP client.
///
/// Provides methods for all Kucoin REST API endpoints needed by NautilusTrader.
pub struct KucoinHttpClient {
    raw_client: KucoinRawHttpClient,
}

impl Debug for KucoinHttpClient {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KucoinHttpClient")
            .field("raw_client", &self.raw_client)
            .finish()
    }
}

impl KucoinHttpClient {
    /// Creates a new [`KucoinHttpClient`] instance.
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying HTTP client cannot be created.
    pub fn new(
        base_url: Option<String>,
        product_type: KucoinProductType,
        credential: Option<Arc<Credential>>,
        timeout_secs: Option<u64>,
        cancel_token: CancellationToken,
    ) -> anyhow::Result<Self> {
        let raw_client = KucoinRawHttpClient::new(
            base_url,
            product_type,
            credential,
            timeout_secs,
            cancel_token,
        )?;
        Ok(Self { raw_client })
    }

    /// **CRITICAL**: Gets a public WebSocket token for establishing connections.
    ///
    /// # Errors
    ///
    /// Returns an error if the token request fails.
    pub async fn get_public_ws_token(&self) -> Result<KucoinWsTokenResponse, KucoinHttpError> {
        self.raw_client.get_public_ws_token().await
    }

    /// **CRITICAL**: Gets a private WebSocket token for authenticated connections.
    ///
    /// # Errors
    ///
    /// Returns an error if the token request fails or no credentials are configured.
    pub async fn get_private_ws_token(&self) -> Result<KucoinWsTokenResponse, KucoinHttpError> {
        self.raw_client.get_private_ws_token().await
    }

    /// Gets the server time.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails.
    pub async fn get_server_time(&self) -> Result<KucoinServerTime, KucoinHttpError> {
        self.raw_client.get_server_time().await
    }

    /// Gets all SPOT instruments.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails.
    pub async fn get_instruments(&self) -> Result<Vec<KucoinInstrument>, KucoinHttpError> {
        self.raw_client.get_instruments().await
    }

    /// Gets all FUTURES contracts.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails.
    pub async fn get_futures_contracts(
        &self,
    ) -> Result<Vec<KucoinFuturesContract>, KucoinHttpError> {
        self.raw_client.get_futures_contracts().await
    }
}
