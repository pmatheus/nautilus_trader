# Task: L3-002 - Implement synthetic order ID generator

## Objective
Create deterministic order ID generator using FNV or xxHash for fast hashing to create synthetic order IDs from L2 price levels.

## Context
Since L2 data doesn't have order IDs, we need to generate synthetic IDs deterministically based on (exchange, symbol, side, price, timestamp) to track orders through their lifecycle.

## Dependencies
- L3-001 must be complete

## Files to Create
1. `crates/l3_engine/src/order_id.rs` - Order ID generation logic

## Requirements
- Implement `OrderIdGenerator` struct
- Use xxHash (xxhash-rust crate) or FNV for speed
- Generate IDs from:
  - Exchange name
  - Symbol
  - Side (bid/ask)
  - Price level
  - Initial timestamp

- Methods:
  - `generate(exchange, symbol, side, price, ts) -> String`
  - `validate(order_id) -> bool` - Check format

- Properties:
  - Deterministic (same inputs = same ID)
  - Fast (<100ns per generation)
  - Collision resistant for practical market data

- Add unit tests with known inputs/outputs

## Report Format
Create `.orchestration/agent-tasks/L3-002-report.md` with:
- Status: SUCCESS/FAILED
- Hash algorithm chosen (xxHash/FNV) with justification
- Performance benchmarks
- Next tasks ready: L3-003

## Success Criteria
- Generates deterministic IDs
- Fast enough for real-time processing
- Tests pass with collision checks
