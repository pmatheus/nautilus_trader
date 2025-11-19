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

//! URL helpers and endpoint metadata for Coinbase services.

use nautilus_core::env::get_env_var;

/// Coinbase endpoint types for determining URL and authentication requirements.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "python", pyo3::pyclass)]
pub enum CoinbaseEndpointType {
    Public,
    Private,
}

/// Checks if endpoint requires authentication.
pub fn requires_authentication(endpoint_type: CoinbaseEndpointType) -> bool {
    matches!(endpoint_type, CoinbaseEndpointType::Private)
}

/// Gets the HTTP base URL.
pub fn get_http_base_url(is_sandbox: bool) -> String {
    if is_sandbox {
        get_env_var("COINBASE_SANDBOX_BASE_URL_HTTP").unwrap_or_else(|_| {
            "https://api-public.sandbox.exchange.coinbase.com".to_string()
        })
    } else {
        get_env_var("COINBASE_BASE_URL_HTTP")
            .unwrap_or_else(|_| "https://api.exchange.coinbase.com".to_string())
    }
}

/// Gets the WebSocket base URL.
pub fn get_ws_base_url(is_sandbox: bool) -> String {
    if is_sandbox {
        get_env_var("COINBASE_SANDBOX_BASE_URL_WS").unwrap_or_else(|_| {
            "wss://ws-feed-public.sandbox.exchange.coinbase.com".to_string()
        })
    } else {
        get_env_var("COINBASE_BASE_URL_WS")
            .unwrap_or_else(|_| "wss://ws-feed.exchange.coinbase.com".to_string())
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_endpoint_authentication() {
        assert!(!requires_authentication(CoinbaseEndpointType::Public));
        assert!(requires_authentication(CoinbaseEndpointType::Private));
    }

    #[rstest]
    fn test_http_base_url_production() {
        assert_eq!(
            get_http_base_url(false),
            "https://api.exchange.coinbase.com"
        );
    }

    #[rstest]
    fn test_http_base_url_sandbox() {
        assert_eq!(
            get_http_base_url(true),
            "https://api-public.sandbox.exchange.coinbase.com"
        );
    }

    #[rstest]
    fn test_ws_base_url_production() {
        assert_eq!(
            get_ws_base_url(false),
            "wss://ws-feed.exchange.coinbase.com"
        );
    }

    #[rstest]
    fn test_ws_base_url_sandbox() {
        assert_eq!(
            get_ws_base_url(true),
            "wss://ws-feed-public.sandbox.exchange.coinbase.com"
        );
    }
}
