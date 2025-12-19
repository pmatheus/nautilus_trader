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

//! Arrow RecordBatch conversion for Iceberg events.
//!
//! This module provides efficient conversion from batched events to Arrow RecordBatch
//! format for writing to Parquet files. The conversions use the schemas defined in
//! the `schema` module and optimize for minimal memory allocation.

use std::sync::Arc;

use arrow::array::{
    ArrayRef, StringArray, TimestampMicrosecondArray, UInt64Array,
};
use arrow::record_batch::RecordBatch;
use thiserror::Error;

use crate::schema::{LiquidationSchema, OrderbookL3EventSchema, TradeSchema};
use crate::types::{LiquidationEvent, OrderbookL3Event, TradeEvent};

/// Errors that can occur during Arrow conversion.
#[derive(Error, Debug)]
pub enum ArrowConversionError {
    /// Schema mismatch between data and Arrow schema.
    #[error("Schema mismatch: {0}")]
    SchemaMismatch(String),

    /// Arrow error during conversion.
    #[error("Arrow error: {0}")]
    ArrowError(#[from] arrow::error::ArrowError),

    /// Empty batch provided for conversion.
    #[error("Cannot convert empty batch")]
    EmptyBatch,

    /// Invalid data in event.
    #[error("Invalid data: {0}")]
    InvalidData(String),
}

/// Converts a batch of `OrderbookL3Event` to an Arrow `RecordBatch`.
///
/// # Arguments
///
/// * `events` - Vector of L3 orderbook events to convert
///
/// # Returns
///
/// An Arrow `RecordBatch` with the schema defined in `OrderbookL3EventSchema`.
///
/// # Errors
///
/// Returns an error if:
/// - The events vector is empty
/// - Arrow conversion fails
/// - Schema validation fails
///
/// # Example
///
/// ```no_run
/// use nautilus_iceberg::arrow::orderbook_l3_to_record_batch;
/// use nautilus_iceberg::types::{OrderbookL3Event, OrderAction, OrderSide};
///
/// let events = vec![
///     OrderbookL3Event {
///         timestamp: 1234567890,
///         instrument_id: "BTC-PERP".to_string(),
///         exchange: "BINANCE".to_string(),
///         order_id: "order_1".to_string(),
///         action: OrderAction::Add,
///         side: OrderSide::Buy,
///         price: "50000.0".to_string(),
///         quantity: "1.5".to_string(),
///         sequence_number: 1,
///     },
/// ];
///
/// let batch = orderbook_l3_to_record_batch(&events).unwrap();
/// assert_eq!(batch.num_rows(), 1);
/// ```
pub fn orderbook_l3_to_record_batch(
    events: &[OrderbookL3Event],
) -> Result<RecordBatch, ArrowConversionError> {
    if events.is_empty() {
        return Err(ArrowConversionError::EmptyBatch);
    }

    let schema = Arc::new(OrderbookL3EventSchema::arrow_schema());
    let num_events = events.len();

    // Pre-allocate vectors with exact capacity
    let mut timestamps = Vec::with_capacity(num_events);
    let mut instrument_ids = Vec::with_capacity(num_events);
    let mut exchanges = Vec::with_capacity(num_events);
    let mut order_ids = Vec::with_capacity(num_events);
    let mut actions = Vec::with_capacity(num_events);
    let mut sides = Vec::with_capacity(num_events);
    let mut prices = Vec::with_capacity(num_events);
    let mut quantities = Vec::with_capacity(num_events);
    let mut sequence_numbers = Vec::with_capacity(num_events);

    // Collect all data
    for event in events {
        timestamps.push(event.timestamp);
        instrument_ids.push(event.instrument_id.as_str());
        exchanges.push(event.exchange.as_str());
        order_ids.push(event.order_id.as_str());
        actions.push(event.action.as_str());
        sides.push(event.side.as_str());
        prices.push(event.price.as_str());
        quantities.push(event.quantity.as_str());
        sequence_numbers.push(event.sequence_number);
    }

    // Create Arrow arrays
    let timestamp_array: ArrayRef = Arc::new(TimestampMicrosecondArray::from(timestamps));
    let instrument_id_array: ArrayRef = Arc::new(StringArray::from(instrument_ids));
    let exchange_array: ArrayRef = Arc::new(StringArray::from(exchanges));
    let order_id_array: ArrayRef = Arc::new(StringArray::from(order_ids));
    let action_array: ArrayRef = Arc::new(StringArray::from(actions));
    let side_array: ArrayRef = Arc::new(StringArray::from(sides));
    let price_array: ArrayRef = Arc::new(StringArray::from(prices));
    let quantity_array: ArrayRef = Arc::new(StringArray::from(quantities));
    let sequence_number_array: ArrayRef = Arc::new(UInt64Array::from(sequence_numbers));

    // Create RecordBatch
    let batch = RecordBatch::try_new(
        schema,
        vec![
            timestamp_array,
            instrument_id_array,
            exchange_array,
            order_id_array,
            action_array,
            side_array,
            price_array,
            quantity_array,
            sequence_number_array,
        ],
    )?;

    Ok(batch)
}

/// Converts a batch of `TradeEvent` to an Arrow `RecordBatch`.
///
/// # Arguments
///
/// * `events` - Vector of trade events to convert
///
/// # Returns
///
/// An Arrow `RecordBatch` with the schema defined in `TradeSchema`.
///
/// # Errors
///
/// Returns an error if:
/// - The events vector is empty
/// - Arrow conversion fails
/// - Schema validation fails
///
/// # Example
///
/// ```no_run
/// use nautilus_iceberg::arrow::trade_to_record_batch;
/// use nautilus_iceberg::types::{TradeEvent, OrderSide};
///
/// let events = vec![
///     TradeEvent {
///         timestamp: 1234567890,
///         instrument_id: "BTC-PERP".to_string(),
///         exchange: "BINANCE".to_string(),
///         trade_id: "trade_1".to_string(),
///         side: OrderSide::Buy,
///         price: "50000.0".to_string(),
///         quantity: "1.5".to_string(),
///         aggressor_side: "BUY".to_string(),
///     },
/// ];
///
/// let batch = trade_to_record_batch(&events).unwrap();
/// assert_eq!(batch.num_rows(), 1);
/// ```
pub fn trade_to_record_batch(
    events: &[TradeEvent],
) -> Result<RecordBatch, ArrowConversionError> {
    if events.is_empty() {
        return Err(ArrowConversionError::EmptyBatch);
    }

    let schema = Arc::new(TradeSchema::arrow_schema());
    let num_events = events.len();

    // Pre-allocate vectors
    let mut timestamps = Vec::with_capacity(num_events);
    let mut instrument_ids = Vec::with_capacity(num_events);
    let mut exchanges = Vec::with_capacity(num_events);
    let mut trade_ids = Vec::with_capacity(num_events);
    let mut sides = Vec::with_capacity(num_events);
    let mut prices = Vec::with_capacity(num_events);
    let mut quantities = Vec::with_capacity(num_events);
    let mut aggressor_sides = Vec::with_capacity(num_events);

    // Collect all data
    for event in events {
        timestamps.push(event.timestamp);
        instrument_ids.push(event.instrument_id.as_str());
        exchanges.push(event.exchange.as_str());
        trade_ids.push(event.trade_id.as_str());
        sides.push(event.side.as_str());
        prices.push(event.price.as_str());
        quantities.push(event.quantity.as_str());
        aggressor_sides.push(event.aggressor_side.as_str());
    }

    // Create Arrow arrays
    let timestamp_array: ArrayRef = Arc::new(TimestampMicrosecondArray::from(timestamps));
    let instrument_id_array: ArrayRef = Arc::new(StringArray::from(instrument_ids));
    let exchange_array: ArrayRef = Arc::new(StringArray::from(exchanges));
    let trade_id_array: ArrayRef = Arc::new(StringArray::from(trade_ids));
    let side_array: ArrayRef = Arc::new(StringArray::from(sides));
    let price_array: ArrayRef = Arc::new(StringArray::from(prices));
    let quantity_array: ArrayRef = Arc::new(StringArray::from(quantities));
    let aggressor_side_array: ArrayRef = Arc::new(StringArray::from(aggressor_sides));

    // Create RecordBatch
    let batch = RecordBatch::try_new(
        schema,
        vec![
            timestamp_array,
            instrument_id_array,
            exchange_array,
            trade_id_array,
            side_array,
            price_array,
            quantity_array,
            aggressor_side_array,
        ],
    )?;

    Ok(batch)
}

/// Converts a batch of `LiquidationEvent` to an Arrow `RecordBatch`.
///
/// # Arguments
///
/// * `events` - Vector of liquidation events to convert
///
/// # Returns
///
/// An Arrow `RecordBatch` with the schema defined in `LiquidationSchema`.
///
/// # Errors
///
/// Returns an error if:
/// - The events vector is empty
/// - Arrow conversion fails
/// - Schema validation fails
///
/// # Example
///
/// ```no_run
/// use nautilus_iceberg::arrow::liquidation_to_record_batch;
/// use nautilus_iceberg::types::{LiquidationEvent, OrderSide};
///
/// let events = vec![
///     LiquidationEvent {
///         timestamp: 1234567890,
///         instrument_id: "BTC-PERP".to_string(),
///         exchange: "BINANCE".to_string(),
///         liquidation_id: "liq_1".to_string(),
///         side: OrderSide::Sell,
///         price: "50000.0".to_string(),
///         quantity: "1.5".to_string(),
///     },
/// ];
///
/// let batch = liquidation_to_record_batch(&events).unwrap();
/// assert_eq!(batch.num_rows(), 1);
/// ```
pub fn liquidation_to_record_batch(
    events: &[LiquidationEvent],
) -> Result<RecordBatch, ArrowConversionError> {
    if events.is_empty() {
        return Err(ArrowConversionError::EmptyBatch);
    }

    let schema = Arc::new(LiquidationSchema::arrow_schema());
    let num_events = events.len();

    // Pre-allocate vectors
    let mut timestamps = Vec::with_capacity(num_events);
    let mut instrument_ids = Vec::with_capacity(num_events);
    let mut exchanges = Vec::with_capacity(num_events);
    let mut liquidation_ids = Vec::with_capacity(num_events);
    let mut sides = Vec::with_capacity(num_events);
    let mut prices = Vec::with_capacity(num_events);
    let mut quantities = Vec::with_capacity(num_events);

    // Collect all data
    for event in events {
        timestamps.push(event.timestamp);
        instrument_ids.push(event.instrument_id.as_str());
        exchanges.push(event.exchange.as_str());
        liquidation_ids.push(event.liquidation_id.as_str());
        sides.push(event.side.as_str());
        prices.push(event.price.as_str());
        quantities.push(event.quantity.as_str());
    }

    // Create Arrow arrays
    let timestamp_array: ArrayRef = Arc::new(TimestampMicrosecondArray::from(timestamps));
    let instrument_id_array: ArrayRef = Arc::new(StringArray::from(instrument_ids));
    let exchange_array: ArrayRef = Arc::new(StringArray::from(exchanges));
    let liquidation_id_array: ArrayRef = Arc::new(StringArray::from(liquidation_ids));
    let side_array: ArrayRef = Arc::new(StringArray::from(sides));
    let price_array: ArrayRef = Arc::new(StringArray::from(prices));
    let quantity_array: ArrayRef = Arc::new(StringArray::from(quantities));

    // Create RecordBatch
    let batch = RecordBatch::try_new(
        schema,
        vec![
            timestamp_array,
            instrument_id_array,
            exchange_array,
            liquidation_id_array,
            side_array,
            price_array,
            quantity_array,
        ],
    )?;

    Ok(batch)
}

////////////////////////////////////////////////////////////////////////////////
// Tests
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{OrderAction, OrderSide};

    #[test]
    fn test_orderbook_l3_conversion() {
        let events = vec![
            OrderbookL3Event {
                timestamp: 1234567890,
                instrument_id: "BTC-PERP".to_string(),
                exchange: "BINANCE".to_string(),
                order_id: "order_1".to_string(),
                action: OrderAction::Add,
                side: OrderSide::Buy,
                price: "50000.0".to_string(),
                quantity: "1.5".to_string(),
                sequence_number: 1,
            },
            OrderbookL3Event {
                timestamp: 1234567891,
                instrument_id: "ETH-PERP".to_string(),
                exchange: "BINANCE".to_string(),
                order_id: "order_2".to_string(),
                action: OrderAction::Update,
                side: OrderSide::Sell,
                price: "3000.0".to_string(),
                quantity: "10.0".to_string(),
                sequence_number: 2,
            },
        ];

        let batch = orderbook_l3_to_record_batch(&events).unwrap();

        assert_eq!(batch.num_rows(), 2);
        assert_eq!(batch.num_columns(), 9);
        assert_eq!(batch.schema().fields().len(), 9);

        // Verify schema field names
        let schema = batch.schema();
        assert_eq!(schema.field(0).name(), "timestamp");
        assert_eq!(schema.field(1).name(), "instrument_id");
        assert_eq!(schema.field(8).name(), "sequence_number");
    }

    #[test]
    fn test_trade_conversion() {
        let events = vec![
            TradeEvent {
                timestamp: 1234567890,
                instrument_id: "BTC-PERP".to_string(),
                exchange: "BINANCE".to_string(),
                trade_id: "trade_1".to_string(),
                side: OrderSide::Buy,
                price: "50000.0".to_string(),
                quantity: "1.5".to_string(),
                aggressor_side: "BUY".to_string(),
            },
            TradeEvent {
                timestamp: 1234567891,
                instrument_id: "ETH-PERP".to_string(),
                exchange: "BINANCE".to_string(),
                trade_id: "trade_2".to_string(),
                side: OrderSide::Sell,
                price: "3000.0".to_string(),
                quantity: "10.0".to_string(),
                aggressor_side: "SELL".to_string(),
            },
        ];

        let batch = trade_to_record_batch(&events).unwrap();

        assert_eq!(batch.num_rows(), 2);
        assert_eq!(batch.num_columns(), 8);

        let schema = batch.schema();
        assert_eq!(schema.field(0).name(), "timestamp");
        assert_eq!(schema.field(3).name(), "trade_id");
        assert_eq!(schema.field(7).name(), "aggressor_side");
    }

    #[test]
    fn test_liquidation_conversion() {
        let events = vec![
            LiquidationEvent {
                timestamp: 1234567890,
                instrument_id: "BTC-PERP".to_string(),
                exchange: "BINANCE".to_string(),
                liquidation_id: "liq_1".to_string(),
                side: OrderSide::Sell,
                price: "50000.0".to_string(),
                quantity: "1.5".to_string(),
            },
        ];

        let batch = liquidation_to_record_batch(&events).unwrap();

        assert_eq!(batch.num_rows(), 1);
        assert_eq!(batch.num_columns(), 7);

        let schema = batch.schema();
        assert_eq!(schema.field(0).name(), "timestamp");
        assert_eq!(schema.field(3).name(), "liquidation_id");
    }

    #[test]
    fn test_empty_batch_error() {
        let events: Vec<OrderbookL3Event> = vec![];
        let result = orderbook_l3_to_record_batch(&events);
        assert!(matches!(result, Err(ArrowConversionError::EmptyBatch)));

        let events: Vec<TradeEvent> = vec![];
        let result = trade_to_record_batch(&events);
        assert!(matches!(result, Err(ArrowConversionError::EmptyBatch)));

        let events: Vec<LiquidationEvent> = vec![];
        let result = liquidation_to_record_batch(&events);
        assert!(matches!(result, Err(ArrowConversionError::EmptyBatch)));
    }

    #[test]
    fn test_large_batch_conversion() {
        // Test with 1000 events
        let events: Vec<OrderbookL3Event> = (0..1000)
            .map(|i| OrderbookL3Event {
                timestamp: 1234567890 + i,
                instrument_id: format!("BTC-PERP-{}", i),
                exchange: "BINANCE".to_string(),
                order_id: format!("order_{}", i),
                action: OrderAction::Add,
                side: if i % 2 == 0 {
                    OrderSide::Buy
                } else {
                    OrderSide::Sell
                },
                price: format!("{}.0", 50000 + i),
                quantity: format!("{}.5", i),
                sequence_number: i as u64,
            })
            .collect();

        let batch = orderbook_l3_to_record_batch(&events).unwrap();
        assert_eq!(batch.num_rows(), 1000);
    }

    #[test]
    fn test_schema_compatibility() {
        let events = vec![OrderbookL3Event {
            timestamp: 1234567890,
            instrument_id: "BTC-PERP".to_string(),
            exchange: "BINANCE".to_string(),
            order_id: "order_1".to_string(),
            action: OrderAction::Add,
            side: OrderSide::Buy,
            price: "50000.0".to_string(),
            quantity: "1.5".to_string(),
            sequence_number: 1,
        }];

        let batch = orderbook_l3_to_record_batch(&events).unwrap();
        let expected_schema = OrderbookL3EventSchema::arrow_schema();

        // Compare schemas
        assert_eq!(batch.schema().fields().len(), expected_schema.fields().len());
        for (actual, expected) in batch.schema().fields().iter().zip(expected_schema.fields()) {
            assert_eq!(actual.name(), expected.name());
            assert_eq!(actual.data_type(), expected.data_type());
            assert_eq!(actual.is_nullable(), expected.is_nullable());
        }
    }

    #[test]
    fn test_string_data_preservation() {
        let events = vec![OrderbookL3Event {
            timestamp: 1234567890,
            instrument_id: "BTC-PERP".to_string(),
            exchange: "BINANCE".to_string(),
            order_id: "order_123_test".to_string(),
            action: OrderAction::Add,
            side: OrderSide::Buy,
            price: "50000.12345".to_string(),
            quantity: "1.5678".to_string(),
            sequence_number: 999,
        }];

        let batch = orderbook_l3_to_record_batch(&events).unwrap();

        // Verify data is preserved correctly
        let order_id_array = batch
            .column(3)
            .as_any()
            .downcast_ref::<StringArray>()
            .unwrap();
        assert_eq!(order_id_array.value(0), "order_123_test");

        let price_array = batch
            .column(6)
            .as_any()
            .downcast_ref::<StringArray>()
            .unwrap();
        assert_eq!(price_array.value(0), "50000.12345");
    }
}
