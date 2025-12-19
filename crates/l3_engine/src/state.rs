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

//! Local orderbook state manager for L3 reconstruction.
//!
//! Maintains a local representation of the orderbook state with synthetic L3 orders,
//! enabling correlation of L2 delta changes to specific orders.
//!
//! # Design
//!
//! The state manager tracks synthetic orders at each price level, organizing them by:
//! - Price levels (using BTreeMap for ordered iteration)
//! - Order entries at each level
//! - Order index for fast lookup by synthetic ID
//!
//! This enables efficient tracking of order lifecycle events (add, update, delete) and
//! quick queries for orders at specific price levels.

use std::collections::{BTreeMap, HashMap};

use nautilus_core::nanos::UnixNanos;
use nautilus_model::{
    data::OrderBookDelta,
    enums::{BookAction, OrderSide},
    identifiers::{InstrumentId, VenueOrderId},
    types::{Price, Quantity},
};

use crate::correlator::L3OrderAction;

/// Represents a single synthetic order in the orderbook.
#[derive(Debug, Clone, PartialEq)]
pub struct OrderEntry {
    /// Synthetic order ID
    pub order_id: VenueOrderId,
    /// Price level
    pub price: Price,
    /// Current quantity
    pub quantity: Quantity,
    /// Order side
    pub side: OrderSide,
    /// Creation timestamp
    pub ts_created: UnixNanos,
    /// Last update timestamp
    pub ts_updated: UnixNanos,
    /// Sequence number of last update
    pub sequence: u64,
}

impl OrderEntry {
    /// Creates a new order entry.
    #[must_use]
    pub fn new(
        order_id: VenueOrderId,
        price: Price,
        quantity: Quantity,
        side: OrderSide,
        ts_event: UnixNanos,
        sequence: u64,
    ) -> Self {
        Self {
            order_id,
            price,
            quantity,
            side,
            ts_created: ts_event,
            ts_updated: ts_event,
            sequence,
        }
    }

    /// Updates the order's quantity and timestamp.
    pub fn update(&mut self, quantity: Quantity, ts_event: UnixNanos, sequence: u64) {
        self.quantity = quantity;
        self.ts_updated = ts_event;
        self.sequence = sequence;
    }
}

/// Maintains local orderbook state with synthetic L3 orders.
///
/// # State Organization
///
/// The state manager uses a multi-level indexing strategy:
/// - Price levels: BTreeMap<i64, Vec<OrderEntry>> for ordered price iteration
/// - Order index: HashMap<VenueOrderId, OrderEntry> for fast lookups
///
/// # Thread Safety
///
/// This structure is not thread-safe. External synchronization is required for
/// concurrent access.
pub struct OrderbookState {
    /// Instrument identifier
    instrument_id: InstrumentId,
    /// Bid levels (price → orders) - sorted descending
    bid_levels: BTreeMap<i64, Vec<OrderEntry>>,
    /// Ask levels (price → orders) - sorted ascending
    ask_levels: BTreeMap<i64, Vec<OrderEntry>>,
    /// Fast lookup index (order_id → order)
    order_index: HashMap<VenueOrderId, OrderEntry>,
    /// Sequence number of last processed delta
    sequence: u64,
    /// Last update timestamp
    ts_last_update: UnixNanos,
}

impl OrderbookState {
    /// Creates a new orderbook state manager for an instrument.
    #[must_use]
    pub fn new(instrument_id: InstrumentId) -> Self {
        Self {
            instrument_id,
            bid_levels: BTreeMap::new(),
            ask_levels: BTreeMap::new(),
            order_index: HashMap::new(),
            sequence: 0,
            ts_last_update: UnixNanos::default(),
        }
    }

    /// Returns the instrument ID.
    #[must_use]
    pub fn instrument_id(&self) -> InstrumentId {
        self.instrument_id
    }

    /// Returns the current sequence number.
    #[must_use]
    pub fn sequence(&self) -> u64 {
        self.sequence
    }

    /// Returns the timestamp of the last update.
    #[must_use]
    pub fn ts_last_update(&self) -> UnixNanos {
        self.ts_last_update
    }

    /// Returns the total number of synthetic orders.
    #[must_use]
    pub fn order_count(&self) -> usize {
        self.order_index.len()
    }

    /// Returns the number of bid levels.
    #[must_use]
    pub fn bid_level_count(&self) -> usize {
        self.bid_levels.len()
    }

    /// Returns the number of ask levels.
    #[must_use]
    pub fn ask_level_count(&self) -> usize {
        self.ask_levels.len()
    }

    /// Gets orders at a specific price level.
    #[must_use]
    pub fn get_orders_at_price(&self, price: Price, side: OrderSide) -> &[OrderEntry] {
        let levels = match side {
            OrderSide::Buy => &self.bid_levels,
            OrderSide::Sell => &self.ask_levels,
            _ => return &[],
        };

        levels.get(&price.raw).map(|v| v.as_slice()).unwrap_or(&[])
    }

    /// Gets an order by its synthetic ID.
    #[must_use]
    pub fn get_order(&self, order_id: &VenueOrderId) -> Option<&OrderEntry> {
        self.order_index.get(order_id)
    }

    /// Applies an L3 order action to the state.
    ///
    /// This method updates the internal state based on the action type:
    /// - Placed: Adds new order to book
    /// - Modified: Updates existing order quantity
    /// - PartialFill/Fill: Reduces or removes order
    /// - Canceled: Removes order
    pub fn apply_action(&mut self, action: &L3OrderAction) {
        self.sequence = action.sequence;
        self.ts_last_update = action.ts_event;

        match action.action {
            crate::correlator::L3Action::Placed => {
                self.add_order(action);
            }
            crate::correlator::L3Action::Modified => {
                self.modify_order(action);
            }
            crate::correlator::L3Action::PartialFill => {
                self.partial_fill_order(action);
            }
            crate::correlator::L3Action::Fill => {
                self.remove_order(action);
            }
            crate::correlator::L3Action::Canceled => {
                self.remove_order(action);
            }
        }
    }

    /// Applies an L2 orderbook delta to update the state.
    ///
    /// This method is used when processing raw L2 deltas that haven't been
    /// correlated to L3 actions yet.
    pub fn apply_delta(&mut self, delta: &OrderBookDelta) {
        self.sequence = delta.sequence;
        self.ts_last_update = delta.ts_event;

        match delta.action {
            BookAction::Clear => {
                self.clear();
            }
            BookAction::Add | BookAction::Update | BookAction::Delete => {
                // For now, we don't apply raw deltas directly
                // The correlator should convert these to L3 actions first
            }
        }
    }

    /// Clears all orders from the orderbook.
    pub fn clear(&mut self) {
        self.bid_levels.clear();
        self.ask_levels.clear();
        self.order_index.clear();
    }

    /// Returns a snapshot of all orders as OrderEntry vector.
    #[must_use]
    pub fn snapshot(&self) -> Vec<OrderEntry> {
        self.order_index.values().cloned().collect()
    }

    /// Returns all bid levels in descending price order.
    #[must_use]
    pub fn bid_levels(&self) -> Vec<(Price, &[OrderEntry])> {
        self.bid_levels
            .iter()
            .rev() // Descending order
            .map(|(price_raw, orders)| {
                let price = Price::from_raw(*price_raw, orders[0].price.precision);
                (price, orders.as_slice())
            })
            .collect()
    }

    /// Returns all ask levels in ascending price order.
    #[must_use]
    pub fn ask_levels(&self) -> Vec<(Price, &[OrderEntry])> {
        self.ask_levels
            .iter() // Already ascending
            .map(|(price_raw, orders)| {
                let price = Price::from_raw(*price_raw, orders[0].price.precision);
                (price, orders.as_slice())
            })
            .collect()
    }

    // Internal methods for state manipulation

    fn add_order(&mut self, action: &L3OrderAction) {
        let entry = OrderEntry::new(
            action.synthetic_order_id,
            action.price,
            action.size,
            action.side,
            action.ts_event,
            action.sequence,
        );

        // Add to level
        let levels = self.levels_for_side_mut(action.side);
        levels
            .entry(action.price.raw)
            .or_insert_with(Vec::new)
            .push(entry.clone());

        // Add to index
        self.order_index.insert(action.synthetic_order_id, entry);
    }

    fn modify_order(&mut self, action: &L3OrderAction) {
        // Update in index
        if let Some(order) = self.order_index.get_mut(&action.synthetic_order_id) {
            order.update(action.size, action.ts_event, action.sequence);

            // Update in level
            let levels = self.levels_for_side_mut(action.side);
            if let Some(level_orders) = levels.get_mut(&action.price.raw) {
                if let Some(order_in_level) = level_orders
                    .iter_mut()
                    .find(|o| o.order_id == action.synthetic_order_id)
                {
                    order_in_level.update(action.size, action.ts_event, action.sequence);
                }
            }
        }
    }

    fn partial_fill_order(&mut self, action: &L3OrderAction) {
        // Similar to modify - reduce quantity
        if let Some(order) = self.order_index.get_mut(&action.synthetic_order_id) {
            // Reduce quantity by fill amount
            let new_quantity = Quantity::from_raw(
                order.quantity.raw.saturating_sub(action.size.raw),
                order.quantity.precision,
            );

            if new_quantity.raw == 0 {
                // Fully filled - remove
                self.remove_order(action);
            } else {
                order.update(new_quantity, action.ts_event, action.sequence);

                // Update in level
                let levels = self.levels_for_side_mut(action.side);
                if let Some(level_orders) = levels.get_mut(&action.price.raw) {
                    if let Some(order_in_level) = level_orders
                        .iter_mut()
                        .find(|o| o.order_id == action.synthetic_order_id)
                    {
                        order_in_level.update(new_quantity, action.ts_event, action.sequence);
                    }
                }
            }
        }
    }

    fn remove_order(&mut self, action: &L3OrderAction) {
        // Remove from index
        self.order_index.remove(&action.synthetic_order_id);

        // Remove from level
        let levels = self.levels_for_side_mut(action.side);
        if let Some(level_orders) = levels.get_mut(&action.price.raw) {
            level_orders.retain(|o| o.order_id != action.synthetic_order_id);

            // Remove level if empty
            if level_orders.is_empty() {
                levels.remove(&action.price.raw);
            }
        }
    }

    fn levels_for_side_mut(&mut self, side: OrderSide) -> &mut BTreeMap<i64, Vec<OrderEntry>> {
        match side {
            OrderSide::Buy => &mut self.bid_levels,
            OrderSide::Sell => &mut self.ask_levels,
            _ => panic!("Invalid order side"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::correlator::L3Action;

    fn create_test_action(
        action_type: L3Action,
        side: OrderSide,
        price: &str,
        size: &str,
        order_id: &str,
        sequence: u64,
    ) -> L3OrderAction {
        L3OrderAction {
            synthetic_order_id: VenueOrderId::new(order_id),
            instrument_id: InstrumentId::from("BTCUSDT.BINANCE"),
            action: action_type,
            side,
            price: Price::from(price),
            size: Quantity::from(size),
            ts_event: UnixNanos::from(1000000000),
            confidence: 1.0,
            sequence,
        }
    }

    #[test]
    fn test_new_orderbook_state() {
        let state = OrderbookState::new(InstrumentId::from("BTCUSDT.BINANCE"));
        assert_eq!(state.order_count(), 0);
        assert_eq!(state.bid_level_count(), 0);
        assert_eq!(state.ask_level_count(), 0);
    }

    #[test]
    fn test_add_order() {
        let mut state = OrderbookState::new(InstrumentId::from("BTCUSDT.BINANCE"));

        let action = create_test_action(
            L3Action::Placed,
            OrderSide::Buy,
            "50000.00",
            "1.5",
            "order-1",
            1,
        );

        state.apply_action(&action);

        assert_eq!(state.order_count(), 1);
        assert_eq!(state.bid_level_count(), 1);

        let orders = state.get_orders_at_price(Price::from("50000.00"), OrderSide::Buy);
        assert_eq!(orders.len(), 1);
        assert_eq!(orders[0].quantity, Quantity::from("1.5"));
    }

    #[test]
    fn test_multiple_orders_at_level() {
        let mut state = OrderbookState::new(InstrumentId::from("BTCUSDT.BINANCE"));

        // Add two orders at same price
        let action1 = create_test_action(
            L3Action::Placed,
            OrderSide::Buy,
            "50000.00",
            "1.0",
            "order-1",
            1,
        );
        let action2 = create_test_action(
            L3Action::Placed,
            OrderSide::Buy,
            "50000.00",
            "2.0",
            "order-2",
            2,
        );

        state.apply_action(&action1);
        state.apply_action(&action2);

        assert_eq!(state.order_count(), 2);
        assert_eq!(state.bid_level_count(), 1);

        let orders = state.get_orders_at_price(Price::from("50000.00"), OrderSide::Buy);
        assert_eq!(orders.len(), 2);
    }

    #[test]
    fn test_modify_order() {
        let mut state = OrderbookState::new(InstrumentId::from("BTCUSDT.BINANCE"));

        let place_action = create_test_action(
            L3Action::Placed,
            OrderSide::Buy,
            "50000.00",
            "1.5",
            "order-1",
            1,
        );
        state.apply_action(&place_action);

        let modify_action = create_test_action(
            L3Action::Modified,
            OrderSide::Buy,
            "50000.00",
            "2.5",
            "order-1",
            2,
        );
        state.apply_action(&modify_action);

        let orders = state.get_orders_at_price(Price::from("50000.00"), OrderSide::Buy);
        assert_eq!(orders[0].quantity, Quantity::from("2.5"));
    }

    #[test]
    fn test_remove_order() {
        let mut state = OrderbookState::new(InstrumentId::from("BTCUSDT.BINANCE"));

        let place_action = create_test_action(
            L3Action::Placed,
            OrderSide::Buy,
            "50000.00",
            "1.5",
            "order-1",
            1,
        );
        state.apply_action(&place_action);

        let cancel_action = create_test_action(
            L3Action::Canceled,
            OrderSide::Buy,
            "50000.00",
            "1.5",
            "order-1",
            2,
        );
        state.apply_action(&cancel_action);

        assert_eq!(state.order_count(), 0);
        assert_eq!(state.bid_level_count(), 0);
    }

    #[test]
    fn test_partial_fill() {
        let mut state = OrderbookState::new(InstrumentId::from("BTCUSDT.BINANCE"));

        let place_action = create_test_action(
            L3Action::Placed,
            OrderSide::Sell,
            "50000.00",
            "2.0",
            "order-1",
            1,
        );
        state.apply_action(&place_action);

        let fill_action = create_test_action(
            L3Action::PartialFill,
            OrderSide::Sell,
            "50000.00",
            "0.5",
            "order-1",
            2,
        );
        state.apply_action(&fill_action);

        let orders = state.get_orders_at_price(Price::from("50000.00"), OrderSide::Sell);
        assert_eq!(orders.len(), 1);
        assert_eq!(orders[0].quantity, Quantity::from("1.5"));
    }

    #[test]
    fn test_clear() {
        let mut state = OrderbookState::new(InstrumentId::from("BTCUSDT.BINANCE"));

        state.apply_action(&create_test_action(
            L3Action::Placed,
            OrderSide::Buy,
            "50000.00",
            "1.0",
            "order-1",
            1,
        ));
        state.apply_action(&create_test_action(
            L3Action::Placed,
            OrderSide::Sell,
            "51000.00",
            "1.0",
            "order-2",
            2,
        ));

        assert_eq!(state.order_count(), 2);

        state.clear();

        assert_eq!(state.order_count(), 0);
        assert_eq!(state.bid_level_count(), 0);
        assert_eq!(state.ask_level_count(), 0);
    }

    #[test]
    fn test_snapshot() {
        let mut state = OrderbookState::new(InstrumentId::from("BTCUSDT.BINANCE"));

        state.apply_action(&create_test_action(
            L3Action::Placed,
            OrderSide::Buy,
            "50000.00",
            "1.0",
            "order-1",
            1,
        ));
        state.apply_action(&create_test_action(
            L3Action::Placed,
            OrderSide::Sell,
            "51000.00",
            "1.5",
            "order-2",
            2,
        ));

        let snapshot = state.snapshot();
        assert_eq!(snapshot.len(), 2);
    }

    #[test]
    fn test_bid_ask_levels_ordering() {
        let mut state = OrderbookState::new(InstrumentId::from("BTCUSDT.BINANCE"));

        // Add bids (should be descending)
        state.apply_action(&create_test_action(
            L3Action::Placed,
            OrderSide::Buy,
            "50000.00",
            "1.0",
            "order-1",
            1,
        ));
        state.apply_action(&create_test_action(
            L3Action::Placed,
            OrderSide::Buy,
            "50001.00",
            "1.0",
            "order-2",
            2,
        ));
        state.apply_action(&create_test_action(
            L3Action::Placed,
            OrderSide::Buy,
            "49999.00",
            "1.0",
            "order-3",
            3,
        ));

        let bid_levels = state.bid_levels();
        assert_eq!(bid_levels.len(), 3);
        assert!(bid_levels[0].0 > bid_levels[1].0);
        assert!(bid_levels[1].0 > bid_levels[2].0);

        // Add asks (should be ascending)
        state.apply_action(&create_test_action(
            L3Action::Placed,
            OrderSide::Sell,
            "51000.00",
            "1.0",
            "order-4",
            4,
        ));
        state.apply_action(&create_test_action(
            L3Action::Placed,
            OrderSide::Sell,
            "51001.00",
            "1.0",
            "order-5",
            5,
        ));

        let ask_levels = state.ask_levels();
        assert_eq!(ask_levels.len(), 2);
        assert!(ask_levels[0].0 < ask_levels[1].0);
    }
}
