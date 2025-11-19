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

//! Kucoin adapter constants.

use std::sync::LazyLock;

use nautilus_core::nanos::UnixNanos;
use nautilus_model::identifiers::Venue;
use ustr::Ustr;

/// Production HTTP API base URL (SPOT)
pub const KUCOIN_HTTP_URL: &str = "https://api.kucoin.com";

/// Sandbox HTTP API base URL (SPOT)
pub const KUCOIN_HTTP_URL_SANDBOX: &str = "https://openapi-sandbox.kucoin.com";

/// Production HTTP API base URL (FUTURES)
pub const KUCOIN_FUTURES_HTTP_URL: &str = "https://api-futures.kucoin.com";

/// Sandbox HTTP API base URL (FUTURES)
pub const KUCOIN_FUTURES_HTTP_URL_SANDBOX: &str = "https://api-sandbox-futures.kucoin.com";

/// Production WebSocket API base URL (will be replaced with token-based URL)
pub const KUCOIN_WS_URL: &str = "wss://ws-api-spot.kucoin.com";

/// Sandbox WebSocket API base URL (will be replaced with token-based URL)
pub const KUCOIN_WS_URL_SANDBOX: &str = "wss://ws-api-sandbox.kucoin.com";

/// Kucoin venue identifier
pub static KUCOIN_VENUE: LazyLock<Venue> = LazyLock::new(|| Venue::from("KUCOIN"));

/// Kucoin client ID for Nautilus
pub static KUCOIN_CLIENT_ID: LazyLock<ustr::Ustr> =
    LazyLock::new(|| ustr::Ustr::from("NAUTILUS"));

/// Kucoin broker ID for Nautilus (used in client order IDs)
pub const KUCOIN_NAUTILUS_BROKER_ID: &str = "NAUTILUS";

/// WebSocket token validity duration (24 hours as per Kucoin docs)
pub const KUCOIN_WS_TOKEN_VALIDITY_SECS: u64 = 24 * 60 * 60;

/// Kucoin success code for API responses
pub const KUCOIN_SUCCESS_CODE: &str = "200000";

/// Kucoin WebSocket ping interval (default 18 seconds as recommended)
pub const KUCOIN_WS_PING_INTERVAL_SECS: u64 = 18;

/// Kucoin WebSocket pong timeout (default 10 seconds as recommended)
pub const KUCOIN_WS_PONG_TIMEOUT_SECS: u64 = 10;

/// Maximum message ID for Kucoin WebSocket (incremental counter)
pub const KUCOIN_WS_MAX_MSG_ID: u64 = 9999999999;
