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

//! L3 delta event emitter.
//!
//! Produces L3 orderbook delta events from correlation results, translating
//! synthetic order changes into standardized L3 delta events.
//!
//! # Event Format
//!
//! The emitter converts L3 order actions into `OrderBookDelta` events with:
//! - `BookType::L3_MBO` to indicate market-by-order data
//! - Synthetic order IDs embedded in the delta
//! - Proper sequence numbering
//! - Appropriate `BookAction` types (Add, Update, Delete)
//!
//! # Usage
//!
//! The emitter is typically used within the L3 reconstruction engine to convert
//! inferred order actions into events that can be consumed by orderbook processors.

use nautilus_core::nanos::UnixNanos;
use nautilus_model::{
    data::{BookOrder, OrderBookDelta},
    enums::{BookAction, RecordFlag},
    identifiers::InstrumentId,
};

use crate::{
    correlator::{L3Action, L3OrderAction},
    state::OrderEntry,
};

/// Emits L3 orderbook delta events from synthetic order actions.
///
/// # Design
///
/// The emitter maintains a sequence counter and converts L3 actions into
/// standardized `OrderBookDelta` events. It maps L3 action types to
/// appropriate book actions:
///
/// - `Placed` → `BookAction::Add`
/// - `Modified` → `BookAction::Update`
/// - `PartialFill` → `BookAction::Update` (reduced size)
/// - `Fill` → `BookAction::Delete`
/// - `Canceled` → `BookAction::Delete`
pub struct L3DeltaEmitter {
    /// Instrument identifier
    instrument_id: InstrumentId,
    /// Sequence counter for emitted deltas
    sequence: u64,
}

impl L3DeltaEmitter {
    /// Creates a new L3 delta emitter for an instrument.
    #[must_use]
    pub fn new(instrument_id: InstrumentId) -> Self {
        Self {
            instrument_id,
            sequence: 0,
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

    /// Emits an L3 delta from an order action.
    ///
    /// Converts the L3 action into an appropriate `OrderBookDelta` with
    /// correct action type and sequence numbering.
    pub fn emit(&mut self, action: &L3OrderAction, ts_init: UnixNanos) -> OrderBookDelta {
        self.sequence += 1;

        let book_action = Self::action_to_book_action(action.action);
        let order = Self::create_book_order(action);
        let flags = RecordFlag::F_LAST as u8; // Mark as last in batch

        OrderBookDelta::new(
            self.instrument_id,
            book_action,
            order,
            flags,
            self.sequence,
            action.ts_event,
            ts_init,
        )
    }

    /// Emits an Add delta for a new order placement.
    pub fn emit_add(
        &mut self,
        entry: &OrderEntry,
        ts_event: UnixNanos,
        ts_init: UnixNanos,
    ) -> OrderBookDelta {
        self.sequence += 1;

        let order = BookOrder::new(
            entry.side,
            entry.price,
            entry.quantity,
            Self::venue_order_id_to_u64(&entry.order_id),
        );

        OrderBookDelta::new(
            self.instrument_id,
            BookAction::Add,
            order,
            RecordFlag::F_LAST as u8,
            self.sequence,
            ts_event,
            ts_init,
        )
    }

    /// Emits an Update delta for an order modification.
    pub fn emit_update(
        &mut self,
        entry: &OrderEntry,
        ts_event: UnixNanos,
        ts_init: UnixNanos,
    ) -> OrderBookDelta {
        self.sequence += 1;

        let order = BookOrder::new(
            entry.side,
            entry.price,
            entry.quantity,
            Self::venue_order_id_to_u64(&entry.order_id),
        );

        OrderBookDelta::new(
            self.instrument_id,
            BookAction::Update,
            order,
            RecordFlag::F_LAST as u8,
            self.sequence,
            ts_event,
            ts_init,
        )
    }

    /// Emits a Delete delta for an order removal.
    pub fn emit_delete(
        &mut self,
        entry: &OrderEntry,
        ts_event: UnixNanos,
        ts_init: UnixNanos,
    ) -> OrderBookDelta {
        self.sequence += 1;

        let order = BookOrder::new(
            entry.side,
            entry.price,
            entry.quantity,
            Self::venue_order_id_to_u64(&entry.order_id),
        );

        OrderBookDelta::new(
            self.instrument_id,
            BookAction::Delete,
            order,
            RecordFlag::F_LAST as u8,
            self.sequence,
            ts_event,
            ts_init,
        )
    }

    /// Emits a snapshot of the current orderbook state.
    ///
    /// Creates a series of Add deltas representing all orders in the book.
    /// The first delta is marked with `F_SNAPSHOT` flag.
    pub fn emit_snapshot(
        &mut self,
        entries: &[OrderEntry],
        ts_event: UnixNanos,
        ts_init: UnixNanos,
    ) -> Vec<OrderBookDelta> {
        if entries.is_empty() {
            return Vec::new();
        }

        let mut deltas = Vec::with_capacity(entries.len());

        for (idx, entry) in entries.iter().enumerate() {
            self.sequence += 1;

            let order = BookOrder::new(
                entry.side,
                entry.price,
                entry.quantity,
                Self::venue_order_id_to_u64(&entry.order_id),
            );

            let flags = if idx == 0 {
                RecordFlag::F_SNAPSHOT as u8 | RecordFlag::F_LAST as u8
            } else if idx == entries.len() - 1 {
                RecordFlag::F_LAST as u8
            } else {
                0
            };

            deltas.push(OrderBookDelta::new(
                self.instrument_id,
                BookAction::Add,
                order,
                flags,
                self.sequence,
                ts_event,
                ts_init,
            ));
        }

        deltas
    }

    /// Emits a Clear delta to reset the orderbook.
    pub fn emit_clear(&mut self, ts_event: UnixNanos, ts_init: UnixNanos) -> OrderBookDelta {
        self.sequence += 1;

        OrderBookDelta::clear(self.instrument_id, self.sequence, ts_event, ts_init)
    }

    // Internal helper methods

    fn action_to_book_action(action: L3Action) -> BookAction {
        match action {
            L3Action::Placed => BookAction::Add,
            L3Action::Modified => BookAction::Update,
            L3Action::PartialFill => BookAction::Update,
            L3Action::Fill => BookAction::Delete,
            L3Action::Canceled => BookAction::Delete,
        }
    }

    fn create_book_order(action: &L3OrderAction) -> BookOrder {
        BookOrder::new(
            action.side,
            action.price,
            action.size,
            Self::venue_order_id_to_u64(&action.synthetic_order_id),
        )
    }

    fn venue_order_id_to_u64(venue_order_id: &nautilus_model::identifiers::VenueOrderId) -> u64 {
        // Extract a numeric hash from the venue order ID string
        // For L3 synthetic IDs like "L3-BTCUSDT-1234567890abcdef", we extract the hash part
        let id_str = venue_order_id.to_string();

        // Try to parse the hash part if it's in our L3 format
        if let Some(hash_part) = id_str.split('-').nth(2) {
            if let Ok(hash) = u64::from_str_radix(hash_part, 16) {
                return hash;
            }
        }

        // Fallback: compute a hash of the entire string
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        id_str.hash(&mut hasher);
        hasher.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nautilus_model::{
        enums::OrderSide,
        identifiers::{InstrumentId, VenueOrderId},
        types::{Price, Quantity},
    };

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

    fn create_test_entry(
        order_id: &str,
        side: OrderSide,
        price: &str,
        quantity: &str,
        sequence: u64,
    ) -> OrderEntry {
        OrderEntry::new(
            VenueOrderId::new(order_id),
            Price::from(price),
            Quantity::from(quantity),
            side,
            UnixNanos::from(1000000000),
            sequence,
        )
    }

    #[test]
    fn test_new_emitter() {
        let emitter = L3DeltaEmitter::new(InstrumentId::from("BTCUSDT.BINANCE"));
        assert_eq!(emitter.sequence(), 0);
        assert_eq!(
            emitter.instrument_id(),
            InstrumentId::from("BTCUSDT.BINANCE")
        );
    }

    #[test]
    fn test_emit_placed_action() {
        let mut emitter = L3DeltaEmitter::new(InstrumentId::from("BTCUSDT.BINANCE"));

        let action = create_test_action(
            L3Action::Placed,
            OrderSide::Buy,
            "50000.00",
            "1.5",
            "L3-BTCUSDT-0000000000000001",
            1,
        );

        let delta = emitter.emit(&action, UnixNanos::from(1000000000));

        assert_eq!(delta.action, BookAction::Add);
        assert_eq!(delta.order.side, OrderSide::Buy);
        assert_eq!(delta.order.price, Price::from("50000.00"));
        assert_eq!(delta.order.size, Quantity::from("1.5"));
        assert_eq!(emitter.sequence(), 1);
    }

    #[test]
    fn test_emit_modified_action() {
        let mut emitter = L3DeltaEmitter::new(InstrumentId::from("BTCUSDT.BINANCE"));

        let action = create_test_action(
            L3Action::Modified,
            OrderSide::Sell,
            "51000.00",
            "2.0",
            "L3-BTCUSDT-0000000000000002",
            2,
        );

        let delta = emitter.emit(&action, UnixNanos::from(1000000000));

        assert_eq!(delta.action, BookAction::Update);
        assert_eq!(emitter.sequence(), 1);
    }

    #[test]
    fn test_emit_canceled_action() {
        let mut emitter = L3DeltaEmitter::new(InstrumentId::from("BTCUSDT.BINANCE"));

        let action = create_test_action(
            L3Action::Canceled,
            OrderSide::Buy,
            "50000.00",
            "1.0",
            "L3-BTCUSDT-0000000000000003",
            3,
        );

        let delta = emitter.emit(&action, UnixNanos::from(1000000000));

        assert_eq!(delta.action, BookAction::Delete);
    }

    #[test]
    fn test_emit_add() {
        let mut emitter = L3DeltaEmitter::new(InstrumentId::from("BTCUSDT.BINANCE"));

        let entry = create_test_entry(
            "L3-BTCUSDT-0000000000000001",
            OrderSide::Buy,
            "50000.00",
            "1.5",
            1,
        );

        let delta = emitter.emit_add(
            &entry,
            UnixNanos::from(1000000000),
            UnixNanos::from(1000000000),
        );

        assert_eq!(delta.action, BookAction::Add);
        assert_eq!(delta.order.price, Price::from("50000.00"));
    }

    #[test]
    fn test_emit_update() {
        let mut emitter = L3DeltaEmitter::new(InstrumentId::from("BTCUSDT.BINANCE"));

        let entry = create_test_entry(
            "L3-BTCUSDT-0000000000000001",
            OrderSide::Buy,
            "50000.00",
            "2.5",
            2,
        );

        let delta = emitter.emit_update(
            &entry,
            UnixNanos::from(1000000000),
            UnixNanos::from(1000000000),
        );

        assert_eq!(delta.action, BookAction::Update);
    }

    #[test]
    fn test_emit_delete() {
        let mut emitter = L3DeltaEmitter::new(InstrumentId::from("BTCUSDT.BINANCE"));

        let entry = create_test_entry(
            "L3-BTCUSDT-0000000000000001",
            OrderSide::Buy,
            "50000.00",
            "1.5",
            3,
        );

        let delta = emitter.emit_delete(
            &entry,
            UnixNanos::from(1000000000),
            UnixNanos::from(1000000000),
        );

        assert_eq!(delta.action, BookAction::Delete);
    }

    #[test]
    fn test_emit_snapshot() {
        let mut emitter = L3DeltaEmitter::new(InstrumentId::from("BTCUSDT.BINANCE"));

        let entries = vec![
            create_test_entry("order-1", OrderSide::Buy, "50000.00", "1.0", 1),
            create_test_entry("order-2", OrderSide::Buy, "49999.00", "2.0", 2),
            create_test_entry("order-3", OrderSide::Sell, "51000.00", "1.5", 3),
        ];

        let deltas = emitter.emit_snapshot(
            &entries,
            UnixNanos::from(1000000000),
            UnixNanos::from(1000000000),
        );

        assert_eq!(deltas.len(), 3);
        assert_eq!(deltas[0].action, BookAction::Add);
        assert_eq!(deltas[1].action, BookAction::Add);
        assert_eq!(deltas[2].action, BookAction::Add);

        // Check snapshot flag on first delta
        assert_ne!(deltas[0].flags & RecordFlag::F_SNAPSHOT as u8, 0);
    }

    #[test]
    fn test_emit_clear() {
        let mut emitter = L3DeltaEmitter::new(InstrumentId::from("BTCUSDT.BINANCE"));

        let delta = emitter.emit_clear(UnixNanos::from(1000000000), UnixNanos::from(1000000000));

        assert_eq!(delta.action, BookAction::Clear);
        assert_eq!(emitter.sequence(), 1);
    }

    #[test]
    fn test_sequence_increments() {
        let mut emitter = L3DeltaEmitter::new(InstrumentId::from("BTCUSDT.BINANCE"));

        let action1 = create_test_action(
            L3Action::Placed,
            OrderSide::Buy,
            "50000.00",
            "1.0",
            "order-1",
            1,
        );
        emitter.emit(&action1, UnixNanos::from(1000000000));
        assert_eq!(emitter.sequence(), 1);

        let action2 = create_test_action(
            L3Action::Modified,
            OrderSide::Buy,
            "50000.00",
            "2.0",
            "order-1",
            2,
        );
        emitter.emit(&action2, UnixNanos::from(1000000000));
        assert_eq!(emitter.sequence(), 2);

        let action3 = create_test_action(
            L3Action::Canceled,
            OrderSide::Buy,
            "50000.00",
            "2.0",
            "order-1",
            3,
        );
        emitter.emit(&action3, UnixNanos::from(1000000000));
        assert_eq!(emitter.sequence(), 3);
    }

    #[test]
    fn test_empty_snapshot() {
        let mut emitter = L3DeltaEmitter::new(InstrumentId::from("BTCUSDT.BINANCE"));

        let entries: Vec<OrderEntry> = vec![];
        let deltas = emitter.emit_snapshot(
            &entries,
            UnixNanos::from(1000000000),
            UnixNanos::from(1000000000),
        );

        assert!(deltas.is_empty());
    }

    #[test]
    fn test_venue_order_id_conversion() {
        // Test L3 format
        let id1 = VenueOrderId::new("L3-BTCUSDT-00000000000000ff");
        let numeric1 = L3DeltaEmitter::venue_order_id_to_u64(&id1);
        assert_eq!(numeric1, 0xff);

        // Test fallback for non-L3 format
        let id2 = VenueOrderId::new("some-other-id");
        let numeric2 = L3DeltaEmitter::venue_order_id_to_u64(&id2);
        assert!(numeric2 > 0); // Should produce some hash
    }
}
