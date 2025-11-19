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

//! Core constants shared across the Coinbase adapter components.

use std::sync::LazyLock;

use nautilus_model::{enums::OrderType, identifiers::Venue};
use ustr::Ustr;

pub const COINBASE: &str = "COINBASE";
pub static COINBASE_VENUE: LazyLock<Venue> = LazyLock::new(|| Venue::new(Ustr::from(COINBASE)));

// HTTP and WebSocket URLs
pub const COINBASE_HTTP_URL: &str = "https://api.exchange.coinbase.com";
pub const COINBASE_WS_URL: &str = "wss://ws-feed.exchange.coinbase.com";

// Sandbox URLs for testing
pub const COINBASE_SANDBOX_HTTP_URL: &str = "https://api-public.sandbox.exchange.coinbase.com";
pub const COINBASE_SANDBOX_WS_URL: &str = "wss://ws-feed-public.sandbox.exchange.coinbase.com";

/// Coinbase supported order types for spot trading.
///
/// # Notes
///
/// - Market orders are supported.
/// - Limit orders are supported with GTC or IOC time in force.
/// - Stop orders (stop-loss and stop-entry) are supported.
/// - Post-only is supported as a flag on limit orders.
pub const COINBASE_SUPPORTED_ORDER_TYPES: &[OrderType] = &[
    OrderType::Market,
    OrderType::Limit,
    OrderType::StopMarket,
    OrderType::StopLimit,
];

// API request headers
pub const COINBASE_HEADER_ACCESS_KEY: &str = "CB-ACCESS-KEY";
pub const COINBASE_HEADER_ACCESS_SIGN: &str = "CB-ACCESS-SIGN";
pub const COINBASE_HEADER_ACCESS_TIMESTAMP: &str = "CB-ACCESS-TIMESTAMP";
pub const COINBASE_HEADER_ACCESS_PASSPHRASE: &str = "CB-ACCESS-PASSPHRASE";

// API rate limits (requests per second)
pub const COINBASE_RATE_LIMIT_PUBLIC: usize = 10;
pub const COINBASE_RATE_LIMIT_PRIVATE: usize = 10;

// WebSocket channel names
pub const COINBASE_WS_CHANNEL_HEARTBEAT: &str = "heartbeat";
pub const COINBASE_WS_CHANNEL_STATUS: &str = "status";
pub const COINBASE_WS_CHANNEL_TICKER: &str = "ticker";
pub const COINBASE_WS_CHANNEL_LEVEL2: &str = "level2";
pub const COINBASE_WS_CHANNEL_MATCHES: &str = "matches";
pub const COINBASE_WS_CHANNEL_FULL: &str = "full";
pub const COINBASE_WS_CHANNEL_USER: &str = "user";

// Precision limits
pub const COINBASE_MAX_PRICE_PRECISION: u8 = 8;
pub const COINBASE_MAX_SIZE_PRECISION: u8 = 8;
