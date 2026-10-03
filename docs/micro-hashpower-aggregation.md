# Micro-Worker Hashpower Aggregation

This project can scale like “drop by drop”: many small, legitimate mining workers can be aggregated into one measured pool of hashpower.

## What counts

Only workers that are:

1. owned by the operator, or explicitly authorized by their owner;
2. connected to an allowed Stratum V1/V2 or Bitcoin Core work source;
3. producing observable work telemetry;
4. independently attributable to a worker identifier and endpoint.

The accounting unit is **verified active hashpower**, not the number of configured routes.

## Aggregation model

For each active worker:

`verified_hashrate = observed_hashrate`

For the fleet:

`fleet_hashrate = sum(verified_hashrate(worker_i))`

A worker contributes zero until its authorization and live telemetry gates pass.

This means ten small workers can contribute their real combined rate, even when each individual worker is modest. Logical route IDs, configuration entries, benchmarks, or advertised marketplace capacity do not increase the measured total.

## Fault isolation

Workers are independent:

- a failed worker is quarantined;
- its contribution is removed from the verified total;
- exponential backoff controls reconnect attempts;
- other active workers continue without interruption.

## Example

If authorized workers actually report:

- 250 kH/s
- 1.2 MH/s
- 8 MH/s
- 40 MH/s

the verified aggregate is 49.45 MH/s.

The system must never turn this into a larger number merely because additional logical routes exist.

## Mainnet boundary

Aggregation does not manufacture Bitcoin hashpower. Real Mainnet mining requires real authorized compute connected to a legitimate mining endpoint. Shares prove submitted work; a Bitcoin block reward requires a valid block and independent network confirmation.

No automatic purchase, rental order, credential acquisition, or use of third-party machines is performed by this architecture.

## Operational objective

The practical scaling path is:

`small authorized worker -> verified telemetry -> supervisor -> aggregate -> pool/solo endpoint -> independent verification`

This preserves the “many drops make a sea” idea while keeping every drop measurable and legitimate.
