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

//! URL construction utilities for Bitget API endpoints.

use super::enums::BitgetEnvironment;

/// Returns the HTTP base URL for the given environment.
#[must_use]
pub fn bitget_http_base_url(environment: BitgetEnvironment) -> &'static str {
    match environment {
        BitgetEnvironment::Mainnet => "https://api.bitget.com",
        BitgetEnvironment::Testnet => "https://api.bitget.com",
    }
}

/// Returns the public WebSocket URL for the given environment.
#[must_use]
pub fn bitget_ws_public_url(environment: BitgetEnvironment) -> &'static str {
    match environment {
        BitgetEnvironment::Mainnet => "wss://ws.bitget.com/v2/ws/public",
        BitgetEnvironment::Testnet => "wss://ws.bitget.com/v2/ws/public",
    }
}

/// Returns the private WebSocket URL for the given environment.
#[must_use]
pub fn bitget_ws_private_url(environment: BitgetEnvironment) -> &'static str {
    match environment {
        BitgetEnvironment::Mainnet => "wss://ws.bitget.com/v2/ws/private",
        BitgetEnvironment::Testnet => "wss://ws.bitget.com/v2/ws/private",
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
    fn test_http_base_url_mainnet() {
        assert_eq!(
            bitget_http_base_url(BitgetEnvironment::Mainnet),
            "https://api.bitget.com"
        );
    }

    #[rstest]
    fn test_http_base_url_testnet() {
        assert_eq!(
            bitget_http_base_url(BitgetEnvironment::Testnet),
            "https://api.bitget.com"
        );
    }

    #[rstest]
    fn test_ws_public_url_mainnet() {
        assert_eq!(
            bitget_ws_public_url(BitgetEnvironment::Mainnet),
            "wss://ws.bitget.com/v2/ws/public"
        );
    }

    #[rstest]
    fn test_ws_public_url_testnet() {
        assert_eq!(
            bitget_ws_public_url(BitgetEnvironment::Testnet),
            "wss://ws.bitget.com/v2/ws/public"
        );
    }

    #[rstest]
    fn test_ws_private_url_mainnet() {
        assert_eq!(
            bitget_ws_private_url(BitgetEnvironment::Mainnet),
            "wss://ws.bitget.com/v2/ws/private"
        );
    }

    #[rstest]
    fn test_ws_private_url_testnet() {
        assert_eq!(
            bitget_ws_private_url(BitgetEnvironment::Testnet),
            "wss://ws.bitget.com/v2/ws/private"
        );
    }
}
