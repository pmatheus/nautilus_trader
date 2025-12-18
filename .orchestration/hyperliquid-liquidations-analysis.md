# Hyperliquid Liquidation Stream Analysis

**Task ID:** ADAPT-HL-001
**Date:** 2025-12-18
**Status:** Complete

## Executive Summary

Hyperliquid **does not provide a public liquidation stream** via their standard WebSocket API. Liquidation events are only available through **user-specific subscription channels** (`userEvents` and `userNonFundingLedgerUpdates`), which require knowing the user address beforehand.

However, **third-party infrastructure providers** (like Dwellir) offer gRPC-based solutions that stream all block fills, allowing detection of liquidations across the entire platform by filtering fill data.

---

## Current NautilusTrader Implementation

### Existing Support

The Hyperliquid adapter already has **complete support** for receiving liquidation events via user-specific channels:

#### Data Structures (Rust)

**Location:** `crates/adapters/hyperliquid/src/websocket/messages.rs`

1. **`WsLiquidationData`** (lines 593-601):
```rust
pub struct WsLiquidationData {
    pub lid: u64,                        // Liquidation ID
    pub liquidator: String,               // Address that liquidated
    pub liquidated_user: String,          // Address that got liquidated
    pub liquidated_ntl_pos: String,       // Notional position size
    pub liquidated_account_value: String, // Account value at liquidation
}
```

2. **`FillLiquidationData`** (lines 572-580):
```rust
pub struct FillLiquidationData {
    pub liquidated_user: Option<String>,
    pub mark_px: f64,                     // Mark price at liquidation
    pub method: HyperliquidLiquidationMethod, // "market" or "backstop"
}
```

3. **`HyperliquidLiquidationMethod`** enum:
```rust
pub enum HyperliquidLiquidationMethod {
    Market,    // Liquidation via market orders
    Backstop,  // Liquidation via backstop mechanism
}
```

#### Subscription Support

**Location:** `crates/adapters/hyperliquid/src/websocket/client.rs`

- **`subscribe_user_events(user: &str)`** (line 337-349): Subscribes to fills, funding, and **liquidations** for a specific user address
- **`subscribe_all_user_channels(user: &str)`** (line 352-357): Convenience method for all user data

#### Event Handling

**Location:** `crates/adapters/hyperliquid/src/websocket/messages.rs`

The `WsUserEventData` enum (lines 518-543) handles liquidations:
```rust
pub enum WsUserEventData {
    Fills { fills: Vec<WsFillData> },
    Funding { funding: WsUserFundingData },
    Liquidation { liquidation: WsLiquidationData }, // ← Liquidation event
    NonUserCancel { ... },
    TriggerActivated { ... },
}
```

Individual fills also contain optional liquidation metadata via `WsFillData::liquidation: Option<FillLiquidationData>` (line 565).

---

## Hyperliquid API Documentation

### Official WebSocket Channels

Based on official Hyperliquid documentation:

#### User-Specific Liquidation Channels

1. **`userEvents`** subscription:
   - **Format:** `{"method": "subscribe", "subscription": {"type": "userEvents", "user": "<address>"}}`
   - **Data:** Includes `{"liquidation": WsLiquidation}` events
   - **Requires:** User wallet address
   - **Supported:** ✅ Already implemented in NautilusTrader

2. **`userNonFundingLedgerUpdates`** subscription:
   - **Format:** `{"method": "subscribe", "subscription": {"type": "userNonFundingLedgerUpdates", "user": "<address>"}}`
   - **Data:** Ledger updates including withdrawals, deposits, transfers, and liquidations
   - **Requires:** User wallet address
   - **Supported:** ✅ Channel exists in enums but not exposed as dedicated method

#### Public Channels (No Liquidations)

The following public channels do **NOT** include liquidation data:
- `trades` - Trade ticks for a coin
- `l2Book` - Order book updates
- `bbo` - Best bid/offer quotes
- `candle` - OHLCV candles
- `allMids` - All mid prices

### Info Endpoint (REST API)

**Location:** `https://api.hyperliquid.xyz/info`

The Info endpoint does **NOT** provide:
- Historical liquidation data
- Liquidation query endpoints
- Public liquidation feeds

Available endpoints focus on orders, fills, account data, and vault information.

---

## Alternative Approaches for Public Liquidation Data

### 1. Third-Party gRPC Infrastructure (Dwellir)

**Source:** https://www.dwellir.com/blog/building-real-time-hyperliquid-liquidation-tracker

#### How It Works

- **Stream:** `StreamBlockFills` gRPC method
- **Data:** Continuous stream of all fills (trades) across Hyperliquid
- **Performance:** Processes 70-80 blocks per second with sub-second latency
- **Detection:** Filter fills containing:
  - `liquidation` field in fill data
  - Direction matches "Close Long" or "Close Short"
  - User address equals liquidated user (not liquidator)

#### Advantages

- ✅ Captures **all** liquidations platform-wide
- ✅ No need to know user addresses beforehand
- ✅ Sub-second latency
- ✅ Comprehensive market view

#### Disadvantages

- ❌ Requires third-party infrastructure provider
- ❌ Additional service dependency
- ❌ Potential cost for API access
- ❌ Not part of official Hyperliquid API

### 2. Monitor Public Trade Stream + Heuristics

**Approach:** Subscribe to `trades` channel for all coins and detect liquidations via heuristics

#### Detection Signals

- Large position closes at mark price
- Rapid sequence of fills in same direction
- Fill prices near known liquidation levels
- Correlation with mark price movements

#### Advantages

- ✅ Uses official public API
- ✅ No third-party dependencies

#### Disadvantages

- ❌ Unreliable - cannot definitively identify liquidations
- ❌ High false positive/negative rate
- ❌ Requires complex heuristic logic
- ❌ No access to liquidation metadata (method, account value)

### 3. Multi-User Subscription Strategy

**Approach:** Subscribe to `userEvents` for multiple known user addresses

#### Use Cases

- Monitor specific traders or accounts
- Track vault liquidations
- Follow large position holders

#### Advantages

- ✅ Uses official API
- ✅ Accurate liquidation data
- ✅ Already supported in NautilusTrader

#### Disadvantages

- ❌ Requires knowing user addresses
- ❌ Limited to specific users
- ❌ Does not provide market-wide view

---

## Recommendations

### Immediate Action (Already Supported)

For users wanting to monitor **their own liquidations** or **specific user liquidations**:

**Status:** ✅ **Fully Implemented**

```rust
// User can already do this:
client.subscribe_user_events(user_address).await?;
```

This will emit `WsLiquidationData` events whenever the specified user is liquidated.

### Enhancement Option 1: Expose UserNonFundingLedgerUpdates

**Priority:** Low
**Effort:** Minimal
**Value:** Provides alternative liquidation data source

Add convenience method to `HyperliquidWebSocketClient`:

```rust
pub async fn subscribe_user_ledger(&self, user: &str) -> anyhow::Result<()> {
    let subscription = SubscriptionRequest::UserNonFundingLedgerUpdates {
        user: user.to_string(),
    };
    self.cmd_tx.read().await
        .send(HandlerCommand::Subscribe { subscriptions: vec![subscription] })
        .map_err(|e| anyhow::anyhow!("Failed to send subscribe command: {e}"))?;
    Ok(())
}
```

### Enhancement Option 2: Document Liquidation Data Access

**Priority:** Medium
**Effort:** Low
**Value:** Improves user experience

Document in adapter README:
- How to subscribe to liquidations via `subscribe_user_events()`
- Liquidation data structure and fields
- Example usage patterns
- Clarify that public liquidation stream is not available

### Enhancement Option 3: Third-Party Integration (Future)

**Priority:** Low (on-demand)
**Effort:** High
**Value:** Enables market-wide liquidation tracking

If users require platform-wide liquidation monitoring:

1. **Evaluate gRPC providers** (Dwellir, others)
2. **Design integration layer** for block fill streaming
3. **Implement liquidation filtering** logic
4. **Expose as separate data stream** (not part of standard adapter)

**Recommendation:** Only implement if there's clear user demand, as this requires:
- Third-party service dependencies
- Additional infrastructure complexity
- Potential cost implications

---

## Files Reviewed

### Rust Implementation

1. `crates/adapters/hyperliquid/src/websocket/enums.rs` - Channel definitions
2. `crates/adapters/hyperliquid/src/websocket/messages.rs` - Liquidation data structures
3. `crates/adapters/hyperliquid/src/websocket/client.rs` - Subscription methods
4. `crates/adapters/hyperliquid/src/http/models.rs` - Position liquidation price
5. `crates/adapters/hyperliquid/src/common/enums.rs` - Liquidation method enum

### Python Implementation

1. `nautilus_trader/adapters/hyperliquid/data.py` - Data client (liquidation events handled via Rust)

---

## References

### Official Documentation

- [Hyperliquid WebSocket Subscriptions](https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/api/websocket/subscriptions)
- [Hyperliquid Info Endpoint](https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/api/info-endpoint)
- [Hyperliquid WebSocket API](https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/api/websocket)

### Third-Party Resources

- [Dwellir: Building Real-Time Hyperliquid Liquidation Tracker](https://www.dwellir.com/blog/building-real-time-hyperliquid-liquidation-tracker)
- [Hyperliquid Data API Explained - HypeRPC](https://hyperpc.app/blog/hyperliquid-data-api-explained)

---

## Conclusion

**Key Findings:**

1. ✅ **User-specific liquidations are fully supported** in NautilusTrader via `subscribe_user_events()`
2. ❌ **Public liquidation stream does not exist** in official Hyperliquid API
3. ⚠️ **Market-wide liquidations require third-party infrastructure** (gRPC block streams)

**No immediate implementation work required** - the adapter already handles liquidations correctly for user-specific monitoring. Enhancement opportunities exist for documentation and optional third-party integration if demand arises.
