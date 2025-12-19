# ADAPT-HL-001 Analysis Report: Hyperliquid Liquidation Stream Options

**Date:** 2025-12-18
**Task ID:** ADAPT-HL-001
**Status:** SUCCESS

## Summary

Analysis complete. Hyperliquid does NOT provide a public liquidation stream through its official WebSocket API. However, user-specific liquidation events are fully supported and already implemented in NautilusTrader.

## Findings

### Public Liquidation Stream Available?
**Answer: NO**

- No dedicated public liquidation channel exists
- No public WebSocket feed for market-wide liquidations
- No REST API endpoints for liquidation queries

### User-Specific Liquidation Data Available?
**Answer: YES - Fully Implemented**

- Available via `userEvents` subscription channel
- Available via `userNonFundingLedgerUpdates` subscription channel
- Already integrated in NautilusTrader adapter

## WebSocket Subscription Types Identified

### Primary Channel: userEvents
**Location:** `crates/adapters/hyperliquid/src/websocket/messages.rs` (lines 93-94, 518-545)

- **Subscription Format:** `{"type": "userEvents", "user": "<address>"}`
- **Data Type:** `WsUserEventData::Liquidation`
- **Implementation:** `subscribe_user_events()` method in WebSocketClient
- **Status:** ✅ Ready to use

### Secondary Channel: userNonFundingLedgerUpdates
**Location:** `crates/adapters/hyperliquid/src/websocket/client.rs`

- **Subscription Format:** `{"type": "userNonFundingLedgerUpdates", "user": "<address>"}`
- **Data Includes:** Liquidations, deposits, transfers, withdrawals
- **Implementation:** Channel exists but not exposed as dedicated method
- **Status:** ⚠️ Available but not optimized

## Liquidation Data Structures

### WsLiquidationData
```
- lid: u64 (liquidation ID)
- liquidator: String (address of liquidator)
- liquidated_user: String (address liquidated)
- liquidated_ntl_pos: String (notional position size)
- liquidated_account_value: String (account value at time)
```

### Liquidation Methods Supported
- Market: Liquidation via market orders
- Backstop: Liquidation via backstop mechanism

## Implementation Recommendation

### Immediate (No New Work Required)
Users can subscribe to liquidations using existing API:
```rust
client.subscribe_user_events(user_address).await?;
```

### Optional Enhancement (ADAPT-HL-002)
For harvesting market-wide liquidations, this requires tracking addresses:
- Monitor known addresses via userEvents subscriptions
- Or use third-party infrastructure (Dwellir gRPC) for public feed

### Decision Required
Harvesting system needs to decide:
1. Skip public liquidations (not available natively)
2. Use multi-user subscription approach (track known addresses)
3. Integrate third-party liquidation data source

## Next Task Ready
ADAPT-HL-002: Implement liquidation event processing for harvesting

However, scope needs clarification based on findings:
- **Option A**: Skip Hyperliquid liquidations (no public feed)
- **Option B**: Implement user-based tracking (requires address list)
- **Option C**: Integrate Dwellir or similar service (additional dependency)

## References

1. Hyperliquid Official WebSocket API: https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/api/websocket/subscriptions
2. NautilusTrader Implementation: crates/adapters/hyperliquid/src/websocket/
3. Comprehensive Analysis: .orchestration/hyperliquid-liquidations-analysis.md

## Conclusion

Task ADAPT-HL-001 is COMPLETE with clear findings: Hyperliquid does not provide public liquidation streams. User-specific liquidations are already fully implemented. The harvesting system design needs to accommodate this limitation.
