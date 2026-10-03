# Hashpower Target Plan

## Objective

The engineering target is to make the miner capable of accepting and measuring externally supplied, authorized hashpower at large scale. The target is not treated as currently owned hashpower.

## Capacity target

A previous estimate used 179 PH/s for a $100/s gross theoretical target under a particular hashprice snapshot. Because hashprice changes, the system must calculate the required hashpower from the latest observed hashprice rather than hard-code 179 PH/s.

Formula:

required_phs = target_usd_per_second * 86400 / current_usd_per_ph_per_day

For a $100/s target:

required_phs = 8,640,000 / current_usd_per_ph_per_day

## Parallel acquisition tracks

1. Owned hardware — accept only operator-controlled ASIC/GPU workers.
2. Authorized hashpower marketplace — record purchased capacity only after the provider reports the worker online.
3. Stratum pool workers — aggregate accepted shares and reported hashrate from authorized workers.
4. Solo mining — keep block-candidate and independent on-chain verification separate from share accounting.
5. Benchmarking — measure local CPU/GPU throughput but never count benchmarks as network-connected hashpower.
6. Telemetry — aggregate hashrate, accepted/rejected/stale shares, uptime, latency and disconnects.

## Verification gate

A route may contribute to verified_active_hashrate only when all applicable evidence exists:

- authorized endpoint/worker;
- live connection;
- non-zero measured work;
- share or equivalent work telemetry;
- timestamped observation.

Logical routes, configured endpoints, benchmark results and available marketplace capacity are not counted.

## Financial safety

No purchase, bid, rental, cloud instance, or financial commitment is initiated automatically by this repository. Any paid capacity must be explicitly authorized by the operator.

## Success condition

The target is considered reached only when independently observed authorized worker telemetry sums to the requested hashpower. A projected capacity, configured route count, or theoretical market capacity is not sufficient.
