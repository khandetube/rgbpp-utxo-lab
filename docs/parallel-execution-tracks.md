# Parallel execution tracks

This project intentionally keeps independent mining paths separate so that one experiment does not destabilize another.

## Track A — Stratum V1
Existing Stratum V1 worker, reconnect, share accounting and BIP310 work remain isolated.

## Track B — Bitcoin Core GBT
Independent Mainnet/Testnet GBT miner path. It requires an operator-controlled Bitcoin Core RPC endpoint.

## Track C — Regtest
Deterministic local block-production and payout verification. No public-network funds are involved.

## Track D — CPU proof-of-work benchmark
The pow-benchmark binary measures the SHA256d implementation on the compute environment where it is actually executed. A benchmark is telemetry, not Bitcoin hashrate connected to a pool.

## Track E — GPU backend
GPU acceleration is kept as a separate backend boundary. A CUDA/OpenCL-capable host can implement the backend without changing the Stratum protocol state machine.

## Track F — Pool/route supervision
Logical routes, bounded live sessions, backoff and telemetry remain independent from the hashing implementation.

No track claims real BTC production unless a real authorized worker, accepted shares, and, if a block is claimed, independent on-chain confirmation are observed.
