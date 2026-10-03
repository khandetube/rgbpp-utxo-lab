# Live Hashrate Verification

This document defines the evidence required before the project reports external Bitcoin hashpower as real.

## Required evidence

A worker is counted only when an authorized mining endpoint reports a live connection and measurable work. The verifier records:

- worker identifier;
- endpoint;
- protocol;
- observed hashrate;
- accepted, rejected and stale shares;
- observation timestamp;
- reconnect/error state.

## Reporting rule

Configured routes, marketplace capacity, theoretical capacity, CPU benchmarks and logical route counts are excluded from verified hashpower.

verified_hashrate = sum(observed_hasrate of currently authorized active workers)

The value must remain zero until actual worker telemetry is available.

## Target calculation

For a gross target of $100 per second:

required_PH_s = 100 * 86400 / hashprice_USD_per_PH_day

At a $40.4/PH/day snapshot, this is approximately 213.9 PH/s.

This is a changing market calculation, not a claim that the project currently controls that capacity.

## Independent confirmation

For pool mining, accepted shares establish work performed but do not prove a Bitcoin block was found.

For solo mining, a candidate must additionally be accepted by an authorized Bitcoin node and independently verified on-chain before a block reward is counted.

## Safety

The verifier does not discover, install on, or consume compute from third-party machines. Paid hashrate requires explicit operator authorization and is not purchased automatically.
