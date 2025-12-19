# Task: INFRA-001 - Create nautilus_iceberg crate skeleton

## Objective
Initialize the new Rust crate for Iceberg integration with proper Cargo.toml, lib.rs, and module structure following existing crate patterns in the nautilus_trader codebase.

## Context
This is the foundation for the Iceberg data lake integration. The crate will handle:
- Apache Iceberg table schema management
- Parquet file writing to Supabase S3
- REST catalog integration for atomic commits
- Event batching and buffering

## Files to Create
1. `crates/iceberg/Cargo.toml` - Crate manifest with dependencies
2. `crates/iceberg/src/lib.rs` - Main library entry point
3. `crates/iceberg/src/config.rs` - Configuration types
4. `crates/Cargo.toml` - Update workspace members

## Requirements
- Follow existing nautilus crate patterns (study `crates/adapters/*/` structure)
- Include appropriate dependencies: `arrow`, `parquet`, `object_store`, `serde`, `thiserror`
- Set up module structure for future components: `schema`, `buffer`, `writer`, `catalog`, `sink`
- Add Python bindings placeholder module
- Ensure proper error handling types

## Report Format
Create `.orchestration/agent-tasks/INFRA-001-report.md` with:
- Status: SUCCESS/FAILED
- Files created with paths
- Any deviations from existing patterns (with justification)
- Next dependencies ready: INFRA-002, INFRA-003

## Success Criteria
- Crate compiles with `cargo build -p nautilus-iceberg`
- Workspace recognizes new member
- Module structure is ready for implementation
