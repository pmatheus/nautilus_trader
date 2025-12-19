# Task: ADAPT-BITGET-001 - Create Bitget Python adapter skeleton

## Objective
Initialize the Bitget adapter directory structure following the template pattern used by other exchange adapters in nautilus_trader.

## Context
Bitget is a new exchange integration needed for the harvesting system. This adapter will provide L2 orderbook data, trades, and liquidations.

## Files to Create
1. `nautilus_trader/adapters/bitget/__init__.py` - Package initialization
2. `nautilus_trader/adapters/bitget/config.py` - Configuration classes
3. `nautilus_trader/adapters/bitget/constants.py` - Exchange constants (endpoints, etc.)
4. `nautilus_trader/adapters/bitget/enums.py` - Bitget-specific enums

## Requirements
- Study existing adapter patterns: `nautilus_trader/adapters/bybit/`, `nautilus_trader/adapters/kraken/`
- Follow naming conventions and structure exactly
- Include proper type hints and docstrings
- Set up for future components: `providers.py`, `data.py`, `factories.py`

## Report Format
Create `.orchestration/agent-tasks/ADAPT-BITGET-001-report.md` with:
- Status: SUCCESS/FAILED
- Files created with paths
- Pattern analysis (which adapter was used as template)
- Next tasks ready: ADAPT-BITGET-002

## Success Criteria
- Package imports without errors
- Structure matches existing adapter patterns
- Ready for Rust crate development (ADAPT-BITGET-002)
