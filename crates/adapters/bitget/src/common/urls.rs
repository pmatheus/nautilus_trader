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

//! URL helper functions for Bitget endpoints.

use super::consts::{
    BITGET_BASE_URL, BITGET_DEMO_BASE_URL, BITGET_DEMO_WS_PRIVATE_URL, BITGET_DEMO_WS_PUBLIC_URL,
    BITGET_WS_PRIVATE_URL, BITGET_WS_PUBLIC_URL,
};

/// Returns the HTTP base URL for Bitget API.
#[must_use]
pub fn get_http_base_url(is_demo: bool) -> String {
    if is_demo {
        BITGET_DEMO_BASE_URL.to_string()
    } else {
        BITGET_BASE_URL.to_string()
    }
}

/// Returns the WebSocket public URL for Bitget.
#[must_use]
pub fn get_ws_public_url(is_demo: bool) -> String {
    if is_demo {
        BITGET_DEMO_WS_PUBLIC_URL.to_string()
    } else {
        BITGET_WS_PUBLIC_URL.to_string()
    }
}

/// Returns the WebSocket private URL for Bitget.
#[must_use]
pub fn get_ws_private_url(is_demo: bool) -> String {
    if is_demo {
        BITGET_DEMO_WS_PRIVATE_URL.to_string()
    } else {
        BITGET_WS_PRIVATE_URL.to_string()
    }
}
