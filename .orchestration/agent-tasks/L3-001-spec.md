# Task: L3-001 - Create nautilus_l3_engine crate skeleton

## Objective
Initialize the new Rust crate for L3 orderbook reconstruction with dependencies on nautilus_model and proper module structure.

## Context
This crate implements the intelligent L2→L3 reconstruction algorithm that correlates orderbook deltas with trade events to synthesize L3 market-by-order data for exchanges that don't natively provide it.

## Files to Create
1. `crates/l3_engine/Cargo.toml` - Crate manifest
2. `crates/l3_engine/src/lib.rs` - Main library entry
3. `crates/Cargo.toml` - Update workspace members

## Requirements
- Add dependency on `nautilus-model` for orderbook and trade types
- Include `xxhash-rust` or `fnv` for fast order ID hashing
- Set up module structure: `order_id`, `correlator`, `state`, `emitter`, `engine`, `exchange`
- Add Python bindings placeholder module
- Follow existing nautilus crate patterns

## Report Format
Create `.orchestration/agent-tasks/L3-001-report.md` with:
- Status: SUCCESS/FAILED
- Files created with paths
- Dependency graph verification
- Next tasks ready: L3-002, L3-003

## Success Criteria
- Crate compiles with `cargo build -p nautilus-l3-engine`
- Can import nautilus_model types
- Module structure supports planned architecture
