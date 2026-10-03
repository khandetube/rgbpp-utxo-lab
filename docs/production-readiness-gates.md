# Production Readiness Gates

This checklist defines what must be true before the mining fabric is called operational.

## Gate 1 — Build integrity
- Rust formatting passes.
- Workspace tests pass.
- Workspace check passes.
- Standalone miner binaries compile.
- No secrets are committed.

## Gate 2 — Work correctness
- Bitcoin Core GBT templates are parsed and validated.
- Previous-block changes invalidate old jobs.
- nBits/target conversion is checked.
- Candidate headers are independently verified before submission.
- Stratum share-target comparison uses the correct displayed hash byte order.

Bitcoin Core documents getblocktemplate as the interface for obtaining the data required to construct mining work. Stratum V2 defines mining jobs as work over a candidate block-header search space.

## Gate 3 — Resource integrity
A resource is counted in verified hashrate only when:
1. it is operator-authorized;
2. it is actually connected;
3. work is being observed;
4. telemetry reports non-zero hashrate;
5. stale/rejected/error conditions are tracked.

Configured routes, theoretical marketplace capacity, and disconnected workers do not count.

## Gate 4 — Scale
- Bounded live sessions.
- Deterministic sharding.
- Exponential backoff.
- Per-route telemetry.
- Duplicate-job protection.
- Graceful shutdown.
- Failure isolation.

## Gate 5 — Mainnet
Mainnet submission remains opt-in and requires an operator-controlled Bitcoin Core node. A found candidate must pass local validation and then independent on-chain verification.

## Operational truth

The platform may orchestrate and measure real compute, but it cannot manufacture physical hashpower. Real BTC production requires authorized hardware or purchased/contracted hashpower plus the normal probabilistic mining process.
