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

//! Exchange-specific adapters and quirks for L3 reconstruction.
//!
//! Handles exchange-specific conventions, price precision rules, and other
//! idiosyncrasies that affect L3 orderbook reconstruction.
//!
//! # Design
//!
//! Different exchanges have unique characteristics in their market data:
//! - Sequence number handling (gaps, resets, wraparound)
//! - Timestamp jitter and synchronization issues
//! - Snapshot frequency and formatting
//! - Trade aggregation and reporting patterns
//! - Implied order support (e.g., spread orders)
//!
//! This module provides exchange-specific configuration to handle these quirks
//! in a structured way, improving L3 reconstruction accuracy.

/// Strategy for handling sequence number gaps.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GapStrategy {
    /// Ignore small gaps (< threshold)
    Ignore,
    /// Request snapshot to resync
    RequestSnapshot,
    /// Continue processing with warning
    ContinueWithWarning,
    /// Fail and stop processing
    Fail,
}

/// Exchange-specific quirks and behavior configuration.
///
/// Implementations of this trait provide exchange-specific knowledge that
/// improves L3 reconstruction accuracy.
pub trait ExchangeQuirks {
    /// Returns the exchange name.
    fn exchange_name(&self) -> &str;

    /// Handles sequence number gaps.
    ///
    /// # Arguments
    ///
    /// * `expected` - The expected sequence number
    /// * `actual` - The actual received sequence number
    /// * `gap` - The size of the gap (actual - expected)
    ///
    /// # Returns
    ///
    /// Strategy for handling this gap.
    fn handle_sequence_gap(&self, expected: u64, actual: u64, gap: u64) -> GapStrategy;

    /// Returns the timestamp tolerance in nanoseconds.
    ///
    /// Events within this tolerance are considered simultaneous for
    /// correlation purposes.
    fn timestamp_tolerance_ns(&self) -> u64;

    /// Returns the recommended snapshot refresh interval in seconds.
    ///
    /// How often should we request fresh snapshots to maintain accuracy?
    fn snapshot_interval_sec(&self) -> u64;

    /// Returns whether this exchange supports implied orders.
    ///
    /// Implied orders are synthetic orders created by the exchange (e.g.,
    /// spread orders in derivatives markets).
    fn supports_implied_orders(&self) -> bool;

    /// Returns the maximum expected price levels per side.
    ///
    /// Used for capacity planning and anomaly detection.
    fn max_price_levels(&self) -> usize {
        1000 // Default: 1000 levels
    }

    /// Returns the typical orderbook depth.
    ///
    /// Number of price levels typically maintained.
    fn typical_depth(&self) -> usize {
        100 // Default: 100 levels
    }
}

/// Bitget exchange quirks.
///
/// Based on analysis of Bitget market data characteristics.
pub struct BitgetQuirks;

impl ExchangeQuirks for BitgetQuirks {
    fn exchange_name(&self) -> &str {
        "BITGET"
    }

    fn handle_sequence_gap(&self, _expected: u64, _actual: u64, gap: u64) -> GapStrategy {
        // Bitget occasionally has small gaps due to message coalescing
        if gap <= 3 {
            GapStrategy::Ignore
        } else if gap <= 10 {
            GapStrategy::ContinueWithWarning
        } else {
            GapStrategy::RequestSnapshot
        }
    }

    fn timestamp_tolerance_ns(&self) -> u64 {
        // Bitget has ~5-10ms timestamp jitter
        10_000_000 // 10ms
    }

    fn snapshot_interval_sec(&self) -> u64 {
        300 // 5 minutes
    }

    fn supports_implied_orders(&self) -> bool {
        false
    }

    fn max_price_levels(&self) -> usize {
        500 // Bitget typically maintains fewer levels
    }

    fn typical_depth(&self) -> usize {
        50
    }
}

/// Kraken exchange quirks.
///
/// Based on Kraken's documented behavior and observed characteristics.
pub struct KrakenQuirks;

impl ExchangeQuirks for KrakenQuirks {
    fn exchange_name(&self) -> &str {
        "KRAKEN"
    }

    fn handle_sequence_gap(&self, _expected: u64, _actual: u64, gap: u64) -> GapStrategy {
        // Kraken has very reliable sequencing
        if gap <= 1 {
            GapStrategy::Ignore
        } else {
            GapStrategy::RequestSnapshot
        }
    }

    fn timestamp_tolerance_ns(&self) -> u64 {
        // Kraken has excellent timestamp accuracy
        5_000_000 // 5ms
    }

    fn snapshot_interval_sec(&self) -> u64 {
        600 // 10 minutes - Kraken is very stable
    }

    fn supports_implied_orders(&self) -> bool {
        false
    }

    fn max_price_levels(&self) -> usize {
        1000
    }

    fn typical_depth(&self) -> usize {
        100
    }
}

/// Hyperliquid exchange quirks.
///
/// Hyperliquid-specific behavior patterns.
pub struct HyperliquidQuirks;

impl ExchangeQuirks for HyperliquidQuirks {
    fn exchange_name(&self) -> &str {
        "HYPERLIQUID"
    }

    fn handle_sequence_gap(&self, _expected: u64, _actual: u64, gap: u64) -> GapStrategy {
        // Hyperliquid uses blockchain-based sequencing
        if gap <= 2 {
            GapStrategy::Ignore
        } else if gap <= 5 {
            GapStrategy::ContinueWithWarning
        } else {
            GapStrategy::RequestSnapshot
        }
    }

    fn timestamp_tolerance_ns(&self) -> u64 {
        // Block time can cause timestamp clustering
        50_000_000 // 50ms
    }

    fn snapshot_interval_sec(&self) -> u64 {
        180 // 3 minutes
    }

    fn supports_implied_orders(&self) -> bool {
        true // Hyperliquid supports perpetual swaps with implied orders
    }

    fn max_price_levels(&self) -> usize {
        200 // On-chain orderbook is typically shallower
    }

    fn typical_depth(&self) -> usize {
        30
    }
}

/// Generic exchange quirks with conservative defaults.
///
/// Use this for exchanges without specific quirks configuration.
pub struct GenericQuirks {
    /// Exchange name
    pub exchange_name: String,
}

impl GenericQuirks {
    /// Creates a new generic quirks instance.
    #[must_use]
    pub fn new(exchange_name: impl Into<String>) -> Self {
        Self {
            exchange_name: exchange_name.into(),
        }
    }
}

impl ExchangeQuirks for GenericQuirks {
    fn exchange_name(&self) -> &str {
        &self.exchange_name
    }

    fn handle_sequence_gap(&self, _expected: u64, _actual: u64, gap: u64) -> GapStrategy {
        // Conservative: request snapshot on any significant gap
        if gap <= 2 {
            GapStrategy::Ignore
        } else {
            GapStrategy::RequestSnapshot
        }
    }

    fn timestamp_tolerance_ns(&self) -> u64 {
        // Conservative: assume 20ms tolerance
        20_000_000
    }

    fn snapshot_interval_sec(&self) -> u64 {
        // Conservative: refresh every 5 minutes
        300
    }

    fn supports_implied_orders(&self) -> bool {
        false // Assume no implied orders
    }
}

/// Factory for creating exchange quirks implementations.
pub struct ExchangeQuirksFactory;

impl ExchangeQuirksFactory {
    /// Creates an exchange quirks implementation for the given exchange.
    ///
    /// Returns a boxed trait object that can be used polymorphically.
    #[must_use]
    pub fn create(exchange: &str) -> Box<dyn ExchangeQuirks> {
        let exchange_upper = exchange.to_uppercase();

        match exchange_upper.as_str() {
            "BITGET" => Box::new(BitgetQuirks),
            "KRAKEN" => Box::new(KrakenQuirks),
            "HYPERLIQUID" => Box::new(HyperliquidQuirks),
            _ => Box::new(GenericQuirks::new(exchange)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitget_quirks() {
        let quirks = BitgetQuirks;

        assert_eq!(quirks.exchange_name(), "BITGET");
        assert_eq!(quirks.timestamp_tolerance_ns(), 10_000_000);
        assert!(!quirks.supports_implied_orders());

        // Test gap handling
        assert_eq!(quirks.handle_sequence_gap(100, 101, 1), GapStrategy::Ignore);
        assert_eq!(
            quirks.handle_sequence_gap(100, 105, 5),
            GapStrategy::ContinueWithWarning
        );
        assert_eq!(
            quirks.handle_sequence_gap(100, 120, 20),
            GapStrategy::RequestSnapshot
        );
    }

    #[test]
    fn test_kraken_quirks() {
        let quirks = KrakenQuirks;

        assert_eq!(quirks.exchange_name(), "KRAKEN");
        assert_eq!(quirks.timestamp_tolerance_ns(), 5_000_000);
        assert!(!quirks.supports_implied_orders());

        // Kraken is strict about sequencing
        assert_eq!(
            quirks.handle_sequence_gap(100, 102, 2),
            GapStrategy::RequestSnapshot
        );
    }

    #[test]
    fn test_hyperliquid_quirks() {
        let quirks = HyperliquidQuirks;

        assert_eq!(quirks.exchange_name(), "HYPERLIQUID");
        assert!(quirks.supports_implied_orders());
        assert_eq!(quirks.max_price_levels(), 200);

        // Test timestamp tolerance (should be higher due to block time)
        assert_eq!(quirks.timestamp_tolerance_ns(), 50_000_000);
    }

    #[test]
    fn test_generic_quirks() {
        let quirks = GenericQuirks::new("UNKNOWN_EXCHANGE");

        assert_eq!(quirks.exchange_name(), "UNKNOWN_EXCHANGE");
        assert_eq!(quirks.timestamp_tolerance_ns(), 20_000_000);
        assert!(!quirks.supports_implied_orders());

        // Generic should be conservative
        assert_eq!(
            quirks.handle_sequence_gap(100, 103, 3),
            GapStrategy::RequestSnapshot
        );
    }

    #[test]
    fn test_factory() {
        let bitget = ExchangeQuirksFactory::create("bitget");
        assert_eq!(bitget.exchange_name(), "BITGET");

        let kraken = ExchangeQuirksFactory::create("KRAKEN");
        assert_eq!(kraken.exchange_name(), "KRAKEN");

        let hyperliquid = ExchangeQuirksFactory::create("hyperliquid");
        assert_eq!(hyperliquid.exchange_name(), "HYPERLIQUID");

        let unknown = ExchangeQuirksFactory::create("UNKNOWN");
        assert_eq!(unknown.exchange_name(), "UNKNOWN");
    }

    #[test]
    fn test_default_methods() {
        let quirks = GenericQuirks::new("TEST");

        // Test default implementations
        assert_eq!(quirks.max_price_levels(), 1000);
        assert_eq!(quirks.typical_depth(), 100);
        assert_eq!(quirks.snapshot_interval_sec(), 300);
    }

    #[test]
    fn test_gap_strategy_comparison() {
        // Test that gap strategies can be compared
        assert_eq!(GapStrategy::Ignore, GapStrategy::Ignore);
        assert_ne!(GapStrategy::Ignore, GapStrategy::RequestSnapshot);
    }

    #[test]
    fn test_all_exchanges_have_reasonable_defaults() {
        let exchanges = vec!["BITGET", "KRAKEN", "HYPERLIQUID", "UNKNOWN"];

        for exchange in exchanges {
            let quirks = ExchangeQuirksFactory::create(exchange);

            // All should have reasonable timestamp tolerance
            assert!(quirks.timestamp_tolerance_ns() > 0);
            assert!(quirks.timestamp_tolerance_ns() < 100_000_000); // < 100ms

            // All should have reasonable snapshot intervals
            assert!(quirks.snapshot_interval_sec() > 0);
            assert!(quirks.snapshot_interval_sec() < 3600); // < 1 hour

            // All should have reasonable depth limits
            assert!(quirks.max_price_levels() > 0);
            assert!(quirks.max_price_levels() <= 10000);
        }
    }
}
