# Wave 2 Task Launch Plan

## Ready to Launch (after Wave 1 completes)

### Infrastructure Lane
- **INFRA-002**: Define Iceberg schema types (depends on INFRA-001)
- **INFRA-003**: Implement IcebergConfig (depends on INFRA-001)

### L3 Engine Lane
- **L3-002**: Implement synthetic order ID generator (depends on L3-001)

### Bitget Lane
- **ADAPT-BITGET-002**: Create Bitget Rust crate skeleton (depends on ADAPT-BITGET-001)

### Kraken Lane
- **ADAPT-KRAKEN-002**: Implement Kraken orderbook delta parsing (depends on ADAPT-KRAKEN-001)
- **ADAPT-KRAKEN-003**: Implement Kraken trade tick parsing (depends on ADAPT-KRAKEN-001)

### Hyperliquid Lane
- **ADAPT-HL-002**: Implement Hyperliquid public liquidation subscription (depends on ADAPT-HL-001)

## Parallelization Strategy

Wave 2 can run 7 tasks in parallel once Wave 1 completes:

1. **INFRA-002** + **INFRA-003** (both depend only on INFRA-001)
2. **L3-002** (depends only on L3-001)
3. **ADAPT-BITGET-002** (depends only on ADAPT-BITGET-001)
4. **ADAPT-KRAKEN-002** + **ADAPT-KRAKEN-003** (both depend only on ADAPT-KRAKEN-001, can run in parallel)
5. **ADAPT-HL-002** (depends only on ADAPT-HL-001)

## Wave 3 Dependencies

After Wave 2:
- **INFRA-004**: Needs INFRA-002
- **INFRA-005**: Needs INFRA-002 + INFRA-004
- **L3-003**: Needs L3-001 + L3-002
- **ADAPT-BITGET-003**: Needs ADAPT-BITGET-002
- **ADAPT-BITGET-005**: Needs ADAPT-BITGET-002

## Estimated Timeline

- Wave 1: ~2h (5 tasks in parallel)
- Wave 2: ~2-4h (7 tasks in parallel)
- Wave 3: ~4-6h (10+ tasks in parallel)
- Wave 4: Harvester integration (depends on all prior waves)
