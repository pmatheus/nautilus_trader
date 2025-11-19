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

use crate::common::{enums::BitgetInstrumentType, urls::{get_http_base_url, get_ws_private_url, get_ws_public_url}};

/// Configuration for the Bitget data client.
#[derive(Clone, Debug)]
pub struct BitgetDataClientConfig {
    /// Optional API key for authenticated endpoints.
    pub api_key: Option<String>,
    /// Optional API secret for authenticated endpoints.
    pub api_secret: Option<String>,
    /// Optional API passphrase for authenticated endpoints.
    pub api_passphrase: Option<String>,
    /// Instrument types to load and subscribe to.
    pub instrument_types: Vec<BitgetInstrumentType>,
    /// Optional override for the HTTP base URL.
    pub base_url_http: Option<String>,
    /// Optional override for the public WebSocket URL.
    pub base_url_ws_public: Option<String>,
    /// Optional override for the private WebSocket URL.
    pub base_url_ws_private: Option<String>,
    /// Optional HTTP proxy URL.
    pub http_proxy_url: Option<String>,
    /// Optional WebSocket proxy URL.
    pub ws_proxy_url: Option<String>,
    /// When true the client will use Bitget demo/testnet endpoints.
    pub is_demo: bool,
    /// Optional HTTP timeout in seconds.
    pub http_timeout_secs: Option<u64>,
    /// Optional maximum retry attempts for requests.
    pub max_retries: Option<u32>,
    /// Optional initial retry delay in milliseconds.
    pub retry_delay_initial_ms: Option<u64>,
    /// Optional maximum retry delay in milliseconds.
    pub retry_delay_max_ms: Option<u64>,
    /// Optional interval for refreshing instruments (in minutes).
    pub update_instruments_interval_mins: Option<u64>,
}

impl Default for BitgetDataClientConfig {
    fn default() -> Self {
        Self {
            api_key: None,
            api_secret: None,
            api_passphrase: None,
            instrument_types: vec![BitgetInstrumentType::Spot],
            base_url_http: None,
            base_url_ws_public: None,
            base_url_ws_private: None,
            http_proxy_url: None,
            ws_proxy_url: None,
            is_demo: false,
            http_timeout_secs: Some(60),
            max_retries: Some(3),
            retry_delay_initial_ms: Some(1_000),
            retry_delay_max_ms: Some(10_000),
            update_instruments_interval_mins: Some(60),
        }
    }
}

impl BitgetDataClientConfig {
    /// Creates a new configuration with default settings.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns `true` when all API credential fields are populated.
    #[must_use]
    pub fn has_api_credentials(&self) -> bool {
        self.api_key.is_some() && self.api_secret.is_some() && self.api_passphrase.is_some()
    }

    /// Returns the HTTP base URL, falling back to the default when unset.
    #[must_use]
    pub fn http_base_url(&self) -> String {
        self.base_url_http
            .clone()
            .unwrap_or_else(|| get_http_base_url(self.is_demo))
    }

    /// Returns the public WebSocket URL, respecting the demo flag and overrides.
    #[must_use]
    pub fn ws_public_url(&self) -> String {
        self.base_url_ws_public
            .clone()
            .unwrap_or_else(|| get_ws_public_url(self.is_demo))
    }

    /// Returns the private WebSocket URL, respecting the demo flag and overrides.
    #[must_use]
    pub fn ws_private_url(&self) -> String {
        self.base_url_ws_private
            .clone()
            .unwrap_or_else(|| get_ws_private_url(self.is_demo))
    }
}

/// Configuration for the Bitget execution client.
#[derive(Clone, Debug)]
pub struct BitgetExecClientConfig {
    /// API key for authenticated endpoints.
    pub api_key: String,
    /// API secret for authenticated endpoints.
    pub api_secret: String,
    /// API passphrase for authenticated endpoints.
    pub api_passphrase: String,
    /// Instrument types the execution client should support.
    pub instrument_types: Vec<BitgetInstrumentType>,
    /// Optional override for the HTTP base URL.
    pub base_url_http: Option<String>,
    /// Optional override for the private WebSocket URL.
    pub base_url_ws_private: Option<String>,
    /// Optional HTTP proxy URL.
    pub http_proxy_url: Option<String>,
    /// Optional WebSocket proxy URL.
    pub ws_proxy_url: Option<String>,
    /// When true the client will use Bitget demo/testnet endpoints.
    pub is_demo: bool,
    /// Optional HTTP timeout in seconds.
    pub http_timeout_secs: Option<u64>,
    /// Optional maximum retry attempts for requests.
    pub max_retries: Option<u32>,
    /// Optional initial retry delay in milliseconds.
    pub retry_delay_initial_ms: Option<u64>,
    /// Optional maximum retry delay in milliseconds.
    pub retry_delay_max_ms: Option<u64>,
}

impl BitgetExecClientConfig {
    /// Creates a new execution client configuration.
    #[must_use]
    pub fn new(api_key: String, api_secret: String, api_passphrase: String) -> Self {
        Self {
            api_key,
            api_secret,
            api_passphrase,
            instrument_types: vec![BitgetInstrumentType::Spot],
            base_url_http: None,
            base_url_ws_private: None,
            http_proxy_url: None,
            ws_proxy_url: None,
            is_demo: false,
            http_timeout_secs: Some(60),
            max_retries: Some(3),
            retry_delay_initial_ms: Some(1_000),
            retry_delay_max_ms: Some(10_000),
        }
    }

    /// Returns the HTTP base URL, falling back to the default when unset.
    #[must_use]
    pub fn http_base_url(&self) -> String {
        self.base_url_http
            .clone()
            .unwrap_or_else(|| get_http_base_url(self.is_demo))
    }

    /// Returns the private WebSocket URL, respecting the demo flag and overrides.
    #[must_use]
    pub fn ws_private_url(&self) -> String {
        self.base_url_ws_private
            .clone()
            .unwrap_or_else(|| get_ws_private_url(self.is_demo))
    }
}
