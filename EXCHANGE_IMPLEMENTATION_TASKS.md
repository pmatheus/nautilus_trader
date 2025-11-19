# NautilusTrader Exchange Implementation Tasks

This document breaks down the implementation of exchange adapters into discrete tasks suitable for parallel execution by subagents.

## Task Overview

Total Exchanges to Implement/Complete:
- ✅ Binance (already complete)
- ✅ Bybit (already complete)
- ✅ OKX (already complete)
- ⚠️ Kraken (complete implementation)
- ⚠️ Hyperliquid (complete implementation)
- ❌ Coinbase (add spot support)
- ❌ Bitget (full implementation - spot + futures)
- ❌ Kucoin (full implementation - spot + futures)
- ❌ Aster (full implementation - research + implement)

---

## TASK 1: Complete Kraken Adapter (Spot + Futures)

**Priority:** HIGH
**Status:** Partially implemented, needs completion
**Estimated Effort:** 3-5 days
**Dependencies:** None

### Context
- Existing code: `nautilus_trader/adapters/kraken/` and `crates/adapters/kraken/`
- Kraken has both spot and futures markets
- Reference implementations: Binance, Bybit, OKX adapters

### Subtasks
1. **Audit existing Kraken implementation**
   - Review current Rust implementation in `crates/adapters/kraken/`
   - Review current Python implementation in `nautilus_trader/adapters/kraken/`
   - Identify missing components (HTTP methods, WebSocket subscriptions, order types, etc.)

2. **Complete Rust layer**
   - HTTP client: Implement missing REST API endpoints
     - Spot: Order placement, cancellation, account info, trades
     - Futures: Order placement, cancellation, positions, funding
   - WebSocket client: Implement missing subscriptions
     - Order book updates (spot + futures)
     - Trade updates (spot + futures)
     - Order updates (spot + futures)
     - Balance/position updates (futures)
   - Parsers: Complete message parsing for all message types
   - Python bindings: Ensure all Rust functions are exported via PyO3

3. **Complete Python layer**
   - InstrumentProvider: Handle both spot and futures instruments
   - DataClient: Subscribe to market data for spot + futures
   - ExecutionClient: Order management for spot + futures
   - Configuration: Add missing config options

4. **Testing**
   - Unit tests for Rust parsers
   - Integration tests for Python clients
   - Manual testing with Kraken testnet/sandbox

### Success Criteria
- [ ] All Kraken spot order types supported
- [ ] All Kraken futures order types supported
- [ ] Real-time market data streaming works
- [ ] Order placement/cancellation/modification works
- [ ] Position tracking works for futures
- [ ] Balance tracking works for spot
- [ ] Integration tests pass

### Reference Files
- Similar adapter: `nautilus_trader/adapters/binance/` (separate spot/futures)
- Developer guide: `docs/developer_guide/adapters.md`
- Kraken API docs: https://docs.kraken.com/rest/ and https://docs.kraken.com/websockets/

---

## TASK 2: Complete Hyperliquid Adapter

**Priority:** MEDIUM
**Status:** Partially implemented, needs completion
**Estimated Effort:** 2-4 days
**Dependencies:** None

### Context
- Existing code: `nautilus_trader/adapters/hyperliquid/` and `crates/adapters/hyperliquid/`
- Hyperliquid is a perpetuals-focused DEX (no spot)
- Uses WebSocket for orders (POST over WebSocket)
- Reference: Existing Hyperliquid skeleton

### Subtasks
1. **Audit existing implementation**
   - Review current state in `crates/adapters/hyperliquid/`
   - Review Python layer in `nautilus_trader/adapters/hyperliquid/`
   - Identify gaps

2. **Complete Rust layer**
   - HTTP client: Market data, info endpoints
   - WebSocket client: Complete POST over WebSocket implementation
     - Order placement via WebSocket
     - Order cancellation via WebSocket
     - Subscription management
   - Parsers: Message parsing for all types
   - Python bindings: Complete PyO3 exports

3. **Complete Python layer**
   - InstrumentProvider: Perpetual instruments
   - DataClient: Market data subscriptions
   - ExecutionClient: Order management via WebSocket POST
   - Handle wallet management (private key signing)

4. **Testing**
   - Unit tests for message parsing
   - Integration tests
   - Testnet testing

### Success Criteria
- [ ] Perpetual instrument discovery works
- [ ] Market data streaming works
- [ ] Order placement via WebSocket POST works
- [ ] Order cancellation works
- [ ] Position tracking works
- [ ] Integration tests pass

### Reference Files
- Existing code: `crates/adapters/hyperliquid/`
- WebSocket POST pattern: `crates/adapters/hyperliquid/src/websocket/post.rs`
- Hyperliquid API docs: https://hyperliquid.gitbook.io/hyperliquid-docs/

---

## TASK 3: Implement Coinbase Spot Adapter

**Priority:** HIGH
**Status:** Not implemented (Coinbase INTX exists but is derivatives-only)
**Estimated Effort:** 4-6 days
**Dependencies:** None

### Context
- Existing: `nautilus_trader/adapters/coinbase_intx/` (derivatives exchange, different API)
- Need to implement: Regular Coinbase spot exchange adapter
- Coinbase has comprehensive REST + WebSocket APIs
- Major US exchange with good documentation

### Subtasks
1. **Create new adapter structure**
   - Create `crates/adapters/coinbase/` (new directory)
   - Create `nautilus_trader/adapters/coinbase/` (new directory)
   - Copy template structure from `nautilus_trader/adapters/_template/`

2. **Implement Rust layer**
   - HTTP client:
     - Authentication (API key signing)
     - Market data endpoints (ticker, orderbook, trades, candles)
     - Account endpoints (balances, orders, fills)
     - Order endpoints (place, cancel, list)
   - WebSocket client:
     - Public channels (ticker, level2, matches)
     - Private channels (user orders, balances)
     - Subscription management
   - Types: Define Coinbase-specific enums and structs
   - Parsers: Parse all message types to Nautilus models
   - Python bindings: PyO3 exports

3. **Implement Python layer**
   - Configuration: `config.py` with Coinbase-specific settings
   - InstrumentProvider: `providers.py` for spot instruments
   - DataClient: `data.py` for market data subscriptions
   - ExecutionClient: `execution.py` for order management
   - Factories: `factories.py` for instrument creation

4. **Add to build system**
   - Update `Cargo.toml` to include new crate
   - Update Python package `__init__.py` files
   - Add feature flags if needed

5. **Testing**
   - Unit tests for Rust components
   - Integration tests for Python clients
   - Sandbox/testnet testing

### Success Criteria
- [ ] Spot instrument discovery works
- [ ] Real-time market data (orderbook, trades, ticker) works
- [ ] Order placement works (market, limit, stop orders)
- [ ] Order cancellation works
- [ ] Balance tracking works
- [ ] Fill notifications work
- [ ] Integration tests pass

### Reference Files
- Template: `nautilus_trader/adapters/_template/`
- Similar implementation: `nautilus_trader/adapters/binance/spot/`
- Developer guide: `docs/developer_guide/adapters.md`
- Coinbase API docs: https://docs.cloud.coinbase.com/exchange/docs

---

## TASK 4: Implement Bitget Spot Adapter

**Priority:** HIGH
**Status:** Not implemented
**Estimated Effort:** 4-6 days
**Dependencies:** None

### Context
- Bitget is a growing exchange with spot and futures
- Good API documentation
- Will implement spot first, then futures (Task 5)
- Reference: Binance/OKX implementations

### Subtasks
1. **Research Bitget API**
   - Study REST API documentation
   - Study WebSocket API documentation
   - Understand authentication mechanism
   - Document supported order types and features

2. **Create adapter structure**
   - Create `crates/adapters/bitget/`
   - Create `nautilus_trader/adapters/bitget/`
   - Set up common module for shared code

3. **Implement Rust layer - Spot**
   - HTTP client:
     - Authentication (signature generation)
     - Spot market data endpoints
     - Spot account endpoints
     - Spot order endpoints
   - WebSocket client:
     - Public spot channels
     - Private spot channels
     - Heartbeat/ping handling
   - Types: Bitget-specific enums and structs
   - Parsers: Message parsing
   - Python bindings: PyO3 exports

4. **Implement Python layer - Spot**
   - Configuration: `config.py`
   - InstrumentProvider: Spot instruments
   - DataClient: Market data
   - ExecutionClient: Order management
   - Factories: Instrument factories

5. **Build system integration**
   - Update Cargo workspace
   - Update Python packaging

6. **Testing**
   - Unit tests
   - Integration tests
   - Live testnet testing

### Success Criteria
- [ ] Bitget spot instruments discoverable
- [ ] Market data streaming works
- [ ] Order management works (place, cancel, modify)
- [ ] Balance updates work
- [ ] Integration tests pass

### Reference Files
- Template: `nautilus_trader/adapters/_template/`
- Reference: `nautilus_trader/adapters/okx/` (unified approach)
- Bitget API docs: https://bitgetlimited.github.io/apidoc/en/spot/

---

## TASK 5: Implement Bitget Futures Adapter

**Priority:** HIGH
**Status:** Not implemented
**Estimated Effort:** 4-6 days
**Dependencies:** Task 4 (Bitget spot) should be done first

### Context
- Builds on Task 4 (Bitget spot)
- Bitget futures include USDT-margined and Coin-margined perpetuals
- Can reuse common code from spot implementation

### Subtasks
1. **Research Bitget Futures API**
   - Study futures REST API
   - Study futures WebSocket API
   - Understand position management
   - Document funding rate handling

2. **Extend Rust layer for Futures**
   - HTTP client:
     - Futures market data endpoints
     - Futures account/position endpoints
     - Futures order endpoints
     - Leverage and margin management
   - WebSocket client:
     - Public futures channels
     - Private futures channels (positions, orders)
   - Types: Futures-specific enums (contract type, position side)
   - Parsers: Futures message parsing

3. **Extend Python layer for Futures**
   - Update configuration for futures
   - InstrumentProvider: Handle futures instruments
   - DataClient: Futures market data
   - ExecutionClient: Futures order management, position tracking
   - Handle funding rates

4. **Testing**
   - Unit tests
   - Integration tests
   - Testnet testing

### Success Criteria
- [ ] Bitget futures instruments discoverable (USDT, Coin-margined)
- [ ] Futures market data streaming works
- [ ] Futures order management works
- [ ] Position tracking works
- [ ] Funding rate updates work
- [ ] Integration tests pass

### Reference Files
- Bitget spot code from Task 4
- Reference: `nautilus_trader/adapters/binance/futures/`
- Reference: `nautilus_trader/adapters/bybit/` (multi-product)
- Bitget Futures API docs: https://bitgetlimited.github.io/apidoc/en/mix/

---

## TASK 6: Implement Kucoin Spot Adapter

**Priority:** MEDIUM
**Status:** Not implemented
**Estimated Effort:** 4-6 days
**Dependencies:** None

### Context
- Kucoin is popular in Asia with good liquidity
- Has both spot and futures
- Unique WebSocket connection approach (connect token system)
- Will implement spot first, then futures (Task 7)

### Subtasks
1. **Research Kucoin API**
   - Study REST API documentation
   - Study WebSocket API (understand token-based connection)
   - Understand authentication
   - Document order types and features

2. **Create adapter structure**
   - Create `crates/adapters/kucoin/`
   - Create `nautilus_trader/adapters/kucoin/`
   - Set up common module

3. **Implement Rust layer - Spot**
   - HTTP client:
     - Authentication
     - WebSocket token acquisition endpoint
     - Spot market data endpoints
     - Spot account endpoints
     - Spot order endpoints
   - WebSocket client:
     - Token-based connection establishment
     - Public spot channels
     - Private spot channels
     - Ping/pong handling
   - Types: Kucoin-specific types
   - Parsers: Message parsing
   - Python bindings

4. **Implement Python layer - Spot**
   - Configuration
   - InstrumentProvider
   - DataClient
   - ExecutionClient
   - Factories

5. **Build system integration**
   - Update Cargo workspace
   - Update Python packaging

6. **Testing**
   - Unit tests
   - Integration tests
   - Sandbox testing

### Success Criteria
- [ ] WebSocket token acquisition works
- [ ] Spot instruments discoverable
- [ ] Market data streaming works
- [ ] Order management works
- [ ] Balance tracking works
- [ ] Integration tests pass

### Reference Files
- Template: `nautilus_trader/adapters/_template/`
- Reference: `nautilus_trader/adapters/okx/`
- Kucoin API docs: https://docs.kucoin.com/

---

## TASK 7: Implement Kucoin Futures Adapter

**Priority:** MEDIUM
**Status:** Not implemented
**Estimated Effort:** 4-6 days
**Dependencies:** Task 6 (Kucoin spot) should be done first

### Context
- Builds on Task 6 (Kucoin spot)
- Kucoin Futures has separate API base URLs
- Supports USDT-margined perpetuals and futures

### Subtasks
1. **Research Kucoin Futures API**
   - Study futures REST API
   - Study futures WebSocket API
   - Understand position management
   - Document funding rates

2. **Extend Rust layer for Futures**
   - HTTP client:
     - Futures-specific endpoints (different base URL)
     - Futures market data
     - Futures account/position endpoints
     - Futures order endpoints
   - WebSocket client:
     - Futures channels
     - Position updates
   - Types: Futures-specific types
   - Parsers: Futures message parsing

3. **Extend Python layer for Futures**
   - Update configuration
   - InstrumentProvider: Futures instruments
   - DataClient: Futures market data
   - ExecutionClient: Futures trading, position management

4. **Testing**
   - Unit tests
   - Integration tests
   - Sandbox testing

### Success Criteria
- [ ] Futures instruments discoverable
- [ ] Futures market data streaming works
- [ ] Futures order management works
- [ ] Position tracking works
- [ ] Funding rate handling works
- [ ] Integration tests pass

### Reference Files
- Kucoin spot code from Task 6
- Reference: `nautilus_trader/adapters/bybit/`
- Kucoin Futures API docs: https://docs.kucoin.com/futures/

---

## TASK 8: Implement Aster Exchange Adapter

**Priority:** MEDIUM
**Status:** Not implemented (new exchange)
**Estimated Effort:** 5-8 days (includes research)
**Dependencies:** None

### Context
- Aster is a new exchange to add
- Need to research what markets they support (spot/futures/both)
- Implementation scope depends on exchange features

### Subtasks
1. **Research Aster Exchange**
   - Identify official website and API documentation
   - Determine supported markets (spot, futures, perpetuals, options?)
   - Study authentication mechanism
   - Document API rate limits
   - Understand WebSocket architecture
   - Document supported order types
   - Check for testnet/sandbox availability

2. **Create implementation plan**
   - Based on research, determine implementation approach
   - Identify which existing adapter is best reference (Binance/OKX/Bybit style)
   - Plan Rust crate structure
   - Plan Python package structure

3. **Create adapter structure**
   - Create `crates/adapters/aster/`
   - Create `nautilus_trader/adapters/aster/`
   - Set up common module if needed

4. **Implement Rust layer**
   - HTTP client:
     - Authentication
     - Market data endpoints
     - Account endpoints
     - Order endpoints
     - (Scope depends on supported products)
   - WebSocket client:
     - Public channels
     - Private channels
     - Connection management
   - Types: Aster-specific types
   - Parsers: Message parsing for all types
   - Python bindings: PyO3 exports

5. **Implement Python layer**
   - Configuration: `config.py`
   - InstrumentProvider: Handle all instrument types
   - DataClient: Market data subscriptions
   - ExecutionClient: Order management, position tracking
   - Factories: Instrument factories

6. **Build system integration**
   - Update Cargo.toml
   - Update Python packaging
   - Add feature flags if needed

7. **Testing**
   - Unit tests for Rust
   - Integration tests for Python
   - Live testing (testnet or mainnet with small amounts)

### Success Criteria
- [ ] API research documented
- [ ] All supported instrument types discoverable
- [ ] Market data streaming works for all products
- [ ] Order management works for all products
- [ ] Position tracking works (if applicable)
- [ ] Balance tracking works
- [ ] Integration tests pass

### Reference Files
- Template: `nautilus_trader/adapters/_template/`
- Reference: Choose based on Aster's architecture
- Developer guide: `docs/developer_guide/adapters.md`
- Aster API docs: (To be determined during research)

### Notes
- If Aster API documentation is unavailable or poor, consider deprioritizing
- If Aster requires special features (DEX, smart contracts, etc.), may need custom approach
- Confirm Aster is a legitimate, active exchange before investing significant effort

---

## TASK 9: Create Integration Tests for All New/Updated Adapters

**Priority:** HIGH
**Status:** Not started
**Estimated Effort:** 3-5 days
**Dependencies:** Tasks 1-8 (all adapter implementations)

### Context
- All new/updated adapters need comprehensive integration tests
- Tests should cover both unit level (Rust) and integration level (Python)
- Follow existing test patterns in `tests/integration_tests/adapters/`

### Subtasks
1. **Create test structure for each adapter**
   - Kraken: Update existing tests
   - Hyperliquid: Update existing tests
   - Coinbase: Create new test directory
   - Bitget: Create new test directory
   - Kucoin: Create new test directory
   - Aster: Create new test directory

2. **Implement unit tests (Rust)**
   - HTTP client request building
   - Message parsing
   - Signature generation
   - Error handling

3. **Implement integration tests (Python)**
   - InstrumentProvider tests
   - DataClient tests (with mocked responses)
   - ExecutionClient tests (with mocked responses)
   - Configuration tests

4. **Create test fixtures**
   - Mock HTTP responses
   - Mock WebSocket messages
   - Test data for various scenarios

5. **Document test setup**
   - Add instructions for running tests
   - Document any required credentials/setup
   - Add CI/CD configuration if needed

### Success Criteria
- [ ] All adapters have unit tests (Rust)
- [ ] All adapters have integration tests (Python)
- [ ] Test coverage >80% for new code
- [ ] All tests pass in CI/CD
- [ ] Test documentation complete

### Reference Files
- Existing tests: `tests/integration_tests/adapters/binance/`
- Existing tests: `tests/integration_tests/adapters/bybit/`
- Testing guide: `docs/developer_guide/testing.md`

---

## TASK 10: Create Documentation for All New/Updated Exchanges

**Priority:** MEDIUM
**Status:** Not started
**Estimated Effort:** 2-3 days
**Dependencies:** Tasks 1-8 (all adapter implementations)

### Context
- All new exchanges need integration documentation
- Documentation should follow existing patterns in `docs/integrations/`
- Should include setup, configuration, examples, and known limitations

### Subtasks
1. **Create/update documentation files**
   - Kraken: Update `docs/integrations/kraken.md`
   - Hyperliquid: Update `docs/integrations/hyperliquid.md`
   - Coinbase: Create `docs/integrations/coinbase.md`
   - Bitget: Create `docs/integrations/bitget.md`
   - Kucoin: Create `docs/integrations/kucoin.md`
   - Aster: Create `docs/integrations/aster.md`

2. **For each exchange, document:**
   - Overview and supported products (spot/futures/perpetuals)
   - Account setup and API key generation
   - Configuration examples
   - Code examples (instrument provider, data client, execution client)
   - Supported order types
   - Rate limits and restrictions
   - Known issues and limitations
   - Testnet/sandbox information

3. **Update main documentation**
   - Update `docs/integrations/index.md` to list new exchanges
   - Update `README.md` if needed
   - Update any feature comparison tables

4. **Create example scripts**
   - Add example scripts to `examples/live/<exchange>/`
   - Include both data streaming and execution examples

### Success Criteria
- [ ] All exchanges have complete documentation
- [ ] Documentation follows consistent format
- [ ] Code examples are tested and working
- [ ] Main index pages updated
- [ ] Example scripts provided

### Reference Files
- Existing docs: `docs/integrations/binance.md`
- Existing docs: `docs/integrations/bybit.md`
- Examples: `examples/live/binance/`

---

## Parallel Execution Strategy

### Phase 1 (Can run in parallel):
- Task 1: Complete Kraken
- Task 3: Implement Coinbase Spot
- Task 4: Implement Bitget Spot
- Task 6: Implement Kucoin Spot
- Task 8: Implement Aster (start with research)

### Phase 2 (After Phase 1):
- Task 2: Complete Hyperliquid (low dependency)
- Task 5: Implement Bitget Futures (needs Task 4)
- Task 7: Implement Kucoin Futures (needs Task 6)
- Task 8: Complete Aster implementation (after research)

### Phase 3 (After implementations):
- Task 9: Integration tests
- Task 10: Documentation

---

## Total Effort Estimate

- **Phase 1:** ~20-30 days of parallel work (5 tasks × 4-6 days each)
- **Phase 2:** ~15-22 days of parallel work (4 tasks × 4-6 days each)
- **Phase 3:** ~5-8 days of parallel work (2 tasks)

**Total calendar time with 5-6 parallel subagents:** ~6-8 weeks
**Total serial time:** ~40-60 weeks

---

## Important Notes for Subagents

### Code Quality Standards
- Follow existing code style (rustfmt for Rust, black for Python)
- Add comprehensive error handling
- Include logging for debugging
- Document complex logic with comments
- Use type hints in Python

### Security Considerations
- Never log API keys or secrets
- Use secure credential storage
- Validate all API responses
- Handle rate limits gracefully
- Implement proper timeout handling

### Testing Requirements
- Write tests alongside implementation
- Test error cases, not just happy paths
- Use mocked responses for integration tests
- Document manual testing procedures

### Documentation Requirements
- Document public APIs with docstrings
- Include examples in documentation
- Document known limitations
- Keep README files updated
