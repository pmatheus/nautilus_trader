# NautilusTrader Exchange Implementation - Detailed Execution Plan

## Status Overview (Current State)

### ✅ Phase 1 Complete - Foundations Built

| Exchange | Task | Status | What's Done | What's Needed |
|----------|------|--------|-------------|---------------|
| **Kraken** | Task 1 | 🟡 Foundation | Market data complete (HTTP + WebSocket), parsers working | **Execution layer 100% missing**: HTTP endpoints, WebSocket handlers, Python execution client (~30-44 hours) |
| **Coinbase** | Task 3 | 🟡 Foundation | HTTP client, authentication (CB-ACCESS-*), core structure | WebSocket client, parsers, Python client logic (~36-50 hours) |
| **Bitget Spot** | Task 4 | 🟡 Foundation | Authentication, common module (70% reusable), HTTP framework | WebSocket, parsers, data/exec clients (~36-50 hours) |
| **Kucoin Spot** | Task 6 | 🟡 Foundation | **Token-based WebSocket system complete**, authentication, HTTP client | WebSocket message handling, parsers, Python clients (~7-11 days) |
| **Aster DEX** | Task 8 | 🟡 Foundation | **EVM signing complete**, API research (815-line report), structure | HTTP client, WebSocket, data/exec modules (~3-6 weeks) |

### 🚧 Phase 2 Pending - Futures Extensions

| Exchange | Task | Dependencies | Estimated Effort |
|----------|------|--------------|------------------|
| **Hyperliquid** | Task 2 | None | Audit + complete (~2-4 days) |
| **Bitget Futures** | Task 5 | Task 4 (Spot) ✅ | ~4-6 days (70% code reuse) |
| **Kucoin Futures** | Task 7 | Task 6 (Spot) ✅ | ~4-6 days (reuse token system) |

### 📋 Phase 3 Pending - Testing & Documentation

| Task | Status | Estimated Effort |
|------|--------|------------------|
| Task 9: Integration tests | Not started | ~3-5 days |
| Task 10: Documentation | Not started | ~2-3 days |

---

## Detailed Implementation Roadmap

### TASK 1: Complete Kraken Adapter (Spot + Futures)

**Location:**
- Rust: `/home/user/nautilus_trader/crates/adapters/kraken/`
- Python: `/home/user/nautilus_trader/nautilus_trader/adapters/kraken/`

**Current State:**
- ✅ HTTP client: Public endpoints (market data) complete
- ✅ WebSocket client: Market data (orderbook, trades, quotes) complete
- ✅ Parsers: OrderBook, Trade, Quote complete
- ✅ Python DataClient: Working
- ❌ **Execution layer: COMPLETELY MISSING**

**What to Implement:**

#### 1. Rust HTTP Execution Endpoints (`crates/adapters/kraken/src/http/client.rs`)
```rust
// Add these methods to KrakenHttpClient:

// Order management
pub async fn place_order(&self, params: OrderRequest) -> Result<OrderResponse>
pub async fn cancel_order(&self, order_id: &str) -> Result<CancelResponse>
pub async fn cancel_all_orders(&self) -> Result<()>
pub async fn modify_order(&self, order_id: &str, params: ModifyRequest) -> Result<OrderResponse>

// Order queries
pub async fn query_order(&self, order_id: &str) -> Result<OrderStatus>
pub async fn query_open_orders(&self) -> Result<Vec<Order>>
pub async fn query_closed_orders(&self, params: QueryParams) -> Result<Vec<Order>>

// Account
pub async fn query_balances(&self) -> Result<HashMap<String, Balance>>
pub async fn query_trades(&self, params: QueryParams) -> Result<Vec<UserTrade>>

// Futures specific
pub async fn query_positions(&self) -> Result<Vec<Position>>
```

**API Reference:**
- Spot orders: https://docs.kraken.com/rest/#tag/Spot-Trading
- Futures: https://docs.kraken.com/rest/#tag/Futures-Trading

#### 2. Rust Models (`crates/adapters/kraken/src/http/models.rs`)
```rust
// Add these structs:
pub struct OrderRequest { /* ... */ }
pub struct OrderResponse { /* ... */ }
pub struct OrderStatus { /* ... */ }
pub struct BalanceInfo { /* ... */ }
pub struct PositionInfo { /* ... */ }
pub struct UserTrade { /* ... */ }
```

#### 3. WebSocket Execution Handlers (`crates/adapters/kraken/src/websocket/`)
Add parsers for:
- `executions` channel → order updates, fills
- `balances` channel → balance changes (futures)

#### 4. Python Execution Client (`nautilus_trader/adapters/kraken/execution.py`)
Create new file with `KrakenExecutionClient` class:
```python
class KrakenExecutionClient(LiveExecutionClient):
    async def _submit_order(self, command: SubmitOrder) -> None
    async def _modify_order(self, command: ModifyOrder) -> None
    async def _cancel_order(self, command: CancelOrder) -> None
    async def _cancel_all_orders(self, command: CancelAllOrders) -> None

    def generate_order_status_report(self, order: Order) -> OrderStatusReport
    def generate_order_status_reports(self) -> list[OrderStatusReport]
    def generate_fill_reports(self) -> list[FillReport]
    def generate_position_status_reports(self) -> list[PositionStatusReport]
```

**Reference:** Study `nautilus_trader/adapters/binance/execution.py` (2000+ lines)

**Estimated Effort:** 30-44 hours

---

### TASK 2: Complete Hyperliquid Adapter

**Location:**
- Rust: `/home/user/nautilus_trader/crates/adapters/hyperliquid/`
- Python: `/home/user/nautilus_trader/nautilus_trader/adapters/hyperliquid/`

**What to Do:**
1. Audit existing implementation (check websocket/post.rs)
2. Complete missing HTTP endpoints
3. Complete WebSocket POST for order placement
4. Add EVM wallet signing if needed
5. Complete Python data and execution clients
6. Test with testnet

**Key Files:**
- `/home/user/nautilus_trader/crates/adapters/hyperliquid/src/websocket/post.rs` (unique POST over WebSocket)

**API Docs:** https://hyperliquid.gitbook.io/hyperliquid-docs/

**Estimated Effort:** 2-4 days

---

### TASK 3: Complete Coinbase Spot Adapter

**Location:**
- Rust: `/home/user/nautilus_trader/crates/adapters/coinbase/`
- Python: `/home/user/nautilus_trader/nautilus_trader/adapters/coinbase/`

**Current State:**
- ✅ HTTP client with CB-ACCESS-* authentication
- ✅ Basic endpoints (products, accounts, orders, fills)
- ✅ Configuration and Python structure
- ❌ WebSocket client (stub only)
- ❌ Parsers (stub only)
- ❌ Python client logic (stub only)

**What to Implement:**

#### 1. WebSocket Client (`crates/adapters/coinbase/src/websocket/`)
```rust
// Implement in websocket/client.rs:
pub struct CoinbaseWebSocketClient {
    // Connection management
    pub async fn connect() -> Result<Self>
    pub async fn subscribe(&mut self, channels: Vec<Channel>) -> Result<()>
    pub async fn handle_message(&mut self, msg: Message) -> Result<()>
}
```

**WebSocket Channels:**
- `ticker` → QuoteTick
- `level2` → OrderBookDelta
- `matches` → TradeTick
- `user` → Order updates
- `full` → Full orderbook

#### 2. Data Parsers (`crates/adapters/coinbase/src/data/`)
Parse WebSocket messages to Nautilus types:
- Ticker → `TradeTick`, `QuoteTick`
- Level2 → `OrderBookDelta`
- Matches → `TradeTick`

#### 3. Execution Parsers (`crates/adapters/coinbase/src/execution/`)
Parse execution messages:
- Order status → `OrderStatusReport`
- Match (fill) → `FillReport`
- Account → `AccountState`

#### 4. Python Clients (`nautilus_trader/adapters/coinbase/`)
Complete these stub files:
- `data.py`: Implement `CoinbaseDataClient` (connect, subscribe, handle messages)
- `execution.py`: Implement `CoinbaseExecutionClient` (order management)
- `providers.py`: Implement `CoinbaseInstrumentProvider` (load instruments)

**API Docs:** https://docs.cloud.coinbase.com/exchange/docs

**Estimated Effort:** 36-50 hours

---

### TASK 4: Complete Bitget Spot Adapter

**Location:**
- Rust: `/home/user/nautilus_trader/crates/adapters/bitget/`
- Python: `/home/user/nautilus_trader/nautilus_trader/adapters/bitget/`

**Current State:**
- ✅ Authentication (HMAC SHA256)
- ✅ Common module (70% reusable for futures)
- ✅ HTTP client framework
- ✅ Configuration
- ❌ WebSocket client (stub)
- ❌ Parsers (stub)
- ❌ Python clients (stubs)

**What to Implement:**

#### 1. Expand HTTP Client (`crates/adapters/bitget/src/http/client.rs`)
Add endpoints:
- Market data: `/api/v2/spot/market/tickers`, `/api/v2/spot/market/orderbook`, `/api/v2/spot/market/fills`
- Trading: `/api/v2/spot/trade/place-order`, `/api/v2/spot/trade/cancel-order`
- Account: `/api/v2/spot/account/info`, `/api/v2/spot/account/assets`

#### 2. WebSocket Client (`crates/adapters/bitget/src/websocket/client.rs`)
Implement:
- Connection to `wss://ws.bitget.com/v2/ws/public` and `/private`
- Subscription management
- Heartbeat (30s interval)
- Message parsing

**Channels:**
- Public: `spot/ticker`, `spot/depth`, `spot/trade`, `spot/candle`
- Private: `spot/orders`, `spot/account`

#### 3. Parsers (`crates/adapters/bitget/src/data/` and `execution/`)
- Ticker → QuoteTick
- Depth → OrderBookDelta
- Trade → TradeTick
- Orders → OrderStatusReport
- Account → AccountState

#### 4. Python Clients
Complete implementation in:
- `data.py`: Market data subscriptions
- `execution.py`: Order management
- `providers.py`: Instrument loading

**API Docs:** https://bitgetlimited.github.io/apidoc/en/spot/

**Estimated Effort:** 36-50 hours

---

### TASK 5: Implement Bitget Futures

**Location:** Same as Task 4 (`crates/adapters/bitget/`)

**Dependencies:** Task 4 complete ✅

**Approach:** Extend existing Bitget adapter with futures support

**What to Implement:**

#### 1. Research
- Futures API: https://bitgetlimited.github.io/apidoc/en/mix/
- Understand USDT-margined vs Coin-margined
- Document position management
- Document funding rates

#### 2. HTTP Futures Endpoints
Add to `http/client.rs`:
- `/api/v2/mix/market/contracts` (instruments)
- `/api/v2/mix/market/ticker`
- `/api/v2/mix/order/placeOrder`
- `/api/v2/mix/position/allPosition`
- `/api/v2/mix/account/account`

#### 3. WebSocket Futures Channels
- `futures/ticker`, `futures/depth`, `futures/trade`
- `futures/orders`, `futures/positions`, `futures/account`

#### 4. Extend Python Clients
Update to handle both spot and futures:
- InstrumentProvider: Load futures instruments
- DataClient: Subscribe to futures data
- ExecutionClient: Futures order management, position tracking

**Code Reuse:** ~70% from Task 4 (common module, authentication)

**Estimated Effort:** 4-6 days

---

### TASK 6: Complete Kucoin Spot Adapter

**Location:**
- Rust: `/home/user/nautilus_trader/crates/adapters/kucoin/`
- Python: `/home/user/nautilus_trader/nautilus_trader/adapters/kucoin/`

**Current State:**
- ✅ **Token-based WebSocket system COMPLETE** (unique feature)
- ✅ HTTP client with token acquisition
- ✅ Authentication (HMAC + encrypted passphrase)
- ❌ WebSocket message handling (stub)
- ❌ Parsers (stub)
- ❌ Python clients (stubs)

**What to Implement:**

#### 1. Complete WebSocket Client (`crates/adapters/kucoin/src/websocket/client.rs`)
The token acquisition is done. Now implement:
- Connection with welcome message handling
- Subscription management
- **Ping/pong (18s intervals)**
- Message routing
- Token refresh (24hr expiry)

#### 2. Data Parsers (`crates/adapters/kucoin/src/data/`)
Parse WebSocket messages:
- `/market/ticker` → QuoteTick, TradeTick
- `/market/level2` → OrderBookDelta
- `/market/match` → TradeTick
- `/market/candles` → Bar

#### 3. Execution Parsers (`crates/adapters/kucoin/src/execution/`)
- `/spotMarket/tradeOrders` → OrderStatusReport
- `/account/balance` → AccountState

#### 4. Python Clients
Complete implementation:
- `providers.py`: Load instruments from `/api/v1/symbols`
- `data.py`: Market data subscriptions
- `execution.py`: Order management

**API Docs:** https://docs.kucoin.com/

**Estimated Effort:** 7-11 days

---

### TASK 7: Implement Kucoin Futures

**Location:** Same as Task 6 (`crates/adapters/kucoin/`)

**Dependencies:** Task 6 complete ✅

**Important:** Kucoin Futures uses **different base URLs**:
- REST: `https://api-futures.kucoin.com`
- WebSocket: May use same token system or different - **verify this first**

**What to Implement:**

#### 1. Research
- Futures API: https://docs.kucoin.com/futures/
- **Verify if token acquisition is same as spot or different**
- Document position management
- Document funding rates

#### 2. Extend HTTP Client
Add futures endpoints (different base URL):
- `/api/v1/contracts/active` (instruments)
- `/api/v1/ticker`
- `/api/v1/orders` (place/cancel)
- `/api/v1/positions`

#### 3. WebSocket (verify token system)
- Get futures WebSocket token (may be `/api/v1/bullet-private` on futures URL)
- Subscribe to futures channels
- Position updates
- Funding rate updates

#### 4. Extend Python Clients
Update for futures support:
- InstrumentProvider: Futures instruments
- DataClient: Futures market data
- ExecutionClient: Position management

**Code Reuse:** Token system from Task 6 (if same)

**Estimated Effort:** 4-6 days

---

### TASK 8: Complete Aster DEX Adapter

**Location:**
- Rust: `/home/user/nautilus_trader/crates/adapters/aster/`
- Python: `/home/user/nautilus_trader/nautilus_trader/adapters/aster/`

**Current State:**
- ✅ **EVM ECDSA signing COMPLETE** (production-ready)
- ✅ API research complete (815-line report: `ASTER_IMPLEMENTATION_REPORT.md`)
- ✅ Structure created
- ❌ HTTP client (stub)
- ❌ WebSocket client (stub)
- ❌ Parsers (stub)
- ❌ Python clients (stubs)

**What to Implement:**

#### 1. HTTP Client (`crates/adapters/aster/src/http/client.rs`)
Use the signing module (`signing/mod.rs`) to implement:
- `/fapi/v1/exchangeInfo` (instruments)
- `/fapi/v1/ticker/24hr`
- `/fapi/v1/depth`
- `/fapi/v1/order` (place/cancel)
- `/fapi/v1/positionRisk`
- `/fapi/v1/account`

**Authentication:**
```rust
use crate::signing::sign_request;

let params = BTreeMap::new();
params.insert("symbol", "BTCUSDT");
params.insert("nonce", get_microsecond_timestamp());

let signature = sign_request(&params, &private_key)?;
params.insert("signature", signature);
```

#### 2. WebSocket Client (`crates/adapters/aster/src/websocket/`)
- Connect to `wss://fstream.asterdex.com`
- Subscribe to streams (Binance-compatible)
- Handle 24hr connection limit
- 60-minute keep-alive

**Streams:**
- `<symbol>@ticker`
- `<symbol>@depth`
- `<symbol>@trade`
- User data stream (requires listenKey)

#### 3. Data & Execution Modules
Implement parsers and logic in:
- `data/mod.rs`: Market data handling
- `execution/mod.rs`: Order and position management

#### 4. Python Bindings & Clients
- PyO3 exports in `python/mod.rs`
- Complete Python clients (providers, data, execution)

**API Docs:** https://docs.asterdex.com/product/aster-perpetual-pro/api

**Reference:** `ASTER_IMPLEMENTATION_REPORT.md` (comprehensive API documentation)

**Estimated Effort:** 3-6 weeks

---

### TASK 9: Integration Tests

**Location:** `/home/user/nautilus_trader/tests/integration_tests/adapters/`

**For Each Exchange:**

#### 1. Create Test Directory Structure
```
tests/integration_tests/adapters/<exchange>/
├── __init__.py
├── test_http.py
├── test_websocket.py
├── test_providers.py
├── test_data.py
├── test_execution.py
├── resources/
│   ├── http_responses/
│   └── websocket_messages/
```

#### 2. Unit Tests (Rust)
For each adapter, add tests in `crates/adapters/<exchange>/src/`:
- HTTP request building
- Signature generation
- Message parsing
- Error handling

Example:
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_ticker() { /* ... */ }

    #[test]
    fn test_signature_generation() { /* ... */ }
}
```

#### 3. Integration Tests (Python)
Mock HTTP responses and WebSocket messages:
```python
def test_instrument_provider_load(mocker):
    # Mock HTTP response
    # Test instrument loading

def test_data_client_subscribe(mocker):
    # Mock WebSocket
    # Test subscriptions

def test_execution_client_submit_order(mocker):
    # Mock order placement
    # Verify request format
```

#### 4. Test Fixtures
Create realistic mock data in `resources/`:
- HTTP responses (JSON)
- WebSocket messages (JSON)
- Test instruments
- Test orders

**Reference Tests:**
- `tests/integration_tests/adapters/binance/`
- `tests/integration_tests/adapters/bybit/`

**Estimated Effort:** 3-5 days

---

### TASK 10: Documentation

**Location:** `/home/user/nautilus_trader/docs/integrations/`

**For Each Exchange, Create:**

#### 1. Integration Guide (`docs/integrations/<exchange>.md`)

Template:
```markdown
# <Exchange> Integration

## Overview
- Supported products: Spot, Futures, Perpetuals
- Account types
- API version

## Setup
### 1. Create Account
### 2. Generate API Keys
### 3. Configure NautilusTrader

## Configuration

```python
from nautilus_trader.adapters.<exchange>.config import <Exchange>DataClientConfig

config = <Exchange>DataClientConfig(
    api_key="your_key",
    api_secret="your_secret",
)
```

## Examples

### Market Data
```python
# Subscribe to orderbook
```

### Order Execution
```python
# Place order
```

## Supported Features
- Order types: Market, Limit, Stop
- Time in Force: GTC, IOC, FOK
- Rate limits: X req/s

## Known Limitations
- ...

## Testnet
- URL: ...
- How to use
```

#### 2. Update Main Docs
- Update `docs/integrations/index.md` with new exchanges
- Update `README.md` feature list
- Update comparison tables

#### 3. Example Scripts
Create in `examples/live/<exchange>/`:
- `data_stream.py` - Market data example
- `simple_order.py` - Order placement example
- `strategy.py` - Simple strategy example

**Reference:**
- `docs/integrations/binance.md`
- `docs/integrations/bybit.md`
- `examples/live/binance/`

**Estimated Effort:** 2-3 days

---

## Quick Start Guide (For You)

### Step 1: Choose Priority Exchanges

I recommend this order based on ease and value:
1. **Kucoin Spot** (Task 6) - Token system done, just need message handling
2. **Bitget Spot** (Task 4) - Foundation solid, straightforward API
3. **Coinbase Spot** (Task 3) - Major exchange, good docs
4. **Kraken** (Task 1) - Data layer done, just need execution
5. **Hyperliquid** (Task 2) - Audit first to see state
6. **Bitget Futures** (Task 5) - After Bitget Spot
7. **Kucoin Futures** (Task 7) - After Kucoin Spot
8. **Aster** (Task 8) - Last (new exchange, less critical)

### Step 2: For Each Exchange

1. **Read the existing code** to understand current state
2. **Implement WebSocket client** (most critical for live trading)
3. **Implement parsers** (convert exchange data → Nautilus types)
4. **Complete Python clients** (wire everything together)
5. **Test with sandbox/testnet**
6. **Add integration tests**
7. **Write documentation**

### Step 3: Build & Test Commands

```bash
# Build specific adapter
cd /home/user/nautilus_trader
cargo build -p nautilus-<exchange>

# Run Rust tests
cargo test -p nautilus-<exchange>

# Run Python tests
pytest tests/integration_tests/adapters/<exchange>/

# Build all
cargo build --workspace

# Test all
cargo test --workspace
pytest tests/
```

### Step 4: Commit Strategy

Commit frequently with clear messages:
```bash
git add crates/adapters/<exchange>/
git commit -m "feat(<exchange>): implement WebSocket client for market data"

git add nautilus_trader/adapters/<exchange>/
git commit -m "feat(<exchange>): complete Python data client"
```

---

## Critical Files Reference

### Rust Adapter Structure (Standard)
```
crates/adapters/<exchange>/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs                 # Re-exports
│   ├── config.rs              # Configuration structs
│   ├── common/
│   │   ├── credential.rs      # Authentication
│   │   ├── enums.rs           # Exchange enums
│   │   ├── models.rs          # Data structures
│   │   ├── urls.rs            # URL helpers
│   │   └── consts.rs          # Constants
│   ├── http/
│   │   ├── client.rs          # HTTP client
│   │   ├── error.rs           # HTTP errors
│   │   └── models.rs          # Request/response models
│   ├── websocket/
│   │   ├── client.rs          # WebSocket client
│   │   ├── error.rs           # WS errors
│   │   └── messages.rs        # WS message types
│   ├── data/
│   │   └── mod.rs             # Data parsers
│   ├── execution/
│   │   └── mod.rs             # Execution parsers
│   └── python/
│       └── mod.rs             # PyO3 bindings
```

### Python Adapter Structure (Standard)
```
nautilus_trader/adapters/<exchange>/
├── __init__.py                # Package exports
├── constants.py               # VENUE, CLIENT_ID
├── config.py                  # Config classes
├── providers.py               # InstrumentProvider
├── data.py                    # DataClient
├── execution.py               # ExecutionClient
└── factories.py               # Factory classes
```

---

## Key Implementation Patterns

### 1. WebSocket Client Pattern (Rust)
```rust
pub struct ExchangeWebSocketClient {
    stream: WebSocketStream<...>,
    subscriptions: HashMap<String, ...>,
}

impl ExchangeWebSocketClient {
    pub async fn connect(url: &str) -> Result<Self>;
    pub async fn subscribe(&mut self, channel: Channel) -> Result<()>;
    pub async fn handle_message(&mut self, msg: Message) -> Result<()>;
    pub async fn ping(&mut self) -> Result<()>;
}
```

### 2. HTTP Client Pattern (Rust)
```rust
pub struct ExchangeHttpClient {
    client: reqwest::Client,
    credential: Option<Credential>,
    base_url: String,
}

impl ExchangeHttpClient {
    pub async fn get<T>(&self, endpoint: &str) -> Result<T>;
    pub async fn post<T>(&self, endpoint: &str, body: Value) -> Result<T>;
    fn sign_request(&self, method: &str, endpoint: &str, body: &str) -> String;
}
```

### 3. Python DataClient Pattern
```python
class ExchangeDataClient(LiveMarketDataClient):
    def __init__(self, http_client, ws_client, config):
        self._http_client = http_client
        self._ws_client = ws_client

    async def _connect(self):
        await self._ws_client.connect()

    async def _subscribe(self, data_type: DataType):
        # Convert Nautilus subscription → Exchange channel
        # Send subscription via WebSocket

    def _handle_ws_message(self, msg):
        # Parse message
        # Convert to Nautilus type
        # Publish via self._handle_data()
```

### 4. Python ExecutionClient Pattern
```python
class ExchangeExecutionClient(LiveExecutionClient):
    async def _submit_order(self, command: SubmitOrder):
        # Convert Nautilus order → Exchange order format
        # Send via HTTP or WebSocket
        # Parse response → OrderStatusReport
        # Publish via self.generate_order_submitted()

    async def _cancel_order(self, command: CancelOrder):
        # Send cancel request
        # Parse response
        # Publish via self.generate_order_canceled()
```

---

## Common Authentication Patterns

### HMAC SHA256 (Binance, Bitget, Kucoin)
```rust
use hmac::{Hmac, Mac};
use sha2::Sha256;

fn sign(secret: &str, message: &str) -> String {
    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(message.as_bytes());
    let result = mac.finalize();
    hex::encode(result.into_bytes())
}

// Usage:
let signature = sign(secret, &format!("{}{}{}{}", timestamp, method, path, body));
```

### Coinbase (CB-ACCESS-*)
```rust
headers.insert("CB-ACCESS-KEY", api_key);
headers.insert("CB-ACCESS-SIGN", signature);
headers.insert("CB-ACCESS-TIMESTAMP", timestamp);
headers.insert("CB-ACCESS-PASSPHRASE", passphrase);
```

### EVM/ECDSA (Aster, Hyperliquid)
```rust
use ethers::signers::{LocalWallet, Signer};

let wallet = LocalWallet::from_bytes(&private_key)?;
let hash = keccak256(message);
let signature = wallet.sign_hash(hash)?;
```

---

## Testing Checklist (Per Exchange)

- [ ] Rust unit tests for signature generation
- [ ] Rust unit tests for message parsing
- [ ] Python integration test for instrument loading
- [ ] Python integration test for market data subscription
- [ ] Python integration test for order placement
- [ ] Python integration test for order cancellation
- [ ] Python integration test for balance queries
- [ ] Python integration test for position queries (futures)
- [ ] Manual test with sandbox/testnet
- [ ] Performance test (latency, throughput)
- [ ] Error handling test (network errors, API errors)
- [ ] Reconnection test (WebSocket disconnect)

---

## Build System Checklist (Per Exchange)

When adding new exchange:

1. [ ] Create `crates/adapters/<exchange>/Cargo.toml`
2. [ ] Add to workspace members in root `Cargo.toml`:
   ```toml
   members = [
       # ...
       "crates/adapters/<exchange>",
   ]
   ```
3. [ ] Add to workspace dependencies:
   ```toml
   [workspace.dependencies]
   nautilus-<exchange> = { path = "crates/adapters/<exchange>" }
   ```
4. [ ] Add to pyo3 adapter dependencies in `crates/pyo3/Cargo.toml`:
   ```toml
   [dependencies]
   nautilus-<exchange> = { workspace = true, optional = true }

   [features]
   extension-module = ["<exchange>", ...]
   <exchange> = ["dep:nautilus-<exchange>"]
   ```
5. [ ] Register Python module in `crates/pyo3/src/lib.rs`:
   ```rust
   #[cfg(feature = "<exchange>")]
   m.add_wrapped(wrap_pymodule!(nautilus_<exchange>::python::create_module))?;
   ```
6. [ ] Create Python package `nautilus_trader/adapters/<exchange>/`
7. [ ] Export in `nautilus_trader/adapters/<exchange>/__init__.py`

---

## Useful Commands

```bash
# Check specific adapter compiles
cargo check -p nautilus-<exchange>

# Build specific adapter
cargo build -p nautilus-<exchange>

# Test specific adapter
cargo test -p nautilus-<exchange>

# Format code
cargo fmt -p nautilus-<exchange>

# Lint code
cargo clippy -p nautilus-<exchange>

# Build Python wheel
maturin develop --release

# Run Python tests
pytest tests/integration_tests/adapters/<exchange>/ -v

# Check all adapters
cargo check --workspace

# Build everything
cargo build --workspace --release

# Full test suite
cargo test --workspace
pytest tests/
```

---

## Files Created by Phase 1

### Kraken (Partial)
- Location: `crates/adapters/kraken/`, `nautilus_trader/adapters/kraken/`
- Status: Data layer complete, execution missing

### Coinbase (Foundation)
- Location: `crates/adapters/coinbase/`, `nautilus_trader/adapters/coinbase/`
- Files: 29 Rust + 7 Python = 36 files
- Status: HTTP complete, WebSocket/parsers needed

### Bitget (Foundation)
- Location: `crates/adapters/bitget/`, `nautilus_trader/adapters/bitget/`
- Files: 20 Rust + 8 Python = 28 files
- Status: Authentication complete, 70% reusable for futures

### Kucoin (Foundation)
- Location: `crates/adapters/kucoin/`, `nautilus_trader/adapters/kucoin/`
- Files: 25 Rust + 7 Python = 32 files
- Status: Token system complete (unique feature working!)

### Aster (Foundation)
- Location: `crates/adapters/aster/`, `nautilus_trader/adapters/aster/`
- Files: 16 Rust + 7 Python = 23 files
- Status: EVM signing complete, API research done (815-line report)
- Report: `ASTER_IMPLEMENTATION_REPORT.md`

---

## API Documentation Links

| Exchange | REST API Docs | WebSocket Docs | Testnet |
|----------|---------------|----------------|---------|
| **Kraken** | https://docs.kraken.com/rest/ | https://docs.kraken.com/websockets/ | Yes |
| **Coinbase** | https://docs.cloud.coinbase.com/exchange/docs | https://docs.cloud.coinbase.com/exchange/docs/websocket-overview | Yes (sandbox) |
| **Bitget** | https://bitgetlimited.github.io/apidoc/en/spot/ | https://bitgetlimited.github.io/apidoc/en/spot/#websocket-api | Yes (demo) |
| **Kucoin** | https://docs.kucoin.com/ | https://docs.kucoin.com/#websocket-feed | Yes (sandbox) |
| **Aster** | https://docs.asterdex.com/product/aster-perpetual-pro/api | Same | No |
| **Hyperliquid** | https://hyperliquid.gitbook.io/hyperliquid-docs/ | Same | Yes |

---

## Summary: Next Steps

1. **Pick starting exchange** (recommend Kucoin or Bitget)
2. **Complete WebSocket client** (most critical)
3. **Implement message parsers**
4. **Wire up Python clients**
5. **Test with sandbox**
6. **Move to next exchange**
7. **Once all done: integration tests + documentation**

All foundations are built. Just need to complete the implementation details for each exchange.

Good luck! 🚀
