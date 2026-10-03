# Sovereign Hashpower Architecture

## Goal

Build an independently operated Bitcoin SHA-256 hashing stack that can scale from one controlled worker to many authorized workers without pretending that software alone creates physical hashpower.

## Design

```
operator-controlled hardware
        |
        v
sha256d worker backend
        |
        +--> CPU backend
        +--> future CUDA/OpenCL backend
        |
        v
job scheduler
        |
        +--> Stratum V1/V2
        +--> Bitcoin Core GBT
        |
        v
route supervisor
        |
        v
telemetry + authorization
        |
        v
verified active hashrate
```

## What makes the design distinctive

1. **Backend equivalence:** every compute backend receives the same 80-byte Bitcoin header work model and must pass the same known-answer tests.
2. **Authorization-first accounting:** configured capacity is never counted as active capacity. Only an operator-authorized worker with live telemetry contributes to verified hashrate.
3. **Protocol-independent work model:** Stratum and Bitcoin Core GBT feed the same internal hashing interface.
4. **Fault isolation:** workers and routes are independently quarantined on repeated failures.
5. **Deterministic scaling:** work is partitioned by nonce start/stride so independently controlled workers can avoid duplicate nonce ranges.
6. **Proof-oriented telemetry:** accepted shares, rejected shares, timestamps, job identifiers and observed hashrate are retained as evidence of actual work.
7. **Mainnet safety:** Mainnet submission remains explicitly gated; no private keys are stored in source.

## Physical-hashpower boundary

This repository can implement the control plane, scheduler, hashing algorithms, telemetry and verification. It cannot manufacture ASIC silicon or expose hidden compute resources.

A worker becomes real hashpower only when an authorized physical or externally contracted compute resource is connected and measurable.

## Scaling path

- Stage 1: controlled CPU/ASIC worker
- Stage 2: optimized native/GPU backend
- Stage 3: multiple operator-controlled workers
- Stage 4: bounded external authorized workers
- Stage 5: independently verified aggregate hashrate

No stage represents a guaranteed BTC yield. Bitcoin mining remains probabilistic.

## Integrity rule

Never count theoretical marketplace capacity, benchmark results, logical routes, configured endpoints, or unconnected workers as owned or active hashrate.
