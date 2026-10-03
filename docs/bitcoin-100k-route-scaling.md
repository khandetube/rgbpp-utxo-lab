# 100K+ Logical Mining Route Scaling

The controller may represent hundreds of thousands of logical routes without opening hundreds of thousands of network connections.

## Design

- 100,000+ deterministic route IDs are generated from a seed.
- Routes are sharded across a bounded worker pool.
- Each live route has health, endpoint, protocol, hashrate, share, latency, and error counters.
- Endpoint connections are pooled and capped by operator configuration.
- Failed routes are quarantined with exponential backoff.
- Route state is persisted as telemetry, not as wallet credentials.
- A route is ACTIVE only after a real authorized worker connection is observed.

## Important distinction

100,000 logical routes are not 100,000 real mining resources. Real hashpower still requires authorized hardware or rented capacity. The controller must never scan, install on, or consume compute from third-party machines.

## Suggested topology

100,000 logical routes
    -> 1,000 route shards
    -> 100 connection supervisors
    -> bounded Stratum V1/V2 sessions
    -> solo/pool destinations
    -> independent block verification

This allows very large orchestration tests while keeping resource usage bounded.

## Route identity

A route ID should be deterministic:

route-000000000001
route-000000000002
...
route-000100000000

The ID is metadata only. It does not represent ownership, credentials, or actual hashpower.

## Scaling tests

The test harness should validate:

1. creation of 100,000 route states;
2. deterministic assignment to shards;
3. bounded concurrent connections;
4. failure isolation;
5. exponential reconnect backoff;
6. telemetry aggregation;
7. duplicate-job avoidance;
8. graceful shutdown.

No test should create real external mining accounts or financial orders.
