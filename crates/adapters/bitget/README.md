# Bitget Exchange Adapter

This adapter provides integration with the Bitget cryptocurrency exchange for NautilusTrader.

## Status

**SPOT TRADING ONLY** - This is an initial implementation focusing on spot trading. Futures support will be added in a separate task (Task 5).

## Implementation Progress

### Rust Layer (Complete - Core Structure)
- ✅ Common module (enums, types, authentication)
- ✅ HTTP client with authentication
- ✅ WebSocket client stub
- ✅ Configuration structures
- ✅ Basic error handling
- ⚠️ Parser implementations (needs expansion)
- ⚠️ Data and execution modules (stubs only)

### Python Layer (Complete - Core Structure)
- ✅ Configuration classes
- ✅ Constants and types
- ✅ InstrumentProvider stub
- ✅ DataClient stub
- ✅ ExecutionClient stub
- ✅ Factory classes stub

### Build System
- ✅ Added to workspace Cargo.toml
- ✅ Cargo.toml for bitget crate created

## Architecture

The adapter follows the unified approach similar to OKX:

```
crates/adapters/bitget/
├── src/
│   ├── common/          # Shared types, enums, authentication
│   ├── http/            # HTTP REST API client
│   ├── websocket/       # WebSocket client
│   ├── data/            # Data client logic
│   ├── execution/       # Execution client logic
│   └── python/          # PyO3 bindings
└── Cargo.toml

nautilus_trader/adapters/bitget/
├── __init__.py          # Package exports
├── config.py            # Configuration classes
├── constants.py         # Venue constants
├── types.py             # Python enums
├── providers.py         # InstrumentProvider
├── data.py              # LiveMarketDataClient
├── execution.py         # LiveExecutionClient
└── factories.py         # Client factories
```

## Common Module

The common module contains code that will be reused when implementing futures support:

- **Authentication**: HMAC SHA256 signature generation
- **Enums**: Order types, sides, statuses, instrument types
- **URLs**: Base URLs for production and demo environments
- **Types**: Common data structures

## Bitget API Information

### Base URLs
- **Production HTTP**: `https://api.bitget.com`
- **Production WebSocket Public**: `wss://ws.bitget.com/v2/ws/public`
- **Production WebSocket Private**: `wss://ws.bitget.com/v2/ws/private`
- **Demo HTTP**: `https://api.bitgetapi.com`
- **Demo WebSocket Public**: `wss://wspap.bitget.com/v2/ws/public`
- **Demo WebSocket Private**: `wss://wspap.bitget.com/v2/ws/private`

### Authentication
Bitget uses HMAC SHA256 authentication:
1. Concatenate: `timestamp + method + requestPath + body`
2. Sign with secret key using HMAC SHA256
3. Base64 encode the signature
4. Include in headers:
   - `access-key`: API key
   - `access-sign`: Signature
   - `access-timestamp`: Timestamp (milliseconds)
   - `access-passphrase`: API passphrase

### Rate Limits
- Subscription rate: 240 requests per hour per connection
- Max subscriptions: 1000 channels per connection
- WebSocket heartbeat: 30 seconds

### Supported Order Types (Spot)
- Market orders
- Limit orders
- Time in force: GTC, IOC, FOK, Post-only

## What Needs to Be Done

To complete the spot trading implementation:

### High Priority
1. **HTTP Client Expansion**
   - Add all spot market data endpoints
   - Add spot account/balance endpoints
   - Add spot order management endpoints
   - Implement proper rate limiting

2. **WebSocket Client**
   - Complete connection logic
   - Implement subscription management
   - Add heartbeat/ping handling
   - Parse market data messages
   - Parse order updates

3. **Parsers**
   - Parse instrument data to Nautilus instruments
   - Parse ticker data
   - Parse order book data
   - Parse trade data
   - Parse order updates
   - Parse balance updates

4. **Data Client**
   - Implement market data subscriptions
   - Handle WebSocket reconnection
   - Manage subscription state

5. **Execution Client**
   - Implement order placement
   - Implement order cancellation
   - Implement order modification
   - Handle order status updates
   - Handle balance updates

6. **PyO3 Bindings**
   - Export all necessary types
   - Ensure proper async handling

### Medium Priority
7. **Testing**
   - Unit tests for parsers
   - Unit tests for authentication
   - Integration tests with mock responses
   - Manual testing with testnet

8. **Documentation**
   - Complete API endpoint documentation
   - Add usage examples
   - Document configuration options

### Lower Priority
9. **Optimization**
   - Implement connection pooling
   - Optimize WebSocket message handling
   - Add caching where appropriate

10. **Error Handling**
    - Comprehensive error mapping
    - Retry logic for transient errors
    - Proper error propagation

## API Endpoints Reference

### Spot Market Data
- `GET /api/v2/spot/public/symbols` - Get instruments
- `GET /api/v2/spot/market/tickers` - Get ticker data
- `GET /api/v2/spot/market/orderbook` - Get order book
- `GET /api/v2/spot/market/fills` - Get recent trades
- `GET /api/v2/spot/market/candles` - Get candlestick data

### Spot Trading
- `POST /api/v2/spot/trade/place-order` - Place order
- `POST /api/v2/spot/trade/cancel-order` - Cancel order
- `POST /api/v2/spot/trade/batch-orders` - Batch place orders
- `POST /api/v2/spot/trade/batch-cancel-order` - Batch cancel
- `GET /api/v2/spot/trade/unfilled-orders` - Get open orders
- `GET /api/v2/spot/trade/history-orders` - Get order history
- `GET /api/v2/spot/trade/fills` - Get fills

### Spot Account
- `GET /api/v2/spot/account/info` - Get account info
- `GET /api/v2/spot/account/assets` - Get balances

### WebSocket Channels (Spot)
- `spot/ticker` - Ticker updates
- `spot/depth` - Order book updates
- `spot/trade` - Trade updates
- `spot/candle` - Candlestick updates
- `spot/orders` - Order updates (private)
- `spot/account` - Account updates (private)

## Testing

The adapter can be tested against Bitget's demo/testnet environment by setting `is_demo=True` in the configuration.

## Future Work (Task 5)

After spot implementation is complete and tested, futures support will be added:
- USDT-margined perpetuals
- Coin-margined perpetuals
- USDC perpetuals
- Position management
- Funding rates
- Leverage management

## References

- [Bitget API Documentation](https://www.bitget.com/api-doc/)
- [Bitget Spot API](https://bitgetlimited.github.io/apidoc/en/spot/)
- [Bitget WebSocket API](https://www.bitget.com/api-doc/common/websocket-intro)
