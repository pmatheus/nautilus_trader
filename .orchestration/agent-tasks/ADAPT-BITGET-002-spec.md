# Task: ADAPT-BITGET-002 - Create Bitget Rust crate skeleton

## Objective
Initialize Rust crate for Bitget adapter following patterns from nautilus-bybit.

## Context
The Rust crate provides high-performance HTTP and WebSocket clients for Bitget integration.

## Dependencies
- ADAPT-BITGET-001 must be complete (Python skeleton exists)

## Files to Create
1. `crates/adapters/bitget/Cargo.toml` - Crate manifest
2. `crates/adapters/bitget/src/lib.rs` - Main entry
3. `crates/adapters/bitget/src/config.rs` - Rust config types
4. Update `crates/Cargo.toml` - Add to workspace

## Requirements
- Study `crates/adapters/bybit/` structure as template
- Add dependencies:
  - `nautilus-model` for data types
  - `reqwest` for HTTP
  - `tokio-tungstenite` for WebSocket
  - `serde`, `serde_json` for serialization
  - `hmac`, `sha2` for authentication
  - `thiserror` for errors

- Set up module structure:
  - `http/` - HTTP client
  - `websocket/` - WebSocket client
  - `common/` - Shared types
  - `python/` - PyO3 bindings

- Add error types in `lib.rs`

## Report Format
Create `.orchestration/agent-tasks/ADAPT-BITGET-002-report.md` with:
- Status: SUCCESS/FAILED
- Files created
- Dependencies added
- Next tasks ready: ADAPT-BITGET-003, ADAPT-BITGET-005

## Success Criteria
- Compiles with `cargo build -p nautilus-bitget`
- Module structure mirrors bybit pattern
