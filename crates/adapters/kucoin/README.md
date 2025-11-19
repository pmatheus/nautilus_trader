# nautilus-kucoin

[NautilusTrader](http://nautilustrader.io) adapter for the [Kucoin](https://www.kucoin.com) cryptocurrency exchange.

The `nautilus-kucoin` crate provides client bindings (HTTP & WebSocket), data
models and helper utilities that wrap the official **Kucoin API**.

The official Kucoin API reference can be found at <https://docs.kucoin.com/>.

## Features

- SPOT trading support
- Token-based WebSocket connection system
- Real-time market data streaming
- Order management and execution
- Account and balance tracking

## WebSocket Token System

Kucoin uses a unique token-based WebSocket connection system:
1. Request a connection token via REST API (public or private)
2. Use the token to establish WebSocket connections
3. Tokens are valid for 24 hours

This is handled automatically by the adapter.

## Documentation

- [Kucoin API Documentation](https://docs.kucoin.com/)
- [Kucoin WebSocket Feed](https://docs.kucoin.com/#websocket-feed)
- [Kucoin SPOT Trading](https://docs.kucoin.com/#spot-trading-market-data)
