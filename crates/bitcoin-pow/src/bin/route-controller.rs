use bitcoin_pow::route_orchestrator::{Orchestrator, OrchestratorConfig};

fn main() {
    let route_count = std::env::var("LOGICAL_ROUTE_COUNT")
        .ok().and_then(|v| v.parse().ok()).unwrap_or(100_000u64);
    let shard_count = std::env::var("ROUTE_SHARD_COUNT")
        .ok().and_then(|v| v.parse().ok()).unwrap_or(1_000u32);
    let max_live_sessions = std::env::var("MAX_LIVE_STRATUM_SESSIONS")
        .ok().and_then(|v| v.parse().ok()).unwrap_or(100usize);
    let endpoint = std::env::var("STRATUM_URL")
        .unwrap_or_else(|_| "solo.ckpool.org:3333".into());

    let orchestrator = Orchestrator::new(OrchestratorConfig {
        route_count,
        shard_count,
        max_live_sessions,
        endpoint,
    });

    let first = orchestrator.route(0).expect("route 0 must exist");
    let last = orchestrator.route(route_count.saturating_sub(1))
        .expect("last route must exist when route_count > 0");

    println!("logical mining route controller");
    println!("logical_routes={}", orchestrator.table.len());
    println!("shards={}", shard_count.max(1));
    println!("max_live_sessions={}", max_live_sessions.max(1));
    println!("first_route={} shard={}", first.id, first.shard);
    println!("last_route={} shard={}", last.id, last.shard);
    println!("endpoint={}", first.endpoint);
    println!("live_sessions={}", orchestrator.live_sessions());
    println!("No external connections were opened by this controller.");
}
