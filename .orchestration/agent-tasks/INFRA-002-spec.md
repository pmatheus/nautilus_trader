# Task: INFRA-002 - Define Iceberg schema types for L3 events

## Objective
Create Rust structs for the three Iceberg table schemas (orderbook_l3_events, trades, liquidations) with Arrow schema derivation support.

## Context
These schemas define the structure of data written to Supabase Iceberg tables. Must support conversion to Apache Arrow format for Parquet serialization.

## Dependencies
- INFRA-001 must be complete (crate skeleton exists)

## Files to Create
1. `crates/iceberg/src/schema.rs` - Main schema definitions
2. `crates/iceberg/src/types.rs` - Supporting data types

## Requirements
- Define schema for `orderbook_l3_events` table:
  - timestamp (i64, microseconds)
  - exchange (string)
  - symbol (string)
  - order_id (string, synthetic for L3)
  - side (enum: bid/ask)
  - action (enum: add/modify/delete/trade)
  - price (decimal/f64)
  - quantity (decimal/f64)
  - sequence (i64)

- Define schema for `trades` table:
  - timestamp, exchange, symbol, trade_id, side, price, quantity, aggressor_side

- Define schema for `liquidations` table:
  - timestamp, exchange, symbol, side, price, quantity, liquidation_type

- Add Arrow schema derivation (use `arrow-schema` crate)
- Implement serialization/deserialization traits
- Add validation methods

## Report Format
Create `.orchestration/agent-tasks/INFRA-002-report.md` with:
- Status: SUCCESS/FAILED
- Files created
- Schema field counts and types
- Arrow compatibility confirmed
- Next tasks ready: INFRA-004, INFRA-005

## Success Criteria
- Schemas compile and derive Arrow schemas
- Types are compatible with nautilus_model types
- Ready for RecordBatch conversion
