# L3 Orderbook Harvesting Architecture

## Overview

This fork extends NautilusTrader to build a **multi-exchange L3 orderbook reconstruction and harvesting system** that streams market data to Supabase Iceberg tables for historical analysis.

## Data Sources by Exchange

| Exchange | L2 Orderbook | Trades | Liquidations | Status |
|----------|-------------|--------|--------------|--------|
| Binance | ✓ | ✓ | ✓ | Stable |
| Bybit | ✓ | ✓ | ✓ | Stable |
| Bitget | ✓ | ✓ | ✓ | **NEW** |
| Coinbase | ✓ | ✓ | - | Stable |
| Kraken | ✓ | ✓ | - | Building |
| Hyperliquid | ✓ (L2/L4) | ✓ | ✓ | Building |
| OKX | ✓ | ✓ | ✓ | Stable |

## L3 Reconstruction Strategy

### Input Streams
1. **L2 Orderbook Snapshots** - Initial state
2. **L2 Orderbook Deltas** - Price level changes
3. **Trade Events** - Executed orders with aggressor side

### Reconstruction Logic
```
L2 Delta (price, qty_new, qty_old) + Trade Event (price, qty, side, time)
    ↓
Infer Order Actions:
    - ADD: qty_new > qty_old (new liquidity)
    - CANCEL: qty_new < qty_old AND no trade at price
    - FILL: qty_new < qty_old AND trade at price matches delta
    - MODIFY: Complex inference from sequential deltas
    ↓
Local L3 MBO Representation (synthetic order_id, price, qty, side, action, ts)
```

### Key Components

1. **L3ReconstructionEngine** (new Rust crate)
   - Maintains local orderbook state per instrument
   - Correlates L2 deltas with trade events
   - Generates synthetic order IDs for tracking
   - Emits OrderBookDelta events with L3_MBO book_type

2. **IcebergSink** (new persistence module)
   - Batches OrderBookDelta/OrderBookDepth events
   - Converts to Arrow RecordBatch
   - Writes Parquet to Supabase S3
   - Updates Iceberg catalog metadata

## Data Schema (Iceberg Tables)

### `orderbook_l3_events`
| Column | Type | Description |
|--------|------|-------------|
| ts_event | TIMESTAMP(ns) | Exchange event time |
| ts_init | TIMESTAMP(ns) | Local processing time |
| instrument_id | STRING | e.g., "BTCUSDT.BINANCE" |
| venue | STRING | Exchange name |
| action | INT8 | ADD=1, UPDATE=2, DELETE=3, CLEAR=4 |
| side | INT8 | BID=1, ASK=2 |
| price | DECIMAL(18,8) | Price level |
| size | DECIMAL(18,8) | Order size |
| order_id | UINT64 | Synthetic order ID |
| flags | UINT8 | F_LAST, F_SNAPSHOT, etc. |
| sequence | UINT64 | Exchange sequence number |

**Partitioning**: `venue`, `instrument_id`, `date(ts_event)`

### `trades`
| Column | Type | Description |
|--------|------|-------------|
| ts_event | TIMESTAMP(ns) | Trade time |
| instrument_id | STRING | Instrument identifier |
| venue | STRING | Exchange name |
| trade_id | STRING | Exchange trade ID |
| price | DECIMAL(18,8) | Trade price |
| size | DECIMAL(18,8) | Trade size |
| aggressor_side | INT8 | BUY=1, SELL=2 |

### `liquidations`
| Column | Type | Description |
|--------|------|-------------|
| ts_event | TIMESTAMP(ns) | Liquidation time |
| instrument_id | STRING | Instrument identifier |
| venue | STRING | Exchange name |
| side | INT8 | LONG=1, SHORT=2 |
| price | DECIMAL(18,8) | Liquidation price |
| size | DECIMAL(18,8) | Liquidated quantity |

## Exchange Adapter Status

### Existing (Stable)
- **Binance**: Full L2/trades/liquidations
- **Bybit**: Full L2/trades/liquidations
- **Coinbase International**: L2/trades
- **OKX**: Full L2/trades/liquidations

### Needs Completion
- **Kraken**: Data client exists, needs WebSocket depth/trades
- **Hyperliquid**: Data client exists, uses L2/L4 (not L3), needs liquidations

### New Implementation Required
- **Bitget**: Full adapter from template (REST + WebSocket)

## Supabase Iceberg Integration

### Configuration
```python
SUPABASE_PROJECT_REF = "your-project-ref"
ICEBERG_WAREHOUSE = "orderbook-harvest"
S3_ENDPOINT = f"https://{SUPABASE_PROJECT_REF}.supabase.co/storage/v1/s3"
CATALOG_URI = f"https://{SUPABASE_PROJECT_REF}.supabase.co/storage/v1/iceberg"
```

### Write Pattern
1. Buffer events in memory (configurable batch size/time)
2. Convert to Arrow RecordBatch
3. Write Parquet file to S3
4. Commit to Iceberg catalog (atomic)

## Implementation Phases

### Phase 1: Infrastructure
- [ ] Create `nautilus_iceberg` crate for Supabase integration
- [ ] Implement IcebergWriter with batching
- [ ] Add PyIceberg/Arrow serialization

### Phase 2: L3 Reconstruction
- [ ] Create `nautilus_l3_engine` crate
- [ ] Implement L2→L3 inference algorithm
- [ ] Add synthetic order ID generation
- [ ] Handle exchange-specific quirks

### Phase 3: Exchange Adapters
- [ ] Complete Kraken WebSocket data streams
- [ ] Complete Hyperliquid L2/trades/liquidations
- [ ] Implement Bitget adapter from scratch

### Phase 4: Harvester Service
- [ ] Create harvester configuration system
- [ ] Multi-exchange concurrent streaming
- [ ] Graceful reconnection handling
- [ ] Monitoring and metrics
