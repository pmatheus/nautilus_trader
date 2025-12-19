# Wave 3: Advanced Implementation - Execution Plan

**Start Time**: 2025-12-19T11:58:16Z
**Status**: LAUNCHING
**Total Tasks**: 6
**Parallel Lanes**: 4

---

## Wave 3 Task Allocation

### Lane 1: Infrastructure (3 tasks in sequence)
**Agent**: `rust-expert`
**Tasks**: INFRA-004 → INFRA-005 → INFRA-006
**Focus**: Event batching, Arrow conversion, Parquet writing
**Estimated Duration**: ~8-10h sequential work
**Dependencies**: INFRA-002 completed ✓

**Task Details**:
1. **INFRA-004**: Event batching buffer (thread-safe ring buffer)
2. **INFRA-005**: Arrow RecordBatch conversion (batch events to Arrow)
3. **INFRA-006**: Parquet file writer with S3 upload (Supabase S3)

---

### Lane 2: L3 Engine (1 task)
**Agent**: `rust-expert`
**Tasks**: L3-003
**Focus**: Core L2→L3 correlation algorithm
**Estimated Duration**: ~4-8h
**Dependencies**: L3-001, L3-002 completed ✓

**Task Details**:
1. **L3-003**: L2 delta correlation engine (correlate L2 deltas with trades to infer order actions)

---

### Lane 3: Bitget Adapter (3 tasks in parallel)
**Agent**: `rust-expert`
**Tasks**: ADAPT-BITGET-003, ADAPT-BITGET-004, ADAPT-BITGET-005
**Focus**: HTTP client, instrument provider, WebSocket client
**Estimated Duration**: ~8-12h parallelizable
**Dependencies**: ADAPT-BITGET-002 completed ✓

**Task Details**:
1. **ADAPT-BITGET-003**: HTTP client (REST API + auth + rate limiting)
2. **ADAPT-BITGET-004**: Instrument provider (fetch/parse symbols)
3. **ADAPT-BITGET-005**: WebSocket client (subscriptions + reconnection)

---

### Lane 4: Kraken Enhancement (1 task)
**Agent**: `python-dev-expert`
**Tasks**: ADAPT-KRAKEN-004
**Focus**: Python data client integration
**Estimated Duration**: ~1-2h
**Dependencies**: ADAPT-KRAKEN-002, ADAPT-KRAKEN-003 completed ✓

**Task Details**:
1. **ADAPT-KRAKEN-004**: Update KrakenDataClient for proper subscription handling

---

## Parallelization Strategy

### Why 4 separate agents instead of 1 agent with 6 tasks?

**Maximum Parallelization**: Each lane is completely independent:
- Lane 1 (Infrastructure): Works in `crates/iceberg/`
- Lane 2 (L3 Engine): Works in `crates/l3_engine/`
- Lane 3 (Bitget): Works in `crates/adapters/bitget/` and `nautilus_trader/adapters/bitget/`
- Lane 4 (Kraken): Works in `nautilus_trader/adapters/kraken/`

**Zero File Conflicts**: No shared files between lanes means zero merge conflicts.

**Optimal Resource Usage**: 4 agents can complete 6 tasks faster than 1 agent sequentially.

---

## Agent Launch Plan

### Launch Order (all simultaneous):

```bash
# Lane 1: Infrastructure - Sequential 3-task chain
Task -> rust-expert -> INFRA-004, INFRA-005, INFRA-006

# Lane 2: L3 Engine - Single critical task
Task -> rust-expert -> L3-003

# Lane 3: Bitget - Parallel HTTP + Instruments + WebSocket
Task -> rust-expert -> ADAPT-BITGET-003, ADAPT-BITGET-004, ADAPT-BITGET-005

# Lane 4: Kraken - Quick Python update
Task -> python-dev-expert -> ADAPT-KRAKEN-004
```

---

## Expected Outputs

### INFRA-004 Deliverables
- `crates/iceberg/src/buffer.rs` - Thread-safe ring buffer
- Unit tests for buffer overflow, flush behavior
- Performance benchmarks (throughput/latency)

### INFRA-005 Deliverables
- `crates/iceberg/src/arrow.rs` - RecordBatch conversion
- Schema validation logic
- Memory-efficient batch processing

### INFRA-006 Deliverables
- `crates/iceberg/src/writer.rs` - Parquet writer
- S3 upload integration (object_store crate)
- Compression and partitioning logic

### L3-003 Deliverables
- `crates/l3_engine/src/correlator.rs` - Core correlation algorithm
- Price level tracking
- Trade size matching logic
- Unit tests with synthetic data

### ADAPT-BITGET-003 Deliverables
- `crates/adapters/bitget/src/http/client.rs` - HTTP client
- `crates/adapters/bitget/src/http/endpoints.rs` - API endpoints
- `crates/adapters/bitget/src/http/models.rs` - Request/response models
- HMAC-SHA256 authentication
- Rate limiting (API quotas)

### ADAPT-BITGET-004 Deliverables
- `nautilus_trader/adapters/bitget/providers.py` - Python provider
- `crates/adapters/bitget/src/http/instruments.rs` - Rust fetcher
- Symbol parsing and normalization

### ADAPT-BITGET-005 Deliverables
- `crates/adapters/bitget/src/websocket/client.rs` - WebSocket client
- `crates/adapters/bitget/src/websocket/messages.rs` - Message types
- `crates/adapters/bitget/src/websocket/handler.rs` - Event handler
- Subscription management
- Auto-reconnection with backoff

### ADAPT-KRAKEN-004 Deliverables
- Updated `nautilus_trader/adapters/kraken/data.py`
- Proper subscription handling for orderbook + trades
- Error handling improvements

---

## Success Criteria

✅ All 6 tasks complete without errors
✅ All new code includes unit tests
✅ Zero file conflicts between lanes
✅ All agents report back within expected timeframes
✅ Code passes existing CI checks

---

## Risk Mitigation

**Risk**: Complex tasks (L3-003, INFRA-005) may take longer than estimated
**Mitigation**: Rust experts have full context from previous waves, strong Rust patterns established

**Risk**: Bitget API may have undocumented quirks
**Mitigation**: Agent will use Context7 MCP for latest Bitget API docs

**Risk**: Agent coordination overhead
**Mitigation**: Zero shared files = zero coordination needed

---

## Monitoring

Progress tracked in real-time via:
- Individual agent task outputs (blocking mode)
- Execution progress JSON updates
- Per-lane status tracking

---

**Orchestrator**: Will collect all agent outputs, update execution-progress.json, create wave-3-summary.md
