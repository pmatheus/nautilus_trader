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

use std::{sync::LazyLock, time::Duration};

use nautilus_model::{enums::OrderType, identifiers::Venue};
use ustr::Ustr;

pub const ASTER: &str = "ASTER";
pub static ASTER_VENUE: LazyLock<Venue> = LazyLock::new(|| Venue::new(Ustr::from(ASTER)));

// Mainnet URLs
pub const ASTER_REST_URL: &str = "https://fapi.asterdex.com";
pub const ASTER_WS_URL: &str = "wss://fstream.asterdex.com";

/// Aster supported order types.
///
/// # Notes
///
/// - Aster follows Binance Futures API patterns
/// - Supports perpetual futures trading
/// - Order types include LIMIT, MARKET, STOP, TAKE_PROFIT, LIQUIDATION
pub const ASTER_SUPPORTED_ORDER_TYPES: &[OrderType] = &[
    OrderType::Market,
    OrderType::Limit,
    OrderType::StopMarket,
    OrderType::StopLimit,
    OrderType::MarketIfTouched,
    OrderType::LimitIfTouched,
];

/// Conditional order types that use trigger orders on Aster.
pub const ASTER_CONDITIONAL_ORDER_TYPES: &[OrderType] = &[
    OrderType::StopMarket,
    OrderType::StopLimit,
    OrderType::MarketIfTouched,
    OrderType::LimitIfTouched,
];

// Default configuration values
// WebSocket connection valid for 24 hours, keep-alive needed within 60 minutes
pub const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(30);
pub const RECONNECT_BASE_BACKOFF: Duration = Duration::from_millis(250);
pub const RECONNECT_MAX_BACKOFF: Duration = Duration::from_secs(30);
pub const HTTP_TIMEOUT: Duration = Duration::from_secs(10);
pub const INFLIGHT_MAX: usize = 100;
pub const QUEUE_MAX: usize = 1000;

////////////////////////////////////////////////////////////////////////////////
// Tests
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_constants_values() {
        assert_eq!(ASTER_REST_URL, "https://fapi.asterdex.com");
        assert_eq!(ASTER_WS_URL, "wss://fstream.asterdex.com");
        assert_eq!(HEARTBEAT_INTERVAL, Duration::from_secs(30));
        assert_eq!(RECONNECT_BASE_BACKOFF, Duration::from_millis(250));
        assert_eq!(RECONNECT_MAX_BACKOFF, Duration::from_secs(30));
        assert_eq!(HTTP_TIMEOUT, Duration::from_secs(10));
        assert_eq!(INFLIGHT_MAX, 100);
        assert_eq!(QUEUE_MAX, 1000);
    }
}
