# Hashpower Verification Schema

This is an independent telemetry contract for parallel mining-capacity tracks.

A worker is counted as VERIFIED_ACTIVE only when all fields below are backed by an actual authorized connection.

Required telemetry:
- worker_id
- route_id
- endpoint
- protocol
- observed_hashrate_hs
- accepted_shares
- rejected_shares
- stale_shares
- first_seen
- last_seen
- uptime_seconds
- difficulty
- network
- payout_destination

State transitions:
PENDING -> CONNECTING -> ACTIVE -> VERIFIED_ACTIVE
ACTIVE -> DEGRADED -> QUARANTINED

Rules:
- A configured endpoint is not proof of capacity.
- A pool's advertised hashrate is not our hashrate.
- A benchmark result is not deployed hashrate.
- A marketplace quote is not delivered capacity.
- VERIFIED_ACTIVE is revoked when telemetry expires.
- Mainnet rewards require independent blockchain confirmation.

Target capacity remains a planning objective; no unverified capacity is added to the project's total.
