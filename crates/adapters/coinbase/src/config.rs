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

//! Configuration types for Coinbase adapter.

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::common::urls::{get_http_base_url, get_ws_base_url};

/// Configuration for Coinbase HTTP client.
#[derive(Clone, Debug, Builder, Serialize, Deserialize)]
#[builder(default)]
pub struct CoinbaseHttpConfig {
    /// Base URL for HTTP API.
    pub base_url: String,
    /// API key (optional for public endpoints).
    pub api_key: Option<String>,
    /// API secret (optional for public endpoints).
    pub api_secret: Option<String>,
    /// API passphrase (optional for public endpoints).
    pub api_passphrase: Option<String>,
    /// Use sandbox environment.
    pub is_sandbox: bool,
    /// HTTP timeout in seconds.
    pub timeout_secs: u64,
}

impl Default for CoinbaseHttpConfig {
    fn default() -> Self {
        Self {
            base_url: get_http_base_url(false),
            api_key: None,
            api_secret: None,
            api_passphrase: None,
            is_sandbox: false,
            timeout_secs: 60,
        }
    }
}

/// Configuration for Coinbase WebSocket client.
#[derive(Clone, Debug, Builder, Serialize, Deserialize)]
#[builder(default)]
pub struct CoinbaseWebSocketConfig {
    /// Base URL for WebSocket API.
    pub base_url: String,
    /// API key (optional for public channels).
    pub api_key: Option<String>,
    /// API secret (optional for public channels).
    pub api_secret: Option<String>,
    /// API passphrase (optional for public channels).
    pub api_passphrase: Option<String>,
    /// Use sandbox environment.
    pub is_sandbox: bool,
}

impl Default for CoinbaseWebSocketConfig {
    fn default() -> Self {
        Self {
            base_url: get_ws_base_url(false),
            api_key: None,
            api_secret: None,
            api_passphrase: None,
            is_sandbox: false,
        }
    }
}
