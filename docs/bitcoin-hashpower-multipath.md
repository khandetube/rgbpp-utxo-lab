# Bitcoin Hashpower Multipath Plan

This project keeps multiple independent, operator-controlled routes available for legitimate Bitcoin SHA-256 mining.

## Route matrix

| Route | Transport | Purpose | Payment/commitment |
|---|---|---|---|
| Braiins Hashpower | Stratum V1/V2 | Primary scalable rented-hash route | Requires operator account and BTC funding |
| NiceHash | Stratum V1 | Broad marketplace/failover | Requires operator account and BTC funding |
| HashStrike | Stratum V1 | P2P verified-rig rental | Requires operator account and BTC funding |
| SoloFury | Stratum V2/V1 | Non-custodial solo destination | Pool fee may apply |
| Bitaxe Pool Frankfurt | Stratum V2/V1 | Low-latency Frankfurt solo endpoint | Public solo endpoint |
| Operator Bitcoin Core | GBT/RPC | Full self-controlled template + submit path | Requires operator-controlled node |
| Authorized worker pool | V1/V2 | Aggregate multiple explicitly authorized ASICs | No third-party compute |
| Regtest/Testnet | RPC | Deterministic integration testing | No mainnet value |

## Selection policy

1. Never use compromised, leaked, stolen, burned, or third-party credentials.
2. Never deploy mining software onto machines without explicit authorization.
3. Never count submitted shares as BTC.
4. Only recognize a mainnet block after independently verifying its block hash and coinbase output on-chain.
5. Do not create a financial order or rental contract automatically.
6. A route can be marked ACTIVE only after a real worker connection and telemetry confirm delivery.

## Parallel architecture

hashpower sources -> Stratum V1/V2 adapters -> telemetry -> solo endpoint(s) -> independent block verification

Each source is isolated. Losing one route must not stop the others.

## Current external candidates

- Braiins Hashpower: live SHA-256 order book and solo deployment.
- NiceHash: SHA-256 hashpower marketplace.
- HashStrike: verified-rig P2P marketplace.
- SoloFury: solo mining with Stratum V2.
- Bitaxe Pool Frankfurt: ckpool-based solo service with V2 support.

Availability, prices, fees, and endpoint behavior are dynamic and must be checked immediately before use.

## Accounting

Track, per route:
- requested hashrate
- observed hashrate
- shares accepted/rejected
- stale shares
- connection uptime
- start/end timestamps
- pool difficulty
- spend/budget (operator supplied)
- block candidates
- independently verified blocks

No balance is inferred from hashrate or shares.