# Task: ADAPT-HL-001 - Analyze Hyperliquid liquidation stream options

## Objective
Research whether Hyperliquid provides a public liquidation stream versus user-specific events, and document findings for implementation.

## Context
The harvesting system needs to capture liquidation events from Hyperliquid. Need to determine if there's a public feed or if we need alternative approaches.

## Research Tasks
1. Review Hyperliquid API documentation
2. Examine existing websocket enums in `crates/adapters/hyperliquid/src/websocket/enums.rs`
3. Check if liquidations are in existing subscriptions
4. Research Hyperliquid community/docs for liquidation data access

## Files to Review
1. `crates/adapters/hyperliquid/src/websocket/enums.rs` - Subscription types
2. `crates/adapters/hyperliquid/src/websocket/client.rs` - WebSocket client
3. Hyperliquid API documentation (external)

## Report Format
Create `.orchestration/agent-tasks/ADAPT-HL-001-report.md` with:
- Status: SUCCESS/FAILED
- Findings: Public stream available? (YES/NO/PARTIAL)
- Implementation approach recommendation
- WebSocket subscription type identified (if exists)
- Next tasks ready: ADAPT-HL-002

## Success Criteria
- Clear answer on liquidation data availability
- Implementation path identified
- Documentation saved to `.orchestration/hyperliquid-liquidations-analysis.md`
