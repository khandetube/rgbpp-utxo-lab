use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RouteState {
    Pending,
    Connecting,
    Active,
    Quarantined,
}

#[derive(Debug, Clone)]
pub struct RouteSpec {
    pub id: u64,
    pub shard: u32,
    pub endpoint: String,
}

#[derive(Debug)]
pub struct RouteTelemetry {
    pub state: RouteState,
    pub attempts: u64,
    pub successful_connections: u64,
    pub errors: u64,
    pub shares: u64,
    pub accepted: u64,
    pub rejected: u64,
    pub last_change: Instant,
}

impl RouteTelemetry {
    fn new() -> Self {
        Self {
            state: RouteState::Pending,
            attempts: 0,
            successful_connections: 0,
            errors: 0,
            shares: 0,
            accepted: 0,
            rejected: 0,
            last_change: Instant::now(),
        }
    }
}

#[derive(Debug)]
pub struct RouteTable {
    routes: HashMap<u64, RouteTelemetry>,
    pub shard_count: u32,
}

impl RouteTable {
    pub fn with_capacity(route_count: u64, shard_count: u32) -> Self {
        let shard_count = shard_count.max(1);
        let mut routes = HashMap::with_capacity(route_count.min(usize::MAX as u64) as usize);
        for id in 0..route_count {
            routes.insert(id, RouteTelemetry::new());
        }
        Self { routes, shard_count }
    }

    pub fn len(&self) -> usize { self.routes.len() }

    pub fn shard_for(&self, route_id: u64) -> u32 {
        (route_id % self.shard_count as u64) as u32
    }

    pub fn mark_connecting(&mut self, route_id: u64) {
        if let Some(route) = self.routes.get_mut(&route_id) {
            route.state = RouteState::Connecting;
            route.attempts = route.attempts.saturating_add(1);
            route.last_change = Instant::now();
        }
    }

    pub fn mark_active(&mut self, route_id: u64) {
        if let Some(route) = self.routes.get_mut(&route_id) {
            route.state = RouteState::Active;
            route.successful_connections = route.successful_connections.saturating_add(1);
            route.last_change = Instant::now();
        }
    }

    pub fn mark_error(&mut self, route_id: u64) {
        if let Some(route) = self.routes.get_mut(&route_id) {
            route.state = RouteState::Quarantined;
            route.errors = route.errors.saturating_add(1);
            route.last_change = Instant::now();
        }
    }

    pub fn record_share(&mut self, route_id: u64, accepted: bool) {
        if let Some(route) = self.routes.get_mut(&route_id) {
            route.shares = route.shares.saturating_add(1);
            if accepted {
                route.accepted = route.accepted.saturating_add(1);
            } else {
                route.rejected = route.rejected.saturating_add(1);
            }
        }
    }

    pub fn counts(&self) -> (usize, usize, usize, usize) {
        let mut pending = 0;
        let mut connecting = 0;
        let mut active = 0;
        let mut quarantined = 0;
        for route in self.routes.values() {
            match route.state {
                RouteState::Pending => pending += 1,
                RouteState::Connecting => connecting += 1,
                RouteState::Active => active += 1,
                RouteState::Quarantined => quarantined += 1,
            }
        }
        (pending, connecting, active, quarantined)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Backoff {
    pub base: Duration,
    pub max: Duration,
}

impl Backoff {
    pub fn delay(&self, failures: u32) -> Duration {
        let shift = failures.min(20);
        let multiplier = 1u32 << shift;
        self.base.saturating_mul(multiplier).min(self.max)
    }
}

#[derive(Debug)]
pub struct OrchestratorConfig {
    pub route_count: u64,
    pub shard_count: u32,
    pub max_live_sessions: usize,
    pub endpoint: String,
}

#[derive(Debug)]
pub struct Orchestrator {
    pub table: RouteTable,
    pub config: OrchestratorConfig,
    live_sessions: AtomicU64,
}

impl Orchestrator {
    pub fn new(config: OrchestratorConfig) -> Self {
        Self {
            table: RouteTable::with_capacity(config.route_count, config.shard_count),
            config,
            live_sessions: AtomicU64::new(0),
        }
    }

    pub fn route(&self, id: u64) -> Option<RouteSpec> {
        if id >= self.config.route_count {
            return None;
        }
        Some(RouteSpec {
            id,
            shard: self.table.shard_for(id),
            endpoint: self.config.endpoint.clone(),
        })
    }

    pub fn try_acquire_session(&self) -> bool {
        let max = self.config.max_live_sessions.max(1) as u64;
        loop {
            let current = self.live_sessions.load(Ordering::Acquire);
            if current >= max {
                return false;
            }
            if self.live_sessions.compare_exchange(
                current, current + 1, Ordering::AcqRel, Ordering::Acquire
            ).is_ok() {
                return true;
            }
        }
    }

    pub fn release_session(&self) {
        let _ = self.live_sessions.fetch_update(Ordering::AcqRel, Ordering::Acquire, |v| {
            Some(v.saturating_sub(1))
        });
    }

    pub fn live_sessions(&self) -> u64 {
        self.live_sessions.load(Ordering::Acquire)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_one_hundred_thousand_logical_routes_without_opening_connections() {
        let orchestrator = Orchestrator::new(OrchestratorConfig {
            route_count: 100_000,
            shard_count: 1_000,
            max_live_sessions: 100,
            endpoint: "127.0.0.1:3333".into(),
        });
        assert_eq!(orchestrator.table.len(), 100_000);
        assert_eq!(orchestrator.route(99_999).unwrap().shard, 999);
        assert!(orchestrator.route(100_000).is_none());
        assert_eq!(orchestrator.live_sessions(), 0);
    }

    #[test]
    fn session_cap_is_hard() {
        let orchestrator = Orchestrator::new(OrchestratorConfig {
            route_count: 10,
            shard_count: 2,
            max_live_sessions: 2,
            endpoint: "127.0.0.1:3333".into(),
        });
        assert!(orchestrator.try_acquire_session());
        assert!(orchestrator.try_acquire_session());
        assert!(!orchestrator.try_acquire_session());
        orchestrator.release_session();
        assert!(orchestrator.try_acquire_session());
    }

    #[test]
    fn backoff_is_bounded() {
        let backoff = Backoff {
            base: Duration::from_secs(1),
            max: Duration::from_secs(30),
        };
        assert_eq!(backoff.delay(0), Duration::from_secs(1));
        assert_eq!(backoff.delay(3), Duration::from_secs(8));
        assert_eq!(backoff.delay(10), Duration::from_secs(30));
    }

    #[test]
    fn telemetry_transitions_are_deterministic() {
        let mut table = RouteTable::with_capacity(4, 2);
        table.mark_connecting(3);
        table.mark_active(3);
        table.record_share(3, true);
        table.record_share(3, false);
        let (_, _, active, _) = table.counts();
        assert_eq!(active, 1);
    }
}
