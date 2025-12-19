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

//! Schema definitions for Iceberg tables storing L3 orderbook events.
//!
//! This module defines three primary Iceberg table schemas:
//! - `OrderbookL3EventSchema`: L3 orderbook events (Add/Update/Delete)
//! - `TradeSchema`: Trade executions
//! - `LiquidationSchema`: Liquidation events

use arrow::datatypes::{DataType, Field, Schema, TimeUnit};
use serde::{Deserialize, Serialize};

/// Schema for the `orderbook_l3_events` Iceberg table.
///
/// This table stores Level 3 orderbook events with individual order updates.
///
/// # Fields
/// - `timestamp`: Event timestamp (microseconds since epoch)
/// - `instrument_id`: Trading instrument identifier (e.g., "BTC-PERP")
/// - `exchange`: Exchange name (e.g., "BINANCE")
/// - `order_id`: Unique order identifier
/// - `action`: Order action (ADD, UPDATE, DELETE)
/// - `side`: Order side (BUY, SELL)
/// - `price`: Order price (stored as decimal string for precision)
/// - `quantity`: Order quantity (stored as decimal string for precision)
/// - `sequence_number`: Monotonically increasing sequence number
///
/// # Partitioning
/// - Partitioned by `hour(timestamp)` for efficient time-range queries
///
/// # Primary Keys
/// - exchange, instrument_id, timestamp
#[derive(Debug, Clone)]
pub struct OrderbookL3EventSchema;

impl OrderbookL3EventSchema {
    /// Returns the Arrow schema for the orderbook L3 events table.
    #[must_use]
    pub fn arrow_schema() -> Schema {
        Schema::new(vec![
            Field::new(
                "timestamp",
                DataType::Timestamp(TimeUnit::Microsecond, None),
                false,
            ),
            Field::new("instrument_id", DataType::Utf8, false),
            Field::new("exchange", DataType::Utf8, false),
            Field::new("order_id", DataType::Utf8, false),
            Field::new("action", DataType::Utf8, false),
            Field::new("side", DataType::Utf8, false),
            Field::new("price", DataType::Utf8, false),
            Field::new("quantity", DataType::Utf8, false),
            Field::new("sequence_number", DataType::UInt64, false),
        ])
    }

    /// Returns the table name.
    #[must_use]
    pub fn table_name() -> &'static str {
        "orderbook_l3_events"
    }

    /// Returns the partition specification (hour-based partitioning).
    #[must_use]
    pub fn partition_spec() -> PartitionSpec {
        PartitionSpec {
            field: "timestamp".to_string(),
            transform: PartitionTransform::Hour,
        }
    }

    /// Returns the primary key fields.
    #[must_use]
    pub fn primary_keys() -> Vec<&'static str> {
        vec!["exchange", "instrument_id", "timestamp"]
    }
}

/// Schema for the `trades` Iceberg table.
///
/// This table stores executed trades from exchanges.
///
/// # Fields
/// - `timestamp`: Trade timestamp (microseconds since epoch)
/// - `instrument_id`: Trading instrument identifier
/// - `exchange`: Exchange name
/// - `trade_id`: Unique trade identifier
/// - `side`: Trade side (BUY, SELL)
/// - `price`: Trade price (stored as decimal string for precision)
/// - `quantity`: Trade quantity (stored as decimal string for precision)
/// - `aggressor_side`: Side of the aggressor (BUY, SELL, or NONE)
///
/// # Partitioning
/// - Partitioned by `hour(timestamp)` for efficient time-range queries
///
/// # Primary Keys
/// - exchange, instrument_id, timestamp
#[derive(Debug, Clone)]
pub struct TradeSchema;

impl TradeSchema {
    /// Returns the Arrow schema for the trades table.
    #[must_use]
    pub fn arrow_schema() -> Schema {
        Schema::new(vec![
            Field::new(
                "timestamp",
                DataType::Timestamp(TimeUnit::Microsecond, None),
                false,
            ),
            Field::new("instrument_id", DataType::Utf8, false),
            Field::new("exchange", DataType::Utf8, false),
            Field::new("trade_id", DataType::Utf8, false),
            Field::new("side", DataType::Utf8, false),
            Field::new("price", DataType::Utf8, false),
            Field::new("quantity", DataType::Utf8, false),
            Field::new("aggressor_side", DataType::Utf8, false),
        ])
    }

    /// Returns the table name.
    #[must_use]
    pub fn table_name() -> &'static str {
        "trades"
    }

    /// Returns the partition specification (hour-based partitioning).
    #[must_use]
    pub fn partition_spec() -> PartitionSpec {
        PartitionSpec {
            field: "timestamp".to_string(),
            transform: PartitionTransform::Hour,
        }
    }

    /// Returns the primary key fields.
    #[must_use]
    pub fn primary_keys() -> Vec<&'static str> {
        vec!["exchange", "instrument_id", "timestamp"]
    }
}

/// Schema for the `liquidations` Iceberg table.
///
/// This table stores liquidation events from exchanges.
///
/// # Fields
/// - `timestamp`: Liquidation timestamp (microseconds since epoch)
/// - `instrument_id`: Trading instrument identifier
/// - `exchange`: Exchange name
/// - `liquidation_id`: Unique liquidation identifier
/// - `side`: Liquidation side (BUY, SELL)
/// - `price`: Liquidation price (stored as decimal string for precision)
/// - `quantity`: Liquidation quantity (stored as decimal string for precision)
///
/// # Partitioning
/// - Partitioned by `hour(timestamp)` for efficient time-range queries
///
/// # Primary Keys
/// - exchange, instrument_id, timestamp
#[derive(Debug, Clone)]
pub struct LiquidationSchema;

impl LiquidationSchema {
    /// Returns the Arrow schema for the liquidations table.
    #[must_use]
    pub fn arrow_schema() -> Schema {
        Schema::new(vec![
            Field::new(
                "timestamp",
                DataType::Timestamp(TimeUnit::Microsecond, None),
                false,
            ),
            Field::new("instrument_id", DataType::Utf8, false),
            Field::new("exchange", DataType::Utf8, false),
            Field::new("liquidation_id", DataType::Utf8, false),
            Field::new("side", DataType::Utf8, false),
            Field::new("price", DataType::Utf8, false),
            Field::new("quantity", DataType::Utf8, false),
        ])
    }

    /// Returns the table name.
    #[must_use]
    pub fn table_name() -> &'static str {
        "liquidations"
    }

    /// Returns the partition specification (hour-based partitioning).
    #[must_use]
    pub fn partition_spec() -> PartitionSpec {
        PartitionSpec {
            field: "timestamp".to_string(),
            transform: PartitionTransform::Hour,
        }
    }

    /// Returns the primary key fields.
    #[must_use]
    pub fn primary_keys() -> Vec<&'static str> {
        vec!["exchange", "instrument_id", "timestamp"]
    }
}

/// Partition specification for Iceberg tables.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartitionSpec {
    /// The field to partition on.
    pub field: String,
    /// The partition transform to apply.
    pub transform: PartitionTransform,
}

/// Partition transform types supported by Iceberg.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PartitionTransform {
    /// Identity transform (no transformation).
    Identity,
    /// Year-based partitioning.
    Year,
    /// Month-based partitioning.
    Month,
    /// Day-based partitioning.
    Day,
    /// Hour-based partitioning.
    Hour,
    /// Bucket partitioning with specified number of buckets.
    Bucket(u32),
    /// Truncate to specified width.
    Truncate(u32),
}

impl PartitionTransform {
    /// Returns the string representation for Iceberg partition specs.
    #[must_use]
    pub fn as_str(&self) -> String {
        match self {
            Self::Identity => "identity".to_string(),
            Self::Year => "year".to_string(),
            Self::Month => "month".to_string(),
            Self::Day => "day".to_string(),
            Self::Hour => "hour".to_string(),
            Self::Bucket(n) => format!("bucket[{}]", n),
            Self::Truncate(w) => format!("truncate[{}]", w),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Tests
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orderbook_l3_schema_fields() {
        let schema = OrderbookL3EventSchema::arrow_schema();
        assert_eq!(schema.fields().len(), 9);

        assert_eq!(schema.field(0).name(), "timestamp");
        assert_eq!(schema.field(1).name(), "instrument_id");
        assert_eq!(schema.field(2).name(), "exchange");
        assert_eq!(schema.field(3).name(), "order_id");
        assert_eq!(schema.field(4).name(), "action");
        assert_eq!(schema.field(5).name(), "side");
        assert_eq!(schema.field(6).name(), "price");
        assert_eq!(schema.field(7).name(), "quantity");
        assert_eq!(schema.field(8).name(), "sequence_number");
    }

    #[test]
    fn test_orderbook_l3_schema_types() {
        let schema = OrderbookL3EventSchema::arrow_schema();

        assert!(matches!(
            schema.field(0).data_type(),
            DataType::Timestamp(TimeUnit::Microsecond, None)
        ));
        assert!(matches!(schema.field(1).data_type(), DataType::Utf8));
        assert!(matches!(schema.field(8).data_type(), DataType::UInt64));
    }

    #[test]
    fn test_orderbook_l3_schema_metadata() {
        assert_eq!(
            OrderbookL3EventSchema::table_name(),
            "orderbook_l3_events"
        );
        assert_eq!(
            OrderbookL3EventSchema::primary_keys(),
            vec!["exchange", "instrument_id", "timestamp"]
        );
        assert_eq!(
            OrderbookL3EventSchema::partition_spec().field,
            "timestamp"
        );
        assert_eq!(
            OrderbookL3EventSchema::partition_spec().transform,
            PartitionTransform::Hour
        );
    }

    #[test]
    fn test_trade_schema_fields() {
        let schema = TradeSchema::arrow_schema();
        assert_eq!(schema.fields().len(), 8);

        assert_eq!(schema.field(0).name(), "timestamp");
        assert_eq!(schema.field(1).name(), "instrument_id");
        assert_eq!(schema.field(2).name(), "exchange");
        assert_eq!(schema.field(3).name(), "trade_id");
        assert_eq!(schema.field(4).name(), "side");
        assert_eq!(schema.field(5).name(), "price");
        assert_eq!(schema.field(6).name(), "quantity");
        assert_eq!(schema.field(7).name(), "aggressor_side");
    }

    #[test]
    fn test_trade_schema_metadata() {
        assert_eq!(TradeSchema::table_name(), "trades");
        assert_eq!(
            TradeSchema::primary_keys(),
            vec!["exchange", "instrument_id", "timestamp"]
        );
    }

    #[test]
    fn test_liquidation_schema_fields() {
        let schema = LiquidationSchema::arrow_schema();
        assert_eq!(schema.fields().len(), 7);

        assert_eq!(schema.field(0).name(), "timestamp");
        assert_eq!(schema.field(1).name(), "instrument_id");
        assert_eq!(schema.field(2).name(), "exchange");
        assert_eq!(schema.field(3).name(), "liquidation_id");
        assert_eq!(schema.field(4).name(), "side");
        assert_eq!(schema.field(5).name(), "price");
        assert_eq!(schema.field(6).name(), "quantity");
    }

    #[test]
    fn test_liquidation_schema_metadata() {
        assert_eq!(LiquidationSchema::table_name(), "liquidations");
        assert_eq!(
            LiquidationSchema::primary_keys(),
            vec!["exchange", "instrument_id", "timestamp"]
        );
    }

    #[test]
    fn test_partition_transform_as_str() {
        assert_eq!(PartitionTransform::Identity.as_str(), "identity");
        assert_eq!(PartitionTransform::Year.as_str(), "year");
        assert_eq!(PartitionTransform::Month.as_str(), "month");
        assert_eq!(PartitionTransform::Day.as_str(), "day");
        assert_eq!(PartitionTransform::Hour.as_str(), "hour");
        assert_eq!(PartitionTransform::Bucket(10).as_str(), "bucket[10]");
        assert_eq!(PartitionTransform::Truncate(5).as_str(), "truncate[5]");
    }

    #[test]
    fn test_partition_spec_serialization() {
        let spec = PartitionSpec {
            field: "timestamp".to_string(),
            transform: PartitionTransform::Hour,
        };

        let json = serde_json::to_string(&spec).unwrap();
        let deserialized: PartitionSpec = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.field, "timestamp");
        assert_eq!(deserialized.transform, PartitionTransform::Hour);
    }

    #[test]
    fn test_all_schemas_non_nullable() {
        let l3_schema = OrderbookL3EventSchema::arrow_schema();
        let trade_schema = TradeSchema::arrow_schema();
        let liquidation_schema = LiquidationSchema::arrow_schema();

        for field in l3_schema.fields() {
            assert!(!field.is_nullable(), "Field {} should be non-nullable", field.name());
        }

        for field in trade_schema.fields() {
            assert!(!field.is_nullable(), "Field {} should be non-nullable", field.name());
        }

        for field in liquidation_schema.fields() {
            assert!(!field.is_nullable(), "Field {} should be non-nullable", field.name());
        }
    }
}
