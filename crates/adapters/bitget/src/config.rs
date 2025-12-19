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

//! Configuration structures for the Bitget adapter.

use std::collections::HashMap;

use nautilus_model::identifiers::AccountId;

use crate::common::{
    enums::{BitgetEnvironment, BitgetMarginMode, BitgetPositionMode, BitgetProductType},
    urls::{bitget_http_base_url, bitget_ws_private_url, bitget_ws_public_url},
};

/// Configuration for the Bitget live data client.
#[derive(Clone, Debug)]
pub struct BitgetDataClientConfig {
    /// Optional API key for authenticated REST/WebSocket requests.
    pub api_key: Option<String>,
    /// Optional API secret for authenticated REST/WebSocket requests.
    pub api_secret: Option<String>,
    /// Optional API passphrase required for Bitget authentication.
    pub api_passphrase: Option<String>,
    /// Product types to subscribe to (e.g., Spot, UsdtFutures, CoinFutures).
    pub product_types: Vec<BitgetProductType>,
    /// Environment selection (Mainnet, Testnet).
    pub environment: BitgetEnvironment,
    /// Optional override for the REST base URL.
    pub base_url_http: Option<String>,
    /// Optional override for the public WebSocket URL.
    pub base_url_ws_public: Option<String>,
    /// Optional override for the private WebSocket URL.
    pub base_url_ws_private: Option<String>,
    /// Optional HTTP proxy URL.
    pub http_proxy_url: Option<String>,
    /// Optional WebSocket proxy URL.
    ///
    /// Note: WebSocket proxy support is not yet implemented. This field is reserved
    /// for future functionality. Use `http_proxy_url` for REST API proxy support.
    pub ws_proxy_url: Option<String>,
    /// Optional REST timeout in seconds.
    pub http_timeout_secs: Option<u64>,
    /// Optional maximum retry attempts for REST requests.
    pub max_retries: Option<u32>,
    /// Optional initial retry backoff in milliseconds.
    pub retry_delay_initial_ms: Option<u64>,
    /// Optional maximum retry backoff in milliseconds.
    pub retry_delay_max_ms: Option<u64>,
    /// Optional heartbeat interval (seconds) for WebSocket clients.
    pub heartbeat_interval_secs: Option<u64>,
    /// Optional receive window in milliseconds for signed requests.
    pub recv_window_ms: Option<u64>,
    /// Optional interval (minutes) for instrument refresh from REST.
    pub update_instruments_interval_mins: Option<u64>,
}

impl Default for BitgetDataClientConfig {
    fn default() -> Self {
        Self {
            api_key: None,
            api_secret: None,
            api_passphrase: None,
            product_types: vec![BitgetProductType::Spot],
            environment: BitgetEnvironment::Mainnet,
            base_url_http: None,
            base_url_ws_public: None,
            base_url_ws_private: None,
            http_proxy_url: None,
            ws_proxy_url: None,
            http_timeout_secs: Some(60),
            max_retries: Some(3),
            retry_delay_initial_ms: Some(1_000),
            retry_delay_max_ms: Some(10_000),
            heartbeat_interval_secs: Some(20),
            recv_window_ms: Some(5_000),
            update_instruments_interval_mins: Some(60),
        }
    }
}

impl BitgetDataClientConfig {
    /// Creates a configuration with default values.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns `true` if all API credentials are available (key, secret, and passphrase).
    #[must_use]
    pub fn has_api_credentials(&self) -> bool {
        self.api_key.is_some() && self.api_secret.is_some() && self.api_passphrase.is_some()
    }

    /// Returns the REST base URL, considering overrides and environment.
    #[must_use]
    pub fn http_base_url(&self) -> String {
        self.base_url_http
            .clone()
            .unwrap_or_else(|| bitget_http_base_url(self.environment).to_string())
    }

    /// Returns the public WebSocket URL, considering overrides and environment.
    #[must_use]
    pub fn ws_public_url(&self) -> String {
        self.base_url_ws_public
            .clone()
            .unwrap_or_else(|| bitget_ws_public_url(self.environment).to_string())
    }

    /// Returns the private WebSocket URL, considering overrides and environment.
    #[must_use]
    pub fn ws_private_url(&self) -> String {
        self.base_url_ws_private
            .clone()
            .unwrap_or_else(|| bitget_ws_private_url(self.environment).to_string())
    }

    /// Returns `true` when private WebSocket connection is required.
    #[must_use]
    pub fn requires_private_ws(&self) -> bool {
        self.has_api_credentials()
    }
}

/// Configuration for the Bitget live execution client.
#[derive(Clone, Debug)]
pub struct BitgetExecClientConfig {
    /// API key for authenticated requests.
    pub api_key: Option<String>,
    /// API secret for authenticated requests.
    pub api_secret: Option<String>,
    /// API passphrase required for Bitget authentication.
    pub api_passphrase: Option<String>,
    /// Product types to support (e.g., Spot, UsdtFutures, CoinFutures).
    pub product_types: Vec<BitgetProductType>,
    /// Environment selection (Mainnet, Testnet).
    pub environment: BitgetEnvironment,
    /// Optional override for the REST base URL.
    pub base_url_http: Option<String>,
    /// Optional override for the private WebSocket URL.
    pub base_url_ws_private: Option<String>,
    /// Optional HTTP proxy URL.
    pub http_proxy_url: Option<String>,
    /// Optional WebSocket proxy URL.
    ///
    /// Note: WebSocket proxy support is not yet implemented. This field is reserved
    /// for future functionality. Use `http_proxy_url` for REST API proxy support.
    pub ws_proxy_url: Option<String>,
    /// Optional REST timeout in seconds.
    pub http_timeout_secs: Option<u64>,
    /// Optional maximum retry attempts for REST requests.
    pub max_retries: Option<u32>,
    /// Optional initial retry backoff in milliseconds.
    pub retry_delay_initial_ms: Option<u64>,
    /// Optional maximum retry backoff in milliseconds.
    pub retry_delay_max_ms: Option<u64>,
    /// Optional heartbeat interval (seconds) for WebSocket clients.
    pub heartbeat_interval_secs: Option<u64>,
    /// Optional receive window in milliseconds for signed requests.
    pub recv_window_ms: Option<u64>,
    /// Optional account identifier to associate with the execution client.
    pub account_id: Option<AccountId>,
    /// Whether to generate position reports from wallet balances for SPOT positions.
    pub use_spot_position_reports: bool,
    /// Leverage configuration for futures (symbol -> leverage).
    pub futures_leverages: Option<HashMap<String, u32>>,
    /// Position mode configuration for symbols (symbol -> mode).
    pub position_mode: Option<HashMap<String, BitgetPositionMode>>,
    /// Margin mode setting.
    pub margin_mode: Option<BitgetMarginMode>,
}

impl Default for BitgetExecClientConfig {
    fn default() -> Self {
        Self {
            api_key: None,
            api_secret: None,
            api_passphrase: None,
            product_types: vec![BitgetProductType::Spot],
            environment: BitgetEnvironment::Mainnet,
            base_url_http: None,
            base_url_ws_private: None,
            http_proxy_url: None,
            ws_proxy_url: None,
            http_timeout_secs: Some(60),
            max_retries: Some(3),
            retry_delay_initial_ms: Some(1_000),
            retry_delay_max_ms: Some(10_000),
            heartbeat_interval_secs: Some(5),
            recv_window_ms: Some(5_000),
            account_id: None,
            use_spot_position_reports: false,
            futures_leverages: None,
            position_mode: None,
            margin_mode: None,
        }
    }
}

impl BitgetExecClientConfig {
    /// Creates a configuration with default values.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns `true` if all API credentials are available (key, secret, and passphrase).
    #[must_use]
    pub fn has_api_credentials(&self) -> bool {
        self.api_key.is_some() && self.api_secret.is_some() && self.api_passphrase.is_some()
    }

    /// Returns the REST base URL, considering overrides and environment.
    #[must_use]
    pub fn http_base_url(&self) -> String {
        self.base_url_http
            .clone()
            .unwrap_or_else(|| bitget_http_base_url(self.environment).to_string())
    }

    /// Returns the private WebSocket URL, considering overrides and environment.
    #[must_use]
    pub fn ws_private_url(&self) -> String {
        self.base_url_ws_private
            .clone()
            .unwrap_or_else(|| bitget_ws_private_url(self.environment).to_string())
    }
}

////////////////////////////////////////////////////////////////////////////////
// Tests
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_data_config_default() {
        let config = BitgetDataClientConfig::default();

        assert!(!config.has_api_credentials());
        assert_eq!(config.product_types, vec![BitgetProductType::Spot]);
        assert_eq!(config.http_timeout_secs, Some(60));
        assert_eq!(config.heartbeat_interval_secs, Some(20));
    }

    #[rstest]
    fn test_data_config_with_credentials() {
        let config = BitgetDataClientConfig {
            api_key: Some("test_key".to_string()),
            api_secret: Some("test_secret".to_string()),
            api_passphrase: Some("test_passphrase".to_string()),
            ..Default::default()
        };

        assert!(config.has_api_credentials());
        assert!(config.requires_private_ws());
    }

    #[rstest]
    fn test_data_config_http_url_mainnet() {
        let config = BitgetDataClientConfig {
            environment: BitgetEnvironment::Mainnet,
            ..Default::default()
        };

        assert_eq!(config.http_base_url(), "https://api.bitget.com");
    }

    #[rstest]
    fn test_data_config_http_url_testnet() {
        let config = BitgetDataClientConfig {
            environment: BitgetEnvironment::Testnet,
            ..Default::default()
        };

        assert_eq!(config.http_base_url(), "https://api.bitget.com");
    }

    #[rstest]
    fn test_data_config_http_url_override() {
        let custom_url = "https://custom.bitget.com";
        let config = BitgetDataClientConfig {
            base_url_http: Some(custom_url.to_string()),
            ..Default::default()
        };

        assert_eq!(config.http_base_url(), custom_url);
    }

    #[rstest]
    fn test_data_config_ws_public_url() {
        let config = BitgetDataClientConfig {
            environment: BitgetEnvironment::Mainnet,
            ..Default::default()
        };

        assert_eq!(
            config.ws_public_url(),
            "wss://ws.bitget.com/v2/ws/public"
        );
    }

    #[rstest]
    fn test_data_config_ws_private_url() {
        let config = BitgetDataClientConfig {
            environment: BitgetEnvironment::Mainnet,
            ..Default::default()
        };

        assert_eq!(
            config.ws_private_url(),
            "wss://ws.bitget.com/v2/ws/private"
        );
    }

    #[rstest]
    fn test_exec_config_default() {
        let config = BitgetExecClientConfig::default();

        assert!(!config.has_api_credentials());
        assert_eq!(config.product_types, vec![BitgetProductType::Spot]);
        assert_eq!(config.http_timeout_secs, Some(60));
        assert_eq!(config.heartbeat_interval_secs, Some(5));
    }

    #[rstest]
    fn test_exec_config_with_credentials() {
        let config = BitgetExecClientConfig {
            api_key: Some("test_key".to_string()),
            api_secret: Some("test_secret".to_string()),
            api_passphrase: Some("test_passphrase".to_string()),
            ..Default::default()
        };

        assert!(config.has_api_credentials());
    }

    #[rstest]
    fn test_exec_config_urls() {
        let config = BitgetExecClientConfig {
            environment: BitgetEnvironment::Mainnet,
            ..Default::default()
        };

        assert_eq!(config.http_base_url(), "https://api.bitget.com");
        assert_eq!(config.ws_private_url(), "wss://ws.bitget.com/v2/ws/private");
    }

    #[rstest]
    fn test_exec_config_custom_urls() {
        let config = BitgetExecClientConfig {
            base_url_http: Some("https://custom-http.bitget.com".to_string()),
            base_url_ws_private: Some("wss://custom-private.bitget.com".to_string()),
            ..Default::default()
        };

        assert_eq!(config.http_base_url(), "https://custom-http.bitget.com");
        assert_eq!(config.ws_private_url(), "wss://custom-private.bitget.com");
    }
}
