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

//! URL management for Kucoin endpoints.

use super::{
    consts::{
        KUCOIN_FUTURES_HTTP_URL, KUCOIN_FUTURES_HTTP_URL_SANDBOX, KUCOIN_HTTP_URL,
        KUCOIN_HTTP_URL_SANDBOX, KUCOIN_WS_URL, KUCOIN_WS_URL_SANDBOX,
    },
    enums::KucoinProductType,
};

/// Returns the appropriate HTTP base URL based on product type and sandbox flag.
#[must_use]
pub fn get_http_base_url(product_type: KucoinProductType, is_sandbox: bool) -> String {
    match (product_type, is_sandbox) {
        (KucoinProductType::Spot, false) => KUCOIN_HTTP_URL.to_string(),
        (KucoinProductType::Spot, true) => KUCOIN_HTTP_URL_SANDBOX.to_string(),
        (KucoinProductType::Futures, false) => KUCOIN_FUTURES_HTTP_URL.to_string(),
        (KucoinProductType::Futures, true) => KUCOIN_FUTURES_HTTP_URL_SANDBOX.to_string(),
    }
}

/// Returns the appropriate WebSocket base URL based on sandbox flag.
///
/// Note: This URL will be replaced with the token-based URL from the API response.
#[must_use]
pub fn get_ws_base_url(is_sandbox: bool) -> String {
    if is_sandbox {
        KUCOIN_WS_URL_SANDBOX.to_string()
    } else {
        KUCOIN_WS_URL.to_string()
    }
}
