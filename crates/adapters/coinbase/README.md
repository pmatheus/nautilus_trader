# nautilus-coinbase

Coinbase exchange (spot) integration adapter for Nautilus Trader.

## Overview

This adapter provides integration with Coinbase's spot trading platform (not Coinbase Advanced Trade or Coinbase International Exchange). It includes:

- **HTTP Client**: REST API endpoints for market data, account information, and order management
- **WebSocket Client**: Real-time streaming for market data and account updates
- **Instrument Provider**: Discovery and management of trading pairs
- **Data Client**: Market data streaming (orderbook, trades, ticker)
- **Execution Client**: Order placement, cancellation, and fills

## Features

- Spot trading (BTC-USD, ETH-USD, etc.)
- Market, limit, and stop orders
- Real-time order book (Level 2)
- Trade feed
- Account balance tracking
- Order and fill notifications

## API Documentation

- [Coinbase Exchange REST API](https://docs.cloud.coinbase.com/exchange/docs)
- [Coinbase Exchange WebSocket API](https://docs.cloud.coinbase.com/exchange/docs/websocket-overview)

## Authentication

Coinbase uses API key authentication with the following headers:
- `CB-ACCESS-KEY`: Your API key
- `CB-ACCESS-SIGN`: Message signature using API secret
- `CB-ACCESS-TIMESTAMP`: Request timestamp
- `CB-ACCESS-PASSPHRASE`: Your API passphrase

## License

LGPL-3.0-or-later
