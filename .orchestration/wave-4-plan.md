# Wave 4 Execution Plan: Integration & Completion

**Date**: 2025-12-19
**Status**: Ready to Execute
**Prerequisites**: Wave 3 Complete (5,364 LOC, 50 tests, 3 lanes at 100%)

## Objective

Complete the L3 Orderbook Harvesting System by:
1. Fixing Bitget compilation issues (critical blocker)
2. Completing Infrastructure catalog integration
3. Finishing L3 Engine state management and emitter
4. Integrating Bitget parsing and Python client
5. Building the Harvester service
6. Adding comprehensive testing and documentation

---

## Wave 4 Structure

### Phase 4A: Critical Fixes & Completions (Parallel - ~2-3h)
**Priority**: CRITICAL - Unblocks all downstream work

#### Lane 1: Bitget Compilation Fixes (CRITICAL PATH)
**Agent**: rust-expert
**Time Estimate**: 20-30 minutes
**Tasks**:
- Fix HttpClient API calls to use `request()` methods
- Fix `Symbol::new()` usage (returns Symbol, not Result)
- Remove unused imports
- Verify compilation with `cargo build -p nautilus-bitget`
- Run existing 13 tests

**Success Criteria**: Clean compilation, all 13 tests passing

#### Lane 2: Infrastructure Catalog Integration
**Agent**: python-dev-expert OR rust-expert
**Time Estimate**: 1-2h
**Tasks**:
- INFRA-007: Implement Iceberg catalog REST client
- INFRA-008: Create IcebergSink unified writer
- INFRA-009: Add Python bindings for IcebergSink
- INFRA-010: Write comprehensive unit tests

**Success Criteria**: IcebergSink fully functional with Python bindings

#### Lane 3: L3 Engine Completion
**Agent**: rust-expert
**Time Estimate**: 1.5-2h
**Tasks**:
- L3-004: Implement local orderbook state manager
- L3-005: Implement L3 delta event emitter
- L3-006: Create L3ReconstructionEngine main struct
- L3-007: Handle exchange-specific quirks
- L3-008: Add Python bindings
- L3-009: Write unit tests

**Success Criteria**: L3ReconstructionEngine complete with Python bindings

---

### Phase 4B: Bitget Integration (Sequential after 4A - ~1.5-2h)
**Depends On**: Lane 1 (Bitget fixes) complete

**Agent**: rust-expert + python-dev-expert
**Tasks**:
- ADAPT-BITGET-006: Implement orderbook delta parsing
- ADAPT-BITGET-007: Implement trade tick parsing
- ADAPT-BITGET-008: Implement liquidation parsing
- ADAPT-BITGET-009: Create Python data client
- ADAPT-BITGET-010: Create factory functions
- ADAPT-BITGET-011: Add PyO3 bindings
- ADAPT-BITGET-012: Write adapter tests

**Success Criteria**: Bitget adapter fully integrated with Python client

---

### Phase 4C: Harvester Service (Sequential after 4A+4B - ~2-3h)
**Depends On**: INFRA-009, L3-008, ADAPT-BITGET-010

**Agent**: python-dev-expert
**Tasks**:
- HARV-001: Create harvester package structure
- HARV-002: Implement configuration system
- HARV-003: Create multi-exchange client manager
- HARV-004: Implement subscription orchestrator
- HARV-005: Integrate L3 reconstruction
- HARV-006: Implement event routing to Iceberg sinks
- HARV-007: Implement graceful reconnection handling
- HARV-008: Create OrderbookHarvester main class
- HARV-009: Add monitoring and metrics
- HARV-010: Create CLI entry point

**Success Criteria**: Functional harvester service with CLI

---

### Phase 4D: Testing & Documentation (Parallel with 4C - ~2h)

#### Lane 1: Testing
**Agent**: python-dev-expert
**Tasks**:
- TEST-001: End-to-end integration tests
- TEST-002: Performance benchmarks
- ADAPT-KRAKEN-005: Kraken integration tests

**Success Criteria**: Comprehensive test coverage, performance validated

#### Lane 2: Documentation
**Agent**: python-dev-expert
**Tasks**:
- DOC-001: Harvester user documentation
- DOC-002: Bitget adapter documentation

**Success Criteria**: Complete user-facing documentation

---

## Execution Strategy

### Stage 1: Critical Path (IMMEDIATE)
```
START → Fix Bitget Compilation (20 min)
     ↓
     ✓ Unblocks all Bitget integration work
```

### Stage 2: Parallel Completion (After Stage 1)
```
Lane A: Infrastructure (INFRA-007→010)
Lane B: L3 Engine (L3-004→009)
Lane C: Bitget Integration (ADAPT-BITGET-006→012) [After fixes]
```

### Stage 3: Harvester Build (After Stage 2)
```
Sequential: HARV-001 → HARV-010
Parallel: Testing (TEST-*) + Documentation (DOC-*)
```

---

## Agent Assignments

| Agent | Primary Tasks | Estimated Time |
|-------|--------------|----------------|
| **rust-expert** | Bitget fixes, INFRA-007/008, L3-004/005/006 | 3-4h |
| **python-dev-expert** | INFRA-009, L3-008, ADAPT-BITGET-009/010, HARV-* | 4-5h |
| **python-dev-expert** | Testing, Documentation | 2h |

**Total Elapsed Time**: ~6-8 hours (with parallelization)
**Total Sequential Time**: ~15-18 hours

---

## Risk Mitigation

### High Risk
- **Bitget Compilation**: Quick manual fixes needed (20 min)
- **Iceberg Catalog Integration**: May need Supabase API research

### Medium Risk
- **L3 Engine State Management**: Complex logic, needs careful testing
- **Harvester Multi-Exchange**: Coordination complexity

### Low Risk
- **Python Bindings**: Well-established patterns
- **Documentation**: Straightforward

---

## Success Metrics

### Code Delivery
- [ ] ~3,000-4,000 additional LOC
- [ ] ~30-40 new tests
- [ ] Clean compilation across all crates
- [ ] All tests passing (current: 58+13+11 = 82, target: 120+)

### Functional
- [ ] Bitget adapter fully operational
- [ ] L3 reconstruction engine complete
- [ ] Iceberg sink writing to Supabase
- [ ] Harvester service running with CLI
- [ ] End-to-end data flow working

### Quality
- [ ] Zero compilation warnings
- [ ] Performance benchmarks documented
- [ ] User documentation complete
- [ ] Integration tests passing

---

## Notes

1. **Bitget fixes are CRITICAL PATH** - must complete before any Bitget integration
2. Infrastructure and L3 Engine can proceed in parallel with Bitget fixes
3. Harvester MUST wait for INFRA-009, L3-008, and ADAPT-BITGET-010
4. Testing can begin as soon as components are complete
5. Documentation can be written in parallel with final integration

**Expected Wave 4 Completion**: 6-8 hours elapsed time (~15-18h sequential work)
