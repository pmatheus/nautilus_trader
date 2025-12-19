# Wave 4B: Bitget Integration Specification

**Status**: Ready to Execute (waiting for Infrastructure + L3 Engine completion)
**Prerequisites**:
- ✅ Bitget compilation fixes complete (Wave 4A)
- ⏳ Infrastructure INFRA-009 (Python bindings)
- ⏳ L3 Engine L3-008 (Python bindings)

## Context

The Bitget adapter has HTTP and WebSocket clients implemented and compiling cleanly (34 tests passing). Now we need to complete the integration with:
1. Message parsing for orderbook deltas, trades, and liquidations
2. Python data client
3. Factory functions
4. PyO3 bindings
5. Comprehensive tests

## Tasks to Complete

### ADAPT-BITGET-006: Implement Bitget Orderbook Delta Parsing
**Estimated**: 45-60 min
**Files**: `crates/adapters/bitget/src/websocket/parse.rs`

Parse WebSocket orderbook messages into OrderBookDelta/OrderBookDeltas.

**Requirements**:
- Parse Bitget orderbook snapshot messages
- Parse incremental delta messages (bids/asks updates)
- Convert to NautilusTrader OrderBookDelta format
- Handle sequence numbers and timestamps
- Validate data integrity

**Bitget Message Format** (reference their docs):
```json
{
  "action": "snapshot|update",
  "arg": {"instType": "SPOT", "channel": "books", "instId": "BTCUSDT"},
  "data": [{
    "asks": [["43000.5", "1.234"]],
    "bids": [["42999.5", "2.345"]],
    "ts": "1234567890123",
    "seqId": 12345
  }]
}
```

### ADAPT-BITGET-007: Implement Bitget Trade Tick Parsing
**Estimated**: 30 min
**Files**: `crates/adapters/bitget/src/websocket/parse.rs`

Parse WebSocket trade messages into TradeTick.

**Requirements**:
- Parse trade tick messages
- Extract price, quantity, side, timestamp
- Convert to NautilusTrader TradeTick format
- Handle trade IDs

**Bitget Trade Format**:
```json
{
  "action": "snapshot|update",
  "arg": {"instType": "SPOT", "channel": "trade", "instId": "BTCUSDT"},
  "data": [{
    "ts": "1234567890123",
    "px": "43000.5",
    "sz": "1.234",
    "side": "buy|sell",
    "tradeId": "123456789"
  }]
}
```

### ADAPT-BITGET-008: Implement Bitget Liquidation Parsing
**Estimated**: 30 min
**Files**:
- `crates/adapters/bitget/src/websocket/parse.rs`
- `crates/adapters/bitget/src/common/types.rs`

Parse WebSocket liquidation messages.

**Requirements**:
- Define `BitgetLiquidation` struct in common/types.rs
- Parse liquidation messages
- Extract instrument, side, price, quantity, timestamp
- Handle futures vs spot liquidations

### ADAPT-BITGET-009: Create Bitget Python Data Client
**Estimated**: 45-60 min
**Files**: `nautilus_trader/adapters/bitget/data.py`

Create BitgetDataClient class extending LiveMarketDataClient.

**Requirements**:
- Extend `LiveMarketDataClient` base class
- Implement subscription methods:
  - `subscribe_order_book_deltas()`
  - `subscribe_trade_ticks()`
  - `subscribe_liquidations()`
- Handle reconnection and state recovery
- Integrate with Rust WebSocket client via PyO3

**Reference Pattern**:
- `nautilus_trader/adapters/bybit/data.py`
- `nautilus_trader/adapters/kraken/data.py`

### ADAPT-BITGET-010: Create Bitget Factory Functions
**Estimated**: 20-30 min
**Files**: `nautilus_trader/adapters/bitget/factories.py`

Create factory module with LiveDataClientFactory integration.

**Requirements**:
- `get_bitget_http_client()` factory
- `get_bitget_websocket_client()` factory
- `get_bitget_data_client()` factory
- `get_bitget_instrument_provider()` factory
- Registration with NautilusTrader factory system

**Reference Pattern**:
- `nautilus_trader/adapters/bybit/factories.py`

### ADAPT-BITGET-011: Add Bitget PyO3 Bindings
**Estimated**: 45-60 min
**Files**:
- `crates/adapters/bitget/src/python/mod.rs`
- `crates/adapters/bitget/src/python/http.rs`
- `crates/adapters/bitget/src/python/websocket.rs`
- Update `crates/pyo3/src/lib.rs`

Create Python bindings for HTTP and WebSocket clients.

**Requirements**:
- PyO3 wrapper for `BitgetHttpClient`
- PyO3 wrapper for `BitgetWebSocketClient`
- Async method handling with pyo3-asyncio
- Proper error conversion to Python exceptions
- Documentation strings and type hints

**Reference Pattern**:
- `crates/adapters/bybit/src/python/`
- `crates/adapters/kraken/src/python/`

### ADAPT-BITGET-012: Write Bitget Adapter Tests
**Estimated**: 60-90 min
**Files**:
- `tests/unit_tests/adapters/bitget/test_parsing.py`
- `tests/unit_tests/adapters/bitget/test_data_client.py`
- `crates/adapters/bitget/src/tests/websocket_tests.rs`
- `crates/adapters/bitget/src/tests/http_tests.rs`

Create unit tests and mock server tests.

**Test Coverage**:
1. **Rust Tests** (`src/tests/`):
   - HTTP client requests/responses
   - WebSocket message parsing
   - Error handling
   - Reconnection logic

2. **Python Tests** (`tests/unit_tests/`):
   - Data client subscription lifecycle
   - Message routing
   - Error handling
   - Factory creation

**Testing Strategy**:
- Use mock HTTP servers (wiremock or similar)
- Use mock WebSocket servers
- Test with real Bitget message samples
- Validate data integrity end-to-end

## Success Criteria

1. ✅ Clean compilation: `cargo build -p nautilus-bitget`
2. ✅ All Rust tests pass: `cargo test -p nautilus-bitget` (target: 50+ tests)
3. ✅ All Python tests pass: `pytest tests/unit_tests/adapters/bitget/`
4. ✅ No clippy warnings: `cargo clippy -p nautilus-bitget`
5. ✅ Python client instantiates: `from nautilus_trader.adapters.bitget import BitgetDataClient`
6. ✅ Can subscribe to data: `client.subscribe_order_book_deltas(instrument_id)`
7. ✅ Documentation complete

## Integration Points

**After Completion**:
- Bitget adapter fully operational
- Ready for Harvester integration (HARV-003)
- Can be used standalone for Bitget data collection

## Time Estimate
Total: 1.5-2 hours

## Execution Strategy

**Sequential Order**:
1. ADAPT-BITGET-006, 007, 008 (Parsing) - Can be parallel
2. ADAPT-BITGET-011 (PyO3 bindings) - Needs parsing
3. ADAPT-BITGET-009 (Python client) - Needs PyO3
4. ADAPT-BITGET-010 (Factory) - Needs Python client
5. ADAPT-BITGET-012 (Tests) - Validates everything

**Optimal Approach**:
Use a single agent (rust-expert or python-dev-expert) to do all tasks sequentially, as they have strong dependencies.

## Notes

- Bitget API documentation: https://www.bitget.com/api-doc/spot/websocket/overview
- Existing Bitget code: `crates/adapters/bitget/`
- Reference implementations: Bybit and Kraken adapters
- All 34 existing tests are passing after Wave 4A fixes
