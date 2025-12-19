# Task: INFRA-003 - Implement IcebergConfig for Supabase connection

## Objective
Create configuration struct with Supabase project ref, warehouse name, S3 endpoint, catalog URI, and credentials handling.

## Context
This config manages connection to Supabase's Iceberg REST catalog and S3-compatible storage.

## Dependencies
- INFRA-001 must be complete

## Files to Modify/Create
1. `crates/iceberg/src/config.rs` - Already created in skeleton, implement it

## Requirements
- Define `IcebergConfig` struct with fields:
  - `supabase_project_ref`: String (e.g., "abc123")
  - `warehouse_name`: String (default: "default")
  - `catalog_uri`: String (computed from project ref)
  - `s3_endpoint`: String (computed from project ref)
  - `access_key_id`: String (from env or config)
  - `secret_access_key`: String (from env or config)
  - `region`: String (default: "us-east-1")

- Implement builder pattern for ergonomic construction
- Add methods:
  - `from_env()` - Read from environment variables
  - `validate()` - Ensure all required fields present
  - `catalog_url()` - Compute full catalog REST URL
  - `s3_url()` - Compute S3 endpoint URL

- Use `serde` for serialization
- Support both TOML and JSON config formats

## Report Format
Create `.orchestration/agent-tasks/INFRA-003-report.md` with:
- Status: SUCCESS/FAILED
- Config structure overview
- Environment variable names used
- Next tasks ready: INFRA-006

## Success Criteria
- Config can be constructed from env vars
- Validation works correctly
- URLs are computed properly
