# Bitcoin multi-pool routing

The miner supports a comma-separated `STRATUM_URLS` failover list. A single ASIC/CPU worker should not be duplicated across multiple pools at the same time unless the operator intentionally partitions hardware; otherwise the same hashpower is simply divided between destinations.

## Publicly documented solo-capable routes

| Route | Protocol | Endpoint | Mode | Notes |
|---|---|---|---|---|
| Solo CKPool | Stratum V1 | `stratum.ckpool.org:3333` | Solo | BTC address as username; 2% fee on a found block |
| Solo CKPool | Stratum V2 | `stratum.ckpool.org:3336` | Solo | Requires SV2-compatible client |
| Public Pool | Stratum V1 | `public-pool.io:3333` | Solo | Publicly documented direct-payout solo route |
| Public Pool | Stratum V1 TLS | `public-pool.io:4333` | Solo | TLS endpoint |
| Public Pool | Stratum V2 | `public-pool.io:23330` | Solo | Requires SV2-compatible client |

## Example failover

    export STRATUM_URLS='stratum.ckpool.org:3333,public-pool.io:3333'
    export STRATUM_USERNAME='YOUR_OPERATOR_CONTROLLED_BTC_ADDRESS'
    export STRATUM_PASSWORD='x'
    cargo run -p bitcoin-pow --bin stratum-miner --release

The order is deterministic. The miner connects to one endpoint at a time and advances to the next route after a disconnect, protocol error or server-requested reconnect.

## Commercial pool routes

Braiins Pool, Foundry USA, F2Pool, ViaBTC, AntPool and other commercial pools can be supported by the same supervisor, but they generally require account/worker credentials and use their own payout schemes. Credentials are never guessed, scraped, or committed to this repository. A route is marked ACTIVE only after an operator-controlled worker has successfully subscribed and authorized.

## Hashpower marketplaces

Hashpower rental services are a separate class from mining pools. They must not be treated as free compute. Any paid order requires explicit operator authorization, a defined budget and independent verification of delivered hashrate before activation.

## Operational rules

1. Only machines and credentials explicitly controlled by the operator are allowed.
2. Shares are telemetry; a share is not a Bitcoin payout.
3. A block candidate is not a confirmed block until independently verified on Bitcoin.
4. A failover list improves availability; it does not multiply hashrate.
5. Pool endpoints and fees can change, so the live pool documentation must be checked before production use.
6. No Mainnet spend, rental order or financial commitment is performed automatically.

## Current project objective

The supervisor should expose one logical worker identity, bounded concurrent sessions, per-route health, share acceptance/rejection, reconnect backoff and independent block verification. The route catalog is deliberately configuration-driven so new legitimate pools can be added without changing mining logic.
