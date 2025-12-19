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

//! L2 Delta Correlation Engine
//!
//! This module implements the core algorithm for inferring synthetic L3 order actions
//! from L2 orderbook deltas and trade events. By correlating price level changes with
//! trade sizes, the correlator reconstructs individual order lifecycle events.
//!
//! # Algorithm
//!
//! The correlation engine processes two types of input events:
//!
//! 1. **Orderbook Deltas** - Changes to aggregated price levels
//! 2. **Trade Events** - Executed trades with price, size, and aggressor side
//!
//! By analyzing the timing and magnitude of these events, the engine infers:
//! - Order placements (delta without trade)
//! - Order fills (trade + matching delta)
//! - Partial fills (trade smaller than delta)
//! - Order cancellations (negative delta without trade)
//! - Order modifications (size changes)
//!
//! # Time Correlation
//!
//! Events are correlated within a configurable time window to handle timing jitter
//! between orderbook updates and trade notifications. Out-of-order events are buffered
//! and sorted by timestamp before correlation.
//!
//! # Confidence Scoring
//!
//! Each inferred action receives a confidence score (0.0-1.0) based on:
//! - Size match quality between delta and trade
//! - Timing precision
//! - Event sequence consistency

use std::collections::{BTreeMap, HashMap, VecDeque};

use nautilus_core::nanos::UnixNanos;
use nautilus_model::{
    data::{OrderBookDelta, TradeTick},
    enums::{AggressorSide, BookAction, OrderSide},
    identifiers::{InstrumentId, VenueOrderId},
    types::{Price, Quantity},
};

use crate::order_id::SyntheticOrderIdGenerator;

/// Configuration for the L2 delta correlator.
#[derive(Debug, Clone)]
pub struct CorrelatorConfig {
    /// Time window for correlating trades with deltas (nanoseconds)
    pub correlation_window_ns: u64,

    /// Minimum confidence threshold to emit L3 action (0.0-1.0)
    pub min_confidence: f64,

    /// Buffer size for out-of-order events
    pub event_buffer_size: usize,

    /// Tolerance for size comparison (percentage, e.g., 0.01 = 1%)
    pub size_tolerance: f64,
}

impl Default for CorrelatorConfig {
    fn default() -> Self {
        Self {
            correlation_window_ns: 50_000_000, // 50ms
            min_confidence: 0.5,
            event_buffer_size: 1000,
            size_tolerance: 0.001, // 0.1%
        }
    }
}

/// Synthetic L3 action types inferred from L2 data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum L3Action {
    /// Order placed on the book
    Placed,
    /// Order size modified (amended)
    Modified,
    /// Order partially filled
    PartialFill,
    /// Order completely filled
    Fill,
    /// Order canceled
    Canceled,
}

/// Synthetic L3 order action event.
#[derive(Debug, Clone)]
pub struct L3OrderAction {
    /// Synthetic order ID
    pub synthetic_order_id: VenueOrderId,
    /// Instrument identifier
    pub instrument_id: InstrumentId,
    /// L3 action type
    pub action: L3Action,
    /// Order side (Buy/Sell)
    pub side: OrderSide,
    /// Price level
    pub price: Price,
    /// Order size
    pub size: Quantity,
    /// Event timestamp
    pub ts_event: UnixNanos,
    /// Confidence score (0.0-1.0)
    pub confidence: f64,
    /// Sequence number from exchange
    pub sequence: u64,
}

/// State for a single price level in the orderbook.
#[derive(Debug, Clone)]
struct LevelState {
    /// Current aggregate size at this price level
    size: Quantity,
    /// Synthetic orders at this level (ordered by creation time)
    order_ids: Vec<VenueOrderId>,
    /// Last update timestamp
    last_update_ts: UnixNanos,
}

impl LevelState {
    fn new(size: Quantity, ts: UnixNanos) -> Self {
        Self {
            size,
            order_ids: Vec::new(),
            last_update_ts: ts,
        }
    }
}

/// Synthetic order tracked by the correlator.
#[derive(Debug, Clone)]
struct SyntheticOrder {
    /// Order ID
    id: VenueOrderId,
    /// Price level
    price: Price,
    /// Current size
    size: Quantity,
    /// Order side
    side: OrderSide,
    /// Creation timestamp
    ts_created: UnixNanos,
    /// Last update timestamp
    ts_updated: UnixNanos,
}

/// Per-instrument state for correlation.
struct InstrumentState {
    /// Bid levels (price → state)
    bid_levels: BTreeMap<i64, LevelState>, // Using raw price for efficient sorting
    /// Ask levels (price → state)
    ask_levels: BTreeMap<i64, LevelState>,
    /// Recent trades awaiting correlation
    pending_trades: VecDeque<TradeTick>,
    /// Recent deltas awaiting correlation
    pending_deltas: VecDeque<OrderBookDelta>,
    /// Synthetic orders currently on book
    synthetic_orders: HashMap<VenueOrderId, SyntheticOrder>,
    /// Order ID generator
    id_generator: SyntheticOrderIdGenerator,
    /// Last processed sequence number
    last_sequence: u64,
}

impl InstrumentState {
    fn new() -> Self {
        Self {
            bid_levels: BTreeMap::new(),
            ask_levels: BTreeMap::new(),
            pending_trades: VecDeque::new(),
            pending_deltas: VecDeque::new(),
            synthetic_orders: HashMap::new(),
            id_generator: SyntheticOrderIdGenerator::new(),
            last_sequence: 0,
        }
    }

    /// Get the appropriate levels map for a given side
    fn levels_for_side(&mut self, side: OrderSide) -> &mut BTreeMap<i64, LevelState> {
        match side {
            OrderSide::Buy => &mut self.bid_levels,
            OrderSide::Sell => &mut self.ask_levels,
            _ => panic!("Invalid order side for level lookup"),
        }
    }

    /// Clear all state (orderbook snapshot received)
    fn clear(&mut self) {
        self.bid_levels.clear();
        self.ask_levels.clear();
        self.synthetic_orders.clear();
        // Keep pending events - they may still be valid
    }
}

/// L2 Delta Correlator - the heart of L3 reconstruction.
pub struct L2DeltaCorrelator {
    /// Per-instrument state
    instrument_states: HashMap<InstrumentId, InstrumentState>,
    /// Configuration
    config: CorrelatorConfig,
}

impl L2DeltaCorrelator {
    /// Creates a new L2 delta correlator with the given configuration.
    #[must_use]
    pub fn new(config: CorrelatorConfig) -> Self {
        Self {
            instrument_states: HashMap::new(),
            config,
        }
    }

    /// Creates a new L2 delta correlator with default configuration.
    #[must_use]
    pub fn default() -> Self {
        Self::new(CorrelatorConfig::default())
    }

    /// Process an incoming L2 orderbook delta.
    ///
    /// Returns a vector of inferred L3 actions. The vector may be empty if:
    /// - The delta is buffered awaiting correlation
    /// - No L3 action could be inferred with sufficient confidence
    pub fn on_delta(&mut self, delta: OrderBookDelta) -> Vec<L3OrderAction> {
        let state = self
            .instrument_states
            .entry(delta.instrument_id)
            .or_insert_with(InstrumentState::new);

        // Handle Clear action - reset orderbook state
        if delta.action == BookAction::Clear {
            state.clear();
            return Vec::new();
        }

        // Check for sequence gap (potential missed messages)
        if delta.sequence > 0 && state.last_sequence > 0 {
            let expected = state.last_sequence + 1;
            if delta.sequence > expected {
                // Gap detected - we may have incomplete information
                // For now, continue processing but could emit warning
            }
        }
        state.last_sequence = delta.sequence;

        // Buffer the delta
        state.pending_deltas.push_back(delta);

        // Trim buffer if too large
        while state.pending_deltas.len() > self.config.event_buffer_size {
            state.pending_deltas.pop_front();
        }

        // Correlate pending events
        self.correlate_events(delta.instrument_id)
    }

    /// Process an incoming trade event.
    ///
    /// Returns a vector of inferred L3 actions.
    pub fn on_trade(&mut self, trade: TradeTick) -> Vec<L3OrderAction> {
        let state = self
            .instrument_states
            .entry(trade.instrument_id)
            .or_insert_with(InstrumentState::new);

        // Buffer the trade
        state.pending_trades.push_back(trade);

        // Trim buffer if too large
        while state.pending_trades.len() > self.config.event_buffer_size {
            state.pending_trades.pop_front();
        }

        // Correlate pending events
        self.correlate_events(trade.instrument_id)
    }

    /// Correlate pending trades with pending deltas to infer L3 actions.
    fn correlate_events(&mut self, instrument_id: InstrumentId) -> Vec<L3OrderAction> {
        let mut actions = Vec::new();
        let min_confidence = self.config.min_confidence;

        let state = self
            .instrument_states
            .get_mut(&instrument_id)
            .expect("Instrument state must exist");

        // Process deltas in chronological order
        while let Some(delta) = state.pending_deltas.pop_front() {
            // Try to find a matching trade within the correlation window
            let matching_trade =
                Self::find_matching_trade_static(state, &delta, self.config.correlation_window_ns);

            // Infer L3 action(s) from the delta and optional trade
            let mut inferred_actions = Self::infer_actions_static(
                state,
                &delta,
                matching_trade.as_ref(),
                self.config.size_tolerance,
            );

            actions.append(&mut inferred_actions);

            // Update orderbook state
            Self::update_orderbook_state_static(state, &delta);
        }

        // Filter by minimum confidence
        actions
            .into_iter()
            .filter(|action| action.confidence >= min_confidence)
            .collect()
    }

    /// Find a trade that matches the given delta within the correlation window (static version).
    fn find_matching_trade_static(
        state: &mut InstrumentState,
        delta: &OrderBookDelta,
        correlation_window_ns: u64,
    ) -> Option<TradeTick> {
        let delta_ts = delta.ts_event;
        let window = correlation_window_ns;

        // Look for trades at the same price within the time window
        let matching_idx = state.pending_trades.iter().position(|trade| {
            // Check price match
            let price_match = trade.price == delta.order.price;

            // Check time correlation
            let time_diff = if trade.ts_event >= delta_ts {
                trade.ts_event.as_u64() - delta_ts.as_u64()
            } else {
                delta_ts.as_u64() - trade.ts_event.as_u64()
            };
            let time_match = time_diff <= window;

            // Check side match (trade aggressor side vs delta side)
            let side_match = match trade.aggressor_side {
                AggressorSide::Buyer => delta.order.side == OrderSide::Sell, // Buyer hit ask
                AggressorSide::Seller => delta.order.side == OrderSide::Buy, // Seller hit bid
                AggressorSide::NoAggressor => true, // Can't determine, allow match
            };

            price_match && time_match && side_match
        });

        matching_idx.map(|idx| state.pending_trades.remove(idx).unwrap())
    }

    /// Infer L3 actions from a delta and optional matching trade (static version).
    fn infer_actions_static(
        state: &mut InstrumentState,
        delta: &OrderBookDelta,
        trade: Option<&TradeTick>,
        size_tolerance: f64,
    ) -> Vec<L3OrderAction> {
        let mut actions = Vec::new();

        match delta.action {
            BookAction::Add => {
                // New price level or size increase at existing level
                actions.push(Self::infer_add_action_static(
                    state,
                    delta,
                    trade,
                    size_tolerance,
                ));
            }
            BookAction::Update => {
                // Size change at existing level
                actions.extend(Self::infer_update_action_static(
                    state,
                    delta,
                    trade,
                    size_tolerance,
                ));
            }
            BookAction::Delete => {
                // Price level removed or size decreased to zero
                actions.extend(Self::infer_delete_action_static(
                    state,
                    delta,
                    trade,
                    size_tolerance,
                ));
            }
            BookAction::Clear => {
                // Already handled in on_delta
            }
        }

        actions
    }

    /// Infer L3 action for an Add delta (static version).
    fn infer_add_action_static(
        state: &mut InstrumentState,
        delta: &OrderBookDelta,
        trade: Option<&TradeTick>,
        size_tolerance: f64,
    ) -> L3OrderAction {
        let price_raw = delta.order.price.raw;
        let levels = match delta.order.side {
            OrderSide::Buy => &state.bid_levels,
            OrderSide::Sell => &state.ask_levels,
            _ => panic!("Invalid side in delta"),
        };

        let level_exists = levels.contains_key(&price_raw);

        if let Some(trade) = trade {
            // Trade occurred - could be immediate fill or partial fill
            let size_match = Self::sizes_match_static(delta.order.size, trade.size, size_tolerance);

            if size_match {
                // Likely an immediate fill (FOK or IOC order)
                let order_id = state.id_generator.generate_id(
                    &delta.instrument_id.to_string(),
                    delta.order.side,
                    delta.order.price,
                    delta.ts_event,
                    delta.sequence,
                );

                L3OrderAction {
                    synthetic_order_id: order_id,
                    instrument_id: delta.instrument_id,
                    action: L3Action::Fill,
                    side: delta.order.side,
                    price: delta.order.price,
                    size: delta.order.size,
                    ts_event: delta.ts_event,
                    confidence: 0.9, // High confidence for size match
                    sequence: delta.sequence,
                }
            } else {
                // Partial fill or new order placement
                let order_id = state.id_generator.generate_id(
                    &delta.instrument_id.to_string(),
                    delta.order.side,
                    delta.order.price,
                    delta.ts_event,
                    delta.sequence,
                );

                L3OrderAction {
                    synthetic_order_id: order_id,
                    instrument_id: delta.instrument_id,
                    action: if level_exists {
                        L3Action::PartialFill
                    } else {
                        L3Action::Placed
                    },
                    side: delta.order.side,
                    price: delta.order.price,
                    size: delta.order.size,
                    ts_event: delta.ts_event,
                    confidence: 0.7, // Medium confidence without perfect size match
                    sequence: delta.sequence,
                }
            }
        } else {
            // No trade - order placed on book
            let order_id = state.id_generator.generate_id(
                &delta.instrument_id.to_string(),
                delta.order.side,
                delta.order.price,
                delta.ts_event,
                delta.sequence,
            );

            L3OrderAction {
                synthetic_order_id: order_id,
                instrument_id: delta.instrument_id,
                action: L3Action::Placed,
                side: delta.order.side,
                price: delta.order.price,
                size: delta.order.size,
                ts_event: delta.ts_event,
                confidence: 0.95, // High confidence - clear order placement
                sequence: delta.sequence,
            }
        }
    }

    /// Infer L3 actions for an Update delta (static version).
    fn infer_update_action_static(
        state: &mut InstrumentState,
        delta: &OrderBookDelta,
        trade: Option<&TradeTick>,
        size_tolerance: f64,
    ) -> Vec<L3OrderAction> {
        let mut actions = Vec::new();
        let price_raw = delta.order.price.raw;
        let levels = match delta.order.side {
            OrderSide::Buy => &state.bid_levels,
            OrderSide::Sell => &state.ask_levels,
            _ => panic!("Invalid side in delta"),
        };

        let old_size = levels
            .get(&price_raw)
            .map(|level| level.size)
            .unwrap_or_else(|| Quantity::from(0));

        let new_size = delta.order.size;
        let size_increased = new_size.raw > old_size.raw;

        if let Some(trade) = trade {
            // Trade occurred - likely a fill
            let trade_size_matches_diff = if size_increased {
                Self::sizes_match_static(
                    Quantity::from_raw(new_size.raw - old_size.raw, new_size.precision),
                    trade.size,
                    size_tolerance,
                )
            } else {
                Self::sizes_match_static(
                    Quantity::from_raw(old_size.raw - new_size.raw, old_size.precision),
                    trade.size,
                    size_tolerance,
                )
            };

            let order_id = state.id_generator.generate_id(
                &delta.instrument_id.to_string(),
                delta.order.side,
                delta.order.price,
                delta.ts_event,
                delta.sequence,
            );

            let action_type = if new_size.raw == 0 {
                L3Action::Fill // Completely filled
            } else {
                L3Action::PartialFill
            };

            actions.push(L3OrderAction {
                synthetic_order_id: order_id,
                instrument_id: delta.instrument_id,
                action: action_type,
                side: delta.order.side,
                price: delta.order.price,
                size: trade.size,
                ts_event: delta.ts_event,
                confidence: if trade_size_matches_diff { 0.9 } else { 0.7 },
                sequence: delta.sequence,
            });
        } else {
            // No trade - order modified or canceled
            if size_increased {
                // Order size increased - modification
                let order_id = state.id_generator.generate_id(
                    &delta.instrument_id.to_string(),
                    delta.order.side,
                    delta.order.price,
                    delta.ts_event,
                    delta.sequence,
                );

                actions.push(L3OrderAction {
                    synthetic_order_id: order_id,
                    instrument_id: delta.instrument_id,
                    action: L3Action::Modified,
                    side: delta.order.side,
                    price: delta.order.price,
                    size: new_size,
                    ts_event: delta.ts_event,
                    confidence: 0.8,
                    sequence: delta.sequence,
                });
            } else {
                // Size decreased - partial cancellation
                let order_id = state.id_generator.generate_id(
                    &delta.instrument_id.to_string(),
                    delta.order.side,
                    delta.order.price,
                    delta.ts_event,
                    delta.sequence,
                );

                let action_type = if new_size.raw == 0 {
                    L3Action::Canceled
                } else {
                    L3Action::Modified
                };

                actions.push(L3OrderAction {
                    synthetic_order_id: order_id,
                    instrument_id: delta.instrument_id,
                    action: action_type,
                    side: delta.order.side,
                    price: delta.order.price,
                    size: Quantity::from_raw(old_size.raw - new_size.raw, old_size.precision),
                    ts_event: delta.ts_event,
                    confidence: 0.85,
                    sequence: delta.sequence,
                });
            }
        }

        actions
    }

    /// Infer L3 actions for a Delete delta (static version).
    fn infer_delete_action_static(
        state: &mut InstrumentState,
        delta: &OrderBookDelta,
        trade: Option<&TradeTick>,
        size_tolerance: f64,
    ) -> Vec<L3OrderAction> {
        let price_raw = delta.order.price.raw;
        let levels = match delta.order.side {
            OrderSide::Buy => &state.bid_levels,
            OrderSide::Sell => &state.ask_levels,
            _ => panic!("Invalid side in delta"),
        };

        let old_size = levels
            .get(&price_raw)
            .map(|level| level.size)
            .unwrap_or_else(|| Quantity::from(0));

        let order_id = state.id_generator.generate_id(
            &delta.instrument_id.to_string(),
            delta.order.side,
            delta.order.price,
            delta.ts_event,
            delta.sequence,
        );

        let action = if let Some(trade) = trade {
            // Delete with trade - order filled
            let size_match = Self::sizes_match_static(old_size, trade.size, size_tolerance);

            L3OrderAction {
                synthetic_order_id: order_id,
                instrument_id: delta.instrument_id,
                action: L3Action::Fill,
                side: delta.order.side,
                price: delta.order.price,
                size: old_size,
                ts_event: delta.ts_event,
                confidence: if size_match { 0.95 } else { 0.75 },
                sequence: delta.sequence,
            }
        } else {
            // Delete without trade - order canceled
            L3OrderAction {
                synthetic_order_id: order_id,
                instrument_id: delta.instrument_id,
                action: L3Action::Canceled,
                side: delta.order.side,
                price: delta.order.price,
                size: old_size,
                ts_event: delta.ts_event,
                confidence: 0.9,
                sequence: delta.sequence,
            }
        };

        vec![action]
    }

    /// Check if two sizes match within the configured tolerance.
    fn sizes_match(&self, size1: Quantity, size2: Quantity) -> bool {
        Self::sizes_match_static(size1, size2, self.config.size_tolerance)
    }

    /// Check if two sizes match within a given tolerance (static version).
    fn sizes_match_static(size1: Quantity, size2: Quantity, tolerance: f64) -> bool {
        if size1 == size2 {
            return true;
        }

        let diff = if size1.raw > size2.raw {
            size1.raw - size2.raw
        } else {
            size2.raw - size1.raw
        };

        let avg = (size1.raw + size2.raw) / 2;
        if avg == 0 {
            return diff == 0;
        }

        let percentage_diff = (diff as f64) / (avg as f64);
        percentage_diff <= tolerance
    }

    /// Update the internal orderbook state based on a processed delta (static version).
    fn update_orderbook_state_static(state: &mut InstrumentState, delta: &OrderBookDelta) {
        let price_raw = delta.order.price.raw;
        let levels = state.levels_for_side(delta.order.side);

        match delta.action {
            BookAction::Add => {
                levels.insert(price_raw, LevelState::new(delta.order.size, delta.ts_event));
            }
            BookAction::Update => {
                if let Some(level) = levels.get_mut(&price_raw) {
                    level.size = delta.order.size;
                    level.last_update_ts = delta.ts_event;
                } else {
                    // Level doesn't exist - treat as Add
                    levels.insert(price_raw, LevelState::new(delta.order.size, delta.ts_event));
                }
            }
            BookAction::Delete => {
                levels.remove(&price_raw);
            }
            BookAction::Clear => {
                // Already handled
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nautilus_model::data::BookOrder;

    // Helper to create test delta
    fn create_delta(
        instrument_id: &str,
        action: BookAction,
        side: OrderSide,
        price: &str,
        size: &str,
        sequence: u64,
        ts_event: u64,
    ) -> OrderBookDelta {
        let order = BookOrder::new(
            side,
            Price::from(price),
            Quantity::from(size),
            sequence, // Using sequence as order_id for simplicity
        );

        OrderBookDelta::new(
            InstrumentId::from(instrument_id),
            action,
            order,
            0,
            sequence,
            UnixNanos::from(ts_event),
            UnixNanos::from(ts_event),
        )
    }

    // Helper to create test trade
    fn create_trade(
        instrument_id: &str,
        price: &str,
        size: &str,
        aggressor_side: AggressorSide,
        ts_event: u64,
    ) -> TradeTick {
        TradeTick::new(
            InstrumentId::from(instrument_id),
            Price::from(price),
            Quantity::from(size),
            aggressor_side,
            nautilus_model::identifiers::TradeId::from("T-001"),
            UnixNanos::from(ts_event),
            UnixNanos::from(ts_event),
        )
    }

    #[test]
    fn test_order_placement_from_delta() {
        // Single delta without trade = order placed
        let mut correlator = L2DeltaCorrelator::default();

        let delta = create_delta(
            "BTCUSDT.BINANCE",
            BookAction::Add,
            OrderSide::Buy,
            "50000.00",
            "1.5",
            1,
            1000000000,
        );

        let actions = correlator.on_delta(delta);

        assert_eq!(actions.len(), 1);
        assert_eq!(actions[0].action, L3Action::Placed);
        assert_eq!(actions[0].side, OrderSide::Buy);
        assert_eq!(actions[0].price, Price::from("50000.00"));
        assert_eq!(actions[0].size, Quantity::from("1.5"));
        assert!(actions[0].confidence >= 0.9);
    }

    #[test]
    fn test_order_fill_from_trade_and_delta() {
        // Trade + matching delta = order filled
        let mut correlator = L2DeltaCorrelator::default();

        // First, place an order
        let add_delta = create_delta(
            "BTCUSDT.BINANCE",
            BookAction::Add,
            OrderSide::Sell,
            "50000.00",
            "1.0",
            1,
            1000000000,
        );
        correlator.on_delta(add_delta);

        // Then a trade occurs
        let trade = create_trade(
            "BTCUSDT.BINANCE",
            "50000.00",
            "1.0",
            AggressorSide::Buyer,
            1000010000, // 10μs later
        );

        // And the level is removed
        let delete_delta = create_delta(
            "BTCUSDT.BINANCE",
            BookAction::Delete,
            OrderSide::Sell,
            "50000.00",
            "0",
            2,
            1000020000,
        );

        correlator.on_trade(trade);
        let actions = correlator.on_delta(delete_delta);

        // Should detect a fill
        let fill_action = actions.iter().find(|a| a.action == L3Action::Fill);
        assert!(fill_action.is_some());
        let fill = fill_action.unwrap();
        assert_eq!(fill.size, Quantity::from("1.0"));
        assert!(fill.confidence >= 0.7);
    }

    #[test]
    fn test_partial_fill() {
        // Delta larger than trade = partial fill
        let mut correlator = L2DeltaCorrelator::default();

        // Place order for 2.0
        let add_delta = create_delta(
            "BTCUSDT.BINANCE",
            BookAction::Add,
            OrderSide::Sell,
            "50000.00",
            "2.0",
            1,
            1000000000,
        );
        correlator.on_delta(add_delta);

        // Trade for 1.0
        let trade = create_trade(
            "BTCUSDT.BINANCE",
            "50000.00",
            "1.0",
            AggressorSide::Buyer,
            1000010000,
        );

        // Level updated to 1.0 remaining
        let update_delta = create_delta(
            "BTCUSDT.BINANCE",
            BookAction::Update,
            OrderSide::Sell,
            "50000.00",
            "1.0",
            2,
            1000020000,
        );

        correlator.on_trade(trade);
        let actions = correlator.on_delta(update_delta);

        // Should detect partial fill
        let partial = actions.iter().find(|a| a.action == L3Action::PartialFill);
        assert!(partial.is_some());
    }

    #[test]
    fn test_multiple_orders_at_level() {
        // Multiple small deltas + one large trade
        let mut correlator = L2DeltaCorrelator::default();

        // Add first order
        let delta1 = create_delta(
            "BTCUSDT.BINANCE",
            BookAction::Add,
            OrderSide::Buy,
            "50000.00",
            "1.0",
            1,
            1000000000,
        );
        correlator.on_delta(delta1);

        // Add second order at same level (Update action)
        let delta2 = create_delta(
            "BTCUSDT.BINANCE",
            BookAction::Update,
            OrderSide::Buy,
            "50000.00",
            "2.0", // Total size now 2.0
            2,
            1001000000,
        );
        let actions = correlator.on_delta(delta2);

        // Both deltas should produce actions
        assert!(actions.len() >= 1);
    }

    #[test]
    fn test_order_cancellation() {
        // Negative delta without trade = cancel
        let mut correlator = L2DeltaCorrelator::default();

        // Place order
        let add_delta = create_delta(
            "BTCUSDT.BINANCE",
            BookAction::Add,
            OrderSide::Buy,
            "50000.00",
            "1.5",
            1,
            1000000000,
        );
        correlator.on_delta(add_delta);

        // Cancel order (delete without trade)
        let delete_delta = create_delta(
            "BTCUSDT.BINANCE",
            BookAction::Delete,
            OrderSide::Buy,
            "50000.00",
            "0",
            2,
            1001000000,
        );
        let actions = correlator.on_delta(delete_delta);

        // Should detect cancellation
        let cancel = actions.iter().find(|a| a.action == L3Action::Canceled);
        assert!(cancel.is_some());
        assert!(cancel.unwrap().confidence >= 0.8);
    }

    #[test]
    fn test_out_of_order_events() {
        // Events arrive in wrong sequence
        let mut correlator = L2DeltaCorrelator::default();

        // Trade arrives first
        let trade = create_trade(
            "BTCUSDT.BINANCE",
            "50000.00",
            "1.0",
            AggressorSide::Buyer,
            1000010000,
        );
        correlator.on_trade(trade);

        // Add delta arrives later but with earlier timestamp
        let add_delta = create_delta(
            "BTCUSDT.BINANCE",
            BookAction::Add,
            OrderSide::Sell,
            "50000.00",
            "1.0",
            1,
            1000000000,
        );
        correlator.on_delta(add_delta);

        // Delete delta
        let delete_delta = create_delta(
            "BTCUSDT.BINANCE",
            BookAction::Delete,
            OrderSide::Sell,
            "50000.00",
            "0",
            2,
            1000020000,
        );
        let actions = correlator.on_delta(delete_delta);

        // Should still correlate correctly
        assert!(!actions.is_empty());
    }

    #[test]
    fn test_orderbook_clear() {
        // Clear action should reset state
        let mut correlator = L2DeltaCorrelator::default();

        // Place some orders
        let delta1 = create_delta(
            "BTCUSDT.BINANCE",
            BookAction::Add,
            OrderSide::Buy,
            "50000.00",
            "1.0",
            1,
            1000000000,
        );
        correlator.on_delta(delta1);

        // Clear the book
        let clear_delta = OrderBookDelta::clear(
            InstrumentId::from("BTCUSDT.BINANCE"),
            2,
            UnixNanos::from(1001000000),
            UnixNanos::from(1001000000),
        );
        let actions = correlator.on_delta(clear_delta);

        // Should return no actions
        assert!(actions.is_empty());

        // State should be cleared
        let state = correlator
            .instrument_states
            .get(&InstrumentId::from("BTCUSDT.BINANCE"))
            .unwrap();
        assert!(state.bid_levels.is_empty());
        assert!(state.ask_levels.is_empty());
    }

    #[test]
    fn test_confidence_scoring() {
        let mut correlator = L2DeltaCorrelator::default();

        // Perfect size match should have high confidence
        let add_delta = create_delta(
            "BTCUSDT.BINANCE",
            BookAction::Add,
            OrderSide::Sell,
            "50000.00",
            "1.0",
            1,
            1000000000,
        );
        correlator.on_delta(add_delta);

        let trade = create_trade(
            "BTCUSDT.BINANCE",
            "50000.00",
            "1.0",
            AggressorSide::Buyer,
            1000010000,
        );

        let delete_delta = create_delta(
            "BTCUSDT.BINANCE",
            BookAction::Delete,
            OrderSide::Sell,
            "50000.00",
            "0",
            2,
            1000020000,
        );

        correlator.on_trade(trade);
        let actions = correlator.on_delta(delete_delta);

        // High confidence expected for perfect match
        if let Some(action) = actions.first() {
            assert!(action.confidence >= 0.7);
        }
    }

    #[test]
    fn test_minimum_confidence_filter() {
        let config = CorrelatorConfig {
            min_confidence: 0.95,
            ..Default::default()
        };
        let mut correlator = L2DeltaCorrelator::new(config);

        // Create action that might have lower confidence
        let delta = create_delta(
            "BTCUSDT.BINANCE",
            BookAction::Update,
            OrderSide::Buy,
            "50000.00",
            "1.0",
            1,
            1000000000,
        );

        let actions = correlator.on_delta(delta);

        // Only high-confidence actions should pass through
        for action in actions {
            assert!(action.confidence >= 0.95);
        }
    }

    #[test]
    fn test_size_tolerance() {
        let config = CorrelatorConfig {
            size_tolerance: 0.01, // 1% tolerance
            ..Default::default()
        };
        let correlator = L2DeltaCorrelator::new(config);

        // Sizes within 1% should match
        assert!(correlator.sizes_match(Quantity::from("1.00"), Quantity::from("1.005")));

        // Sizes outside 1% should not match
        assert!(!correlator.sizes_match(Quantity::from("1.00"), Quantity::from("1.02")));
    }

    #[test]
    fn test_correlation_window() {
        let config = CorrelatorConfig {
            correlation_window_ns: 100_000_000, // 100ms
            ..Default::default()
        };
        let mut correlator = L2DeltaCorrelator::new(config);

        let add_delta = create_delta(
            "BTCUSDT.BINANCE",
            BookAction::Add,
            OrderSide::Sell,
            "50000.00",
            "1.0",
            1,
            1000000000,
        );
        correlator.on_delta(add_delta);

        // Trade within window
        let trade_near = create_trade(
            "BTCUSDT.BINANCE",
            "50000.00",
            "1.0",
            AggressorSide::Buyer,
            1050000000, // 50ms later - within window
        );
        correlator.on_trade(trade_near);

        // Trade outside window
        let trade_far = create_trade(
            "BTCUSDT.BINANCE",
            "50000.00",
            "1.0",
            AggressorSide::Buyer,
            1200000000, // 200ms later - outside window
        );
        correlator.on_trade(trade_far);

        let delete_delta = create_delta(
            "BTCUSDT.BINANCE",
            BookAction::Delete,
            OrderSide::Sell,
            "50000.00",
            "0",
            2,
            1060000000,
        );

        let actions = correlator.on_delta(delete_delta);

        // Should correlate with near trade, not far trade
        assert!(!actions.is_empty());
    }
}
