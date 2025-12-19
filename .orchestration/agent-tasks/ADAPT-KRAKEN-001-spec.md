# Task: ADAPT-KRAKEN-001 - Verify Kraken WebSocket subscribe_book implementation

## Objective
Review and test existing Kraken adapter's subscribe_book and subscribe_trades methods to ensure they're production-ready for the harvesting system.

## Context
Kraken adapter already exists but needs verification that orderbook and trade subscriptions work correctly for continuous data harvesting.

## Files to Review
1. `crates/adapters/kraken/src/websocket/client.rs` - WebSocket client
2. `crates/adapters/kraken/src/websocket/handler.rs` - Message handler
3. `nautilus_trader/adapters/kraken/data.py` - Python data client

## Requirements
- Verify subscribe_book method exists and works
- Verify subscribe_trades method exists and works
- Check message parsing completeness
- Test reconnection handling
- Document any gaps for ADAPT-KRAKEN-002/003

## Report Format
Create `.orchestration/agent-tasks/ADAPT-KRAKEN-001-report.md` with:
- Status: SUCCESS/FAILED
- Current implementation state (complete/partial/missing)
- Test results (if tested)
- Gaps identified for next tasks
- Next tasks ready: ADAPT-KRAKEN-002, ADAPT-KRAKEN-003

## Success Criteria
- Clear understanding of what's implemented
- Gaps documented for follow-up tasks
- No breaking issues found
