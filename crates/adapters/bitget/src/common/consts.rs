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

//! Constants for Bitget API.

/// Bitget HTTP production base URL.
pub const BITGET_BASE_URL: &str = "https://api.bitget.com";

/// Bitget WebSocket production public URL.
pub const BITGET_WS_PUBLIC_URL: &str = "wss://ws.bitget.com/v2/ws/public";

/// Bitget WebSocket production private URL.
pub const BITGET_WS_PRIVATE_URL: &str = "wss://ws.bitget.com/v2/ws/private";

/// Bitget HTTP demo/testnet base URL.
pub const BITGET_DEMO_BASE_URL: &str = "https://api.bitgetapi.com";

/// Bitget WebSocket demo/testnet public URL.
pub const BITGET_DEMO_WS_PUBLIC_URL: &str = "wss://wspap.bitget.com/v2/ws/public";

/// Bitget WebSocket demo/testnet private URL.
pub const BITGET_DEMO_WS_PRIVATE_URL: &str = "wss://wspap.bitget.com/v2/ws/private";

/// WebSocket heartbeat interval (seconds).
pub const WS_HEARTBEAT_INTERVAL_SECS: u64 = 30;

/// Maximum number of subscriptions per WebSocket connection.
pub const MAX_SUBSCRIPTIONS_PER_CONNECTION: usize = 1000;

/// Rate limit: subscriptions per hour per connection.
pub const SUBSCRIPTION_RATE_LIMIT_PER_HOUR: u32 = 240;
