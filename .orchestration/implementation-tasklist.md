# L3 Orderbook Harvesting System - Implementation Tasklist

## Summary Statistics

| Category | Tasks | Effort |
|----------|-------|--------|
| Infrastructure (INFRA) | 10 | ~28h |
| L3 Engine (L3) | 9 | ~22h |
| Bitget Adapter (ADAPT-BITGET) | 12 | ~28h |
| Kraken Enhancement (ADAPT-KRAKEN) | 5 | ~8h |
| Hyperliquid Enhancement (ADAPT-HL) | 5 | ~8h |
| Harvester Service (HARV) | 10 | ~22h |
| Testing & Docs (TEST/DOC) | 4 | ~12h |
| **TOTAL** | **55 tasks** | **~128h** |

---

## Phase 1: Infrastructure

### Task ID: INFRA-001
**Title**: Create nautilus_iceberg crate skeleton
**Description**: Initialize the new Rust crate for Iceberg integration with Cargo.toml, lib.rs, module structure following existing crate patterns.
**Files/Locations**:
- `crates/iceberg/Cargo.toml`
- `crates/iceberg/src/lib.rs`
- `crates/iceberg/src/config.rs`
- `crates/Cargo.toml` (workspace update)
**Dependencies**: None
**Estimated Effort**: S (1-2h)
**Parallelizable With**: INFRA-002, ADAPT-BITGET-001, L3-001

---

### Task ID: INFRA-002
**Title**: Define Iceberg schema types for L3 events
**Description**: Create Rust structs for the three Iceberg table schemas (orderbook_l3_events, trades, liquidations) with Arrow schema derivation support.
**Files/Locations**:
- `crates/iceberg/src/schema.rs`
- `crates/iceberg/src/types.rs`
**Dependencies**: INFRA-001
**Estimated Effort**: M (2-4h)
**Parallelizable With**: ADAPT-BITGET-001, ADAPT-KRAKEN-001

---

### Task ID: INFRA-003
**Title**: Implement IcebergConfig for Supabase connection
**Description**: Create configuration struct with Supabase project ref, warehouse name, S3 endpoint, catalog URI, and credentials handling.
**Files/Locations**:
- `crates/iceberg/src/config.rs`
**Dependencies**: INFRA-001
**Estimated Effort**: S (1-2h)
**Parallelizable With**: INFRA-002, ADAPT-BITGET-002

---

### Task ID: INFRA-004
**Title**: Implement event batching buffer
**Description**: Create a thread-safe ring buffer that collects events with configurable batch size and flush interval.
**Files/Locations**:
- `crates/iceberg/src/buffer.rs`
**Dependencies**: INFRA-002
**Estimated Effort**: M (2-4h)
**Parallelizable With**: ADAPT-KRAKEN-002, ADAPT-HL-001

---

### Task ID: INFRA-005
**Title**: Implement Arrow RecordBatch conversion
**Description**: Create conversion functions to transform batched events into Arrow RecordBatch format.
**Files/Locations**:
- `crates/iceberg/src/arrow.rs`
**Dependencies**: INFRA-002, INFRA-004
**Estimated Effort**: M (2-4h)
**Parallelizable With**: ADAPT-BITGET-003, ADAPT-KRAKEN-003

---

### Task ID: INFRA-006
**Title**: Implement Parquet file writer with S3 upload
**Description**: Create Parquet file writer that uploads to Supabase S3 endpoint using object_store crate.
**Files/Locations**:
- `crates/iceberg/src/writer.rs`
**Dependencies**: INFRA-003, INFRA-005
**Estimated Effort**: M (2-4h)
**Parallelizable With**: ADAPT-BITGET-004

---

### Task ID: INFRA-007
**Title**: Implement Iceberg catalog integration
**Description**: Create REST catalog client for Supabase Iceberg API with atomic commit protocol.
**Files/Locations**:
- `crates/iceberg/src/catalog.rs`
**Dependencies**: INFRA-006
**Estimated Effort**: L (4-8h)
**Parallelizable With**: ADAPT-BITGET-005

---

### Task ID: INFRA-008
**Title**: Create IcebergSink unified writer
**Description**: Combine buffer, arrow conversion, parquet writer, and catalog into a single IcebergSink struct.
**Files/Locations**:
- `crates/iceberg/src/sink.rs`
- `crates/iceberg/src/lib.rs`
**Dependencies**: INFRA-004, INFRA-006, INFRA-007
**Estimated Effort**: M (2-4h)
**Parallelizable With**: L3-006

---

### Task ID: INFRA-009
**Title**: Add Python bindings for IcebergSink
**Description**: Create PyO3 bindings for IcebergConfig and IcebergSink.
**Files/Locations**:
- `crates/iceberg/src/python/mod.rs`
- `crates/iceberg/src/python/sink.rs`
- `crates/pyo3/src/lib.rs`
**Dependencies**: INFRA-008
**Estimated Effort**: M (2-4h)
**Parallelizable With**: L3-008

---

### Task ID: INFRA-010
**Title**: Write unit tests for Iceberg crate
**Description**: Create comprehensive unit tests for schema conversion, buffering logic, and Parquet writing.
**Files/Locations**:
- `crates/iceberg/src/tests/`
**Dependencies**: INFRA-008
**Estimated Effort**: M (2-4h)
**Parallelizable With**: L3-009

---

## Phase 2: L3 Reconstruction Engine

### Task ID: L3-001
**Title**: Create nautilus_l3_engine crate skeleton
**Description**: Initialize the new Rust crate for L3 reconstruction with dependencies on nautilus_model.
**Files/Locations**:
- `crates/l3_engine/Cargo.toml`
- `crates/l3_engine/src/lib.rs`
- `crates/Cargo.toml`
**Dependencies**: None
**Estimated Effort**: S (1-2h)
**Parallelizable With**: INFRA-001, ADAPT-BITGET-001

---

### Task ID: L3-002
**Title**: Implement synthetic order ID generator
**Description**: Create deterministic order ID generator using FNV or xxHash for fast hashing.
**Files/Locations**:
- `crates/l3_engine/src/order_id.rs`
**Dependencies**: L3-001
**Estimated Effort**: S (1-2h)
**Parallelizable With**: L3-003, INFRA-004

---

### Task ID: L3-003
**Title**: Implement L2 delta correlation engine
**Description**: Create the core algorithm that correlates L2 orderbook deltas with trade events to infer order actions.
**Files/Locations**:
- `crates/l3_engine/src/correlator.rs`
**Dependencies**: L3-001, L3-002
**Estimated Effort**: L (4-8h)
**Parallelizable With**: INFRA-005

---

### Task ID: L3-004
**Title**: Implement local orderbook state manager
**Description**: Create per-instrument state manager that maintains current L3 representation.
**Files/Locations**:
- `crates/l3_engine/src/state.rs`
**Dependencies**: L3-002, L3-003
**Estimated Effort**: M (2-4h)
**Parallelizable With**: INFRA-006

---

### Task ID: L3-005
**Title**: Implement L3 delta event emitter
**Description**: Create output interface that emits synthetic OrderBookDelta events with BookType::L3_MBO.
**Files/Locations**:
- `crates/l3_engine/src/emitter.rs`
**Dependencies**: L3-003, L3-004
**Estimated Effort**: M (2-4h)
**Parallelizable With**: INFRA-007

---

### Task ID: L3-006
**Title**: Create L3ReconstructionEngine main struct
**Description**: Combine correlator, state manager, and emitter into unified engine.
**Files/Locations**:
- `crates/l3_engine/src/engine.rs`
- `crates/l3_engine/src/lib.rs`
**Dependencies**: L3-003, L3-004, L3-005
**Estimated Effort**: M (2-4h)
**Parallelizable With**: INFRA-008

---

### Task ID: L3-007
**Title**: Handle exchange-specific quirks
**Description**: Add exchange-specific configuration for sequence gaps, timing jitter, snapshot frequency.
**Files/Locations**:
- `crates/l3_engine/src/exchange.rs`
- `crates/l3_engine/src/quirks/mod.rs`
**Dependencies**: L3-006
**Estimated Effort**: M (2-4h)
**Parallelizable With**: ADAPT-BITGET-006

---

### Task ID: L3-008
**Title**: Add Python bindings for L3 engine
**Description**: Create PyO3 bindings for L3ReconstructionEngine.
**Files/Locations**:
- `crates/l3_engine/src/python/mod.rs`
- `crates/l3_engine/src/python/engine.rs`
- `crates/pyo3/src/lib.rs`
**Dependencies**: L3-006
**Estimated Effort**: M (2-4h)
**Parallelizable With**: INFRA-009

---

### Task ID: L3-009
**Title**: Write unit tests for L3 engine
**Description**: Create unit tests with synthetic L2+trade data to verify correct L3 inference.
**Files/Locations**:
- `crates/l3_engine/src/tests/`
**Dependencies**: L3-006
**Estimated Effort**: M (2-4h)
**Parallelizable With**: INFRA-010

---

## Phase 3: Exchange Adapters

### Bitget Adapter (NEW)

### Task ID: ADAPT-BITGET-001
**Title**: Create Bitget Python adapter skeleton
**Description**: Initialize adapter directory structure following template pattern.
**Files/Locations**:
- `nautilus_trader/adapters/bitget/__init__.py`
- `nautilus_trader/adapters/bitget/config.py`
- `nautilus_trader/adapters/bitget/constants.py`
- `nautilus_trader/adapters/bitget/enums.py`
**Dependencies**: None
**Estimated Effort**: S (1-2h)
**Parallelizable With**: INFRA-001, L3-001

---

### Task ID: ADAPT-BITGET-002
**Title**: Create Bitget Rust crate skeleton
**Description**: Initialize Rust crate following patterns from nautilus-bybit.
**Files/Locations**:
- `crates/adapters/bitget/Cargo.toml`
- `crates/adapters/bitget/src/lib.rs`
- `crates/adapters/bitget/src/config.rs`
**Dependencies**: ADAPT-BITGET-001
**Estimated Effort**: S (1-2h)
**Parallelizable With**: INFRA-003, L3-002

---

### Task ID: ADAPT-BITGET-003
**Title**: Implement Bitget HTTP client
**Description**: Create REST API client with authentication (HMAC-SHA256), rate limiting, error handling.
**Files/Locations**:
- `crates/adapters/bitget/src/http/client.rs`
- `crates/adapters/bitget/src/http/endpoints.rs`
- `crates/adapters/bitget/src/http/models.rs`
**Dependencies**: ADAPT-BITGET-002
**Estimated Effort**: L (4-8h)
**Parallelizable With**: INFRA-005

---

### Task ID: ADAPT-BITGET-004
**Title**: Implement Bitget instrument provider
**Description**: Create instrument provider that fetches and parses Bitget symbols/contracts.
**Files/Locations**:
- `nautilus_trader/adapters/bitget/providers.py`
- `crates/adapters/bitget/src/http/instruments.rs`
**Dependencies**: ADAPT-BITGET-003
**Estimated Effort**: M (2-4h)
**Parallelizable With**: INFRA-006

---

### Task ID: ADAPT-BITGET-005
**Title**: Implement Bitget WebSocket client
**Description**: Create WebSocket client with subscription management, reconnection, message parsing.
**Files/Locations**:
- `crates/adapters/bitget/src/websocket/client.rs`
- `crates/adapters/bitget/src/websocket/messages.rs`
- `crates/adapters/bitget/src/websocket/handler.rs`
**Dependencies**: ADAPT-BITGET-002
**Estimated Effort**: L (4-8h)
**Parallelizable With**: INFRA-007

---

### Task ID: ADAPT-BITGET-006
**Title**: Implement Bitget orderbook delta parsing
**Description**: Parse WebSocket orderbook messages into OrderBookDelta/OrderBookDeltas.
**Files/Locations**:
- `crates/adapters/bitget/src/websocket/parse.rs`
**Dependencies**: ADAPT-BITGET-005
**Estimated Effort**: M (2-4h)
**Parallelizable With**: L3-007

---

### Task ID: ADAPT-BITGET-007
**Title**: Implement Bitget trade tick parsing
**Description**: Parse WebSocket trade messages into TradeTick.
**Files/Locations**:
- `crates/adapters/bitget/src/websocket/parse.rs`
**Dependencies**: ADAPT-BITGET-005
**Estimated Effort**: S (1-2h)
**Parallelizable With**: ADAPT-BITGET-006

---

### Task ID: ADAPT-BITGET-008
**Title**: Implement Bitget liquidation parsing
**Description**: Parse WebSocket liquidation messages.
**Files/Locations**:
- `crates/adapters/bitget/src/websocket/parse.rs`
- `crates/adapters/bitget/src/common/types.rs`
**Dependencies**: ADAPT-BITGET-005
**Estimated Effort**: S (1-2h)
**Parallelizable With**: ADAPT-BITGET-007

---

### Task ID: ADAPT-BITGET-009
**Title**: Create Bitget Python data client
**Description**: Create BitgetDataClient class extending LiveMarketDataClient.
**Files/Locations**:
- `nautilus_trader/adapters/bitget/data.py`
**Dependencies**: ADAPT-BITGET-004, ADAPT-BITGET-006, ADAPT-BITGET-007, ADAPT-BITGET-008
**Estimated Effort**: M (2-4h)
**Parallelizable With**: L3-008

---

### Task ID: ADAPT-BITGET-010
**Title**: Create Bitget factory functions
**Description**: Create factory module with LiveDataClientFactory integration.
**Files/Locations**:
- `nautilus_trader/adapters/bitget/factories.py`
**Dependencies**: ADAPT-BITGET-009
**Estimated Effort**: S (1-2h)
**Parallelizable With**: ADAPT-BITGET-011

---

### Task ID: ADAPT-BITGET-011
**Title**: Add Bitget PyO3 bindings
**Description**: Create Python bindings for HTTP and WebSocket clients.
**Files/Locations**:
- `crates/adapters/bitget/src/python/mod.rs`
- `crates/adapters/bitget/src/python/http.rs`
- `crates/adapters/bitget/src/python/websocket.rs`
**Dependencies**: ADAPT-BITGET-005, ADAPT-BITGET-003
**Estimated Effort**: M (2-4h)
**Parallelizable With**: ADAPT-BITGET-010

---

### Task ID: ADAPT-BITGET-012
**Title**: Write Bitget adapter tests
**Description**: Create unit tests and mock server tests.
**Files/Locations**:
- `tests/unit_tests/adapters/bitget/`
- `crates/adapters/bitget/src/tests/`
**Dependencies**: ADAPT-BITGET-009, ADAPT-BITGET-010
**Estimated Effort**: M (2-4h)
**Parallelizable With**: HARV-001

---

### Kraken Adapter Enhancement

### Task ID: ADAPT-KRAKEN-001
**Title**: Verify Kraken WebSocket subscribe_book implementation
**Description**: Review and test existing subscribe_book, subscribe_trades methods.
**Files/Locations**:
- `crates/adapters/kraken/src/websocket/client.rs`
- `crates/adapters/kraken/src/websocket/handler.rs`
**Dependencies**: None
**Estimated Effort**: S (1-2h)
**Parallelizable With**: INFRA-002, ADAPT-BITGET-002

---

### Task ID: ADAPT-KRAKEN-002
**Title**: Implement Kraken orderbook delta parsing
**Description**: Complete orderbook delta parsing in handler.rs.
**Files/Locations**:
- `crates/adapters/kraken/src/websocket/parse.rs`
- `crates/adapters/kraken/src/websocket/handler.rs`
**Dependencies**: ADAPT-KRAKEN-001
**Estimated Effort**: M (2-4h)
**Parallelizable With**: INFRA-004

---

### Task ID: ADAPT-KRAKEN-003
**Title**: Implement Kraken trade tick parsing
**Description**: Complete trade parsing to TradeTick in handler.rs.
**Files/Locations**:
- `crates/adapters/kraken/src/websocket/parse.rs`
- `crates/adapters/kraken/src/websocket/handler.rs`
**Dependencies**: ADAPT-KRAKEN-001
**Estimated Effort**: M (2-4h)
**Parallelizable With**: INFRA-005

---

### Task ID: ADAPT-KRAKEN-004
**Title**: Update Kraken Python data client
**Description**: Ensure KrakenDataClient properly handles subscriptions.
**Files/Locations**:
- `nautilus_trader/adapters/kraken/data.py`
**Dependencies**: ADAPT-KRAKEN-002, ADAPT-KRAKEN-003
**Estimated Effort**: S (1-2h)
**Parallelizable With**: ADAPT-BITGET-009

---

### Task ID: ADAPT-KRAKEN-005
**Title**: Write Kraken adapter integration tests
**Description**: Create integration tests for orderbook and trade streaming.
**Files/Locations**:
- `tests/integration_tests/adapters/kraken/`
**Dependencies**: ADAPT-KRAKEN-004
**Estimated Effort**: S (1-2h)
**Parallelizable With**: ADAPT-BITGET-012

---

### Hyperliquid Adapter Enhancement

### Task ID: ADAPT-HL-001
**Title**: Analyze Hyperliquid liquidation stream options
**Description**: Research if public liquidation stream exists vs user-specific events.
**Files/Locations**:
- `crates/adapters/hyperliquid/src/websocket/enums.rs`
- `.orchestration/hyperliquid-liquidations-analysis.md`
**Dependencies**: None
**Estimated Effort**: S (1-2h)
**Parallelizable With**: INFRA-004, ADAPT-KRAKEN-001

---

### Task ID: ADAPT-HL-002
**Title**: Implement Hyperliquid public liquidation subscription
**Description**: Implement subscribe_liquidations method if public channel exists.
**Files/Locations**:
- `crates/adapters/hyperliquid/src/websocket/client.rs`
- `crates/adapters/hyperliquid/src/websocket/messages.rs`
**Dependencies**: ADAPT-HL-001
**Estimated Effort**: M (2-4h)
**Parallelizable With**: ADAPT-KRAKEN-002

---

### Task ID: ADAPT-HL-003
**Title**: Create Hyperliquid liquidation event type
**Description**: Define liquidation event data type and parsing logic.
**Files/Locations**:
- `crates/adapters/hyperliquid/src/common/types.rs`
- `crates/adapters/hyperliquid/src/websocket/parse.rs`
**Dependencies**: ADAPT-HL-002
**Estimated Effort**: S (1-2h)
**Parallelizable With**: ADAPT-KRAKEN-003

---

### Task ID: ADAPT-HL-004
**Title**: Update Hyperliquid Python data client
**Description**: Add subscribe_liquidations method to HyperliquidDataClient.
**Files/Locations**:
- `nautilus_trader/adapters/hyperliquid/data.py`
**Dependencies**: ADAPT-HL-002, ADAPT-HL-003
**Estimated Effort**: S (1-2h)
**Parallelizable With**: ADAPT-KRAKEN-004

---

### Task ID: ADAPT-HL-005
**Title**: Write Hyperliquid liquidation tests
**Description**: Create unit and integration tests for liquidation streaming.
**Files/Locations**:
- `tests/unit_tests/adapters/hyperliquid/`
**Dependencies**: ADAPT-HL-004
**Estimated Effort**: S (1-2h)
**Parallelizable With**: ADAPT-KRAKEN-005

---

## Phase 4: Harvester Service

### Task ID: HARV-001
**Title**: Create harvester Python package structure
**Description**: Initialize harvester module with configuration and service components.
**Files/Locations**:
- `nautilus_trader/harvester/__init__.py`
- `nautilus_trader/harvester/config.py`
**Dependencies**: INFRA-009
**Estimated Effort**: S (1-2h)
**Parallelizable With**: ADAPT-BITGET-012

---

### Task ID: HARV-002
**Title**: Implement harvester configuration system
**Description**: Create HarvesterConfig with venue selection, instrument filters, batching parameters.
**Files/Locations**:
- `nautilus_trader/harvester/config.py`
**Dependencies**: HARV-001
**Estimated Effort**: M (2-4h)
**Parallelizable With**: HARV-003

---

### Task ID: HARV-003
**Title**: Create multi-exchange data client manager
**Description**: Implement DataClientManager that manages multiple exchange data clients.
**Files/Locations**:
- `nautilus_trader/harvester/clients.py`
**Dependencies**: HARV-001, ADAPT-BITGET-010, ADAPT-KRAKEN-004, ADAPT-HL-004
**Estimated Effort**: M (2-4h)
**Parallelizable With**: HARV-002

---

### Task ID: HARV-004
**Title**: Implement subscription orchestrator
**Description**: Create orchestrator for managing subscriptions across all venues.
**Files/Locations**:
- `nautilus_trader/harvester/subscriptions.py`
**Dependencies**: HARV-003
**Estimated Effort**: M (2-4h)
**Parallelizable With**: HARV-005

---

### Task ID: HARV-005
**Title**: Integrate L3 reconstruction with harvester
**Description**: Wire up L3ReconstructionEngine to process incoming data.
**Files/Locations**:
- `nautilus_trader/harvester/l3_processor.py`
**Dependencies**: L3-008, HARV-003
**Estimated Effort**: M (2-4h)
**Parallelizable With**: HARV-004

---

### Task ID: HARV-006
**Title**: Implement event routing to Iceberg sinks
**Description**: Create router for directing events to appropriate table sinks.
**Files/Locations**:
- `nautilus_trader/harvester/router.py`
**Dependencies**: INFRA-009, HARV-005
**Estimated Effort**: M (2-4h)
**Parallelizable With**: HARV-007

---

### Task ID: HARV-007
**Title**: Implement graceful reconnection handling
**Description**: Add reconnection logic with exponential backoff and state recovery.
**Files/Locations**:
- `nautilus_trader/harvester/resilience.py`
**Dependencies**: HARV-003, HARV-004
**Estimated Effort**: M (2-4h)
**Parallelizable With**: HARV-006

---

### Task ID: HARV-008
**Title**: Create OrderbookHarvester main class
**Description**: Combine all components into main class with start/stop methods.
**Files/Locations**:
- `nautilus_trader/harvester/service.py`
- `nautilus_trader/harvester/__init__.py`
**Dependencies**: HARV-004, HARV-005, HARV-006, HARV-007
**Estimated Effort**: M (2-4h)
**Parallelizable With**: HARV-009

---

### Task ID: HARV-009
**Title**: Add monitoring and metrics
**Description**: Implement metrics for throughput, latency, error rates with Prometheus export.
**Files/Locations**:
- `nautilus_trader/harvester/metrics.py`
**Dependencies**: HARV-008
**Estimated Effort**: M (2-4h)
**Parallelizable With**: TEST-001

---

### Task ID: HARV-010
**Title**: Create CLI entry point
**Description**: Create command-line interface for starting harvester.
**Files/Locations**:
- `nautilus_trader/harvester/cli.py`
- `pyproject.toml`
**Dependencies**: HARV-008
**Estimated Effort**: S (1-2h)
**Parallelizable With**: HARV-009

---

## Phase 5: Testing and Documentation

### Task ID: TEST-001
**Title**: Create end-to-end integration tests
**Description**: Write tests that verify full pipeline with mock exchanges.
**Files/Locations**:
- `tests/integration_tests/harvester/`
**Dependencies**: HARV-008
**Estimated Effort**: L (4-8h)
**Parallelizable With**: HARV-009

---

### Task ID: TEST-002
**Title**: Create performance benchmarks
**Description**: Benchmark L3 reconstruction throughput and memory usage.
**Files/Locations**:
- `tests/performance_tests/harvester/`
**Dependencies**: HARV-008
**Estimated Effort**: M (2-4h)
**Parallelizable With**: DOC-001

---

### Task ID: DOC-001
**Title**: Write harvester user documentation
**Description**: Create docs covering setup, configuration, running, monitoring.
**Files/Locations**:
- `docs/harvester/getting-started.md`
- `docs/harvester/configuration.md`
- `docs/harvester/monitoring.md`
**Dependencies**: HARV-010
**Estimated Effort**: M (2-4h)
**Parallelizable With**: TEST-002

---

### Task ID: DOC-002
**Title**: Write Bitget adapter documentation
**Description**: Document Bitget adapter usage and configuration.
**Files/Locations**:
- `docs/integrations/bitget.md`
**Dependencies**: ADAPT-BITGET-012
**Estimated Effort**: S (1-2h)
**Parallelizable With**: DOC-001

---

## Dependency Graph (ASCII)

```
PHASE 1: INFRASTRUCTURE                    PHASE 2: L3 ENGINE
═══════════════════════                    ══════════════════

INFRA-001 ──┬──► INFRA-002 ──► INFRA-004   L3-001 ──► L3-002 ──┬──► L3-003
            │         │              │                         │       │
            └──► INFRA-003           │                         │       ▼
                     │               ▼                         │   L3-004
                     └────────► INFRA-005                      │       │
                                     │                         │       ▼
                                     ▼                         │   L3-005
                                INFRA-006 ◄────────────────────┘       │
                                     │                                 ▼
                                     ▼                             L3-006 ──┬──► L3-007
                                INFRA-007                              │    │
                                     │                                 │    ├──► L3-008
                                     ▼                                 │    └──► L3-009
                                INFRA-008 ◄────────────────────────────┘
                                     │
                                     ▼
                                INFRA-009 ──► INFRA-010


PHASE 3: ADAPTERS
═════════════════

BITGET (NEW):
ADAPT-BITGET-001 ──► -002 ──┬──► -003 ──► -004 ─────────────────────────┐
                            │                                            │
                            └──► -005 ──┬──► -006 ──┐                   │
                                        ├──► -007 ──┼──► -009 ──► -010 ─┼──► -012
                                        └──► -008 ──┘         │         │
                                                              └──► -011 ┘

KRAKEN:
ADAPT-KRAKEN-001 ──┬──► -002 ──┬──► -004 ──► -005
                   └──► -003 ──┘

HYPERLIQUID:
ADAPT-HL-001 ──► -002 ──► -003 ──► -004 ──► -005


PHASE 4: HARVESTER
══════════════════

                    ┌─────────────────────────────────┐
                    │  Depends on: INFRA-009, L3-008, │
                    │  ADAPT-BITGET-010, ADAPT-KRAKEN-004, ADAPT-HL-004
                    └─────────────────────────────────┘
                                     │
                                     ▼
HARV-001 ──┬──► HARV-002 ───────────────────────────┐
           │                                         │
           └──► HARV-003 ──┬──► HARV-004 ───────────┼──► HARV-008 ──┬──► HARV-009
                           │                         │       ▲       │
                           ├──► HARV-005 ────────────┤       │       └──► HARV-010
                           │                         │       │
                           └──► HARV-007 ────────────┘       │
                                                             │
           INFRA-009 ──► HARV-006 ───────────────────────────┘


PHASE 5: TESTING & DOCS
═══════════════════════

HARV-008 ──┬──► TEST-001
           └──► TEST-002 ──► DOC-001

ADAPT-BITGET-012 ──► DOC-002
```

---

## Parallel Execution Lanes

### Maximum Parallelization (6 Concurrent Lanes)

```
LANE 1 (Infrastructure - Critical Path):
────────────────────────────────────────
Week 1-2: INFRA-001 → INFRA-002/003 → INFRA-004/005 → INFRA-006 → INFRA-007 → INFRA-008 → INFRA-009/010

LANE 2 (L3 Engine):
───────────────────
Week 1-2: L3-001 → L3-002 → L3-003 → L3-004/005 → L3-006 → L3-007/008/009

LANE 3 (Bitget Adapter):
────────────────────────
Week 1-3: ADAPT-BITGET-001 → -002 → -003/-005 → -004/-006/-007/-008 → -009 → -010/-011 → -012

LANE 4 (Kraken Enhancement):
────────────────────────────
Week 1: ADAPT-KRAKEN-001 → -002/-003 → -004 → -005

LANE 5 (Hyperliquid Enhancement):
─────────────────────────────────
Week 1: ADAPT-HL-001 → -002 → -003 → -004 → -005

LANE 6 (Harvester - After Lanes 1-5):
─────────────────────────────────────
Week 3-4: HARV-001 → HARV-002/003 → HARV-004/005/006/007 → HARV-008 → HARV-009/010 → TEST/DOC
```

---

## Milestone Delivery Schedule

### Milestone 1: Foundations (Week 1)
**Tasks**: INFRA-001 to INFRA-005, L3-001 to L3-002, ADAPT-BITGET-001/002, ADAPT-KRAKEN-001, ADAPT-HL-001
**Deliverable**: Crate skeletons, basic schemas, research complete

### Milestone 2: Core Engines (Week 2)
**Tasks**: INFRA-006 to INFRA-010, L3-003 to L3-009
**Deliverable**: Working IcebergSink and L3ReconstructionEngine

### Milestone 3: Adapters Complete (Week 3)
**Tasks**: All ADAPT-* tasks
**Deliverable**: Bitget adapter, enhanced Kraken/Hyperliquid

### Milestone 4: Harvester Service (Week 4)
**Tasks**: HARV-001 to HARV-010
**Deliverable**: Multi-exchange harvester with CLI

### Milestone 5: Production Ready (Week 5)
**Tasks**: TEST-001, TEST-002, DOC-001, DOC-002
**Deliverable**: Tested, documented, deployable system

---

## Quick Reference: Task Dependencies

| Task | Blocked By | Blocks |
|------|-----------|--------|
| INFRA-001 | - | INFRA-002, INFRA-003 |
| INFRA-008 | INFRA-004, INFRA-006, INFRA-007 | INFRA-009, HARV-006 |
| L3-003 | L3-001, L3-002 | L3-004, L3-005, L3-006 |
| L3-006 | L3-003, L3-004, L3-005 | L3-007, L3-008, L3-009 |
| ADAPT-BITGET-009 | -004, -006, -007, -008 | -010, HARV-003 |
| HARV-003 | HARV-001, Adapters | HARV-004, HARV-005, HARV-007 |
| HARV-008 | HARV-004, -005, -006, -007 | HARV-009, -010, TEST-001 |
