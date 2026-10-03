# Hashpower Scaling Plan

This document tracks independent paths for increasing **verified, authorized** Bitcoin mining hashpower toward a 179 PH/s target.

## Parallel tracks

1. **Owned ASIC track** — register each operator-controlled worker, record advertised and observed hashrate, and verify accepted shares.
2. **Hashpower-market track** — evaluate only authorized marketplace capacity; record quoted capacity, price, duration, pool destination, and actual delivered hashrate before counting it.
3. **Pool-worker track** — connect authorized ASIC/worker capacity through Stratum V1/V2 and aggregate share telemetry.
4. **Solo-node track** — maintain the Bitcoin Core GBT path for independently constructing and validating block candidates.
5. **Benchmark track** — benchmark local CPU/GPU backends separately; benchmark numbers are never treated as deployed network hashrate.
6. **Observability track** — aggregate hashrate, accepted/rejected/stale shares, uptime, latency, reconnects, and route state.
7. **Verification track** — a capacity becomes VERIFIED_ACTIVE only after a real authorized worker connection produces timestamped telemetry and accepted shares.

## Target

The working target is approximately **179 PH/s** based on the earlier theoretical conversion from the requested $100/second figure. This is a planning target, not a current capability claim.

## Accounting rules

- Logical routes do not equal physical hashpower.
- A pool's published hashrate does not equal our hashrate.
- Submitted shares do not equal BTC rewards.
- A quoted marketplace order does not equal delivered capacity.
- No financial order or external account action is initiated automatically.
- No compromised, leaked, stolen, or third-party credentials are used.
- Mainnet BTC is counted only after independent on-chain confirmation.

## Current verified state

At the time this file was added, no externally authorized mining worker was connected to the project, so externally delivered Mainnet hashpower remains **0 H/s**. Local benchmark capacity is tracked separately.
