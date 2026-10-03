use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RouteState {
    Ready,
    Active,
    Backoff,
    Quarantined,
}

#[derive(Debug, Clone)]
pub struct PoolRoute {
    pub id: String,
    pub endpoint: String,
    pub priority: u32,
    pub state: RouteState,
    pub attempts: u64,
    pub accepted_shares: u64,
    pub rejected_shares: u64,
    pub last_error: Option<String>,
    next_retry: Instant,
}

impl PoolRoute {
    pub fn new(id: impl Into<String>, endpoint: impl Into<String>, priority: u32) -> Self {
        Self {
            id: id.into(),
            endpoint: endpoint.into(),
            priority,
            state: RouteState::Ready,
            attempts: 0,
            accepted_shares: 0,
            rejected_shares: 0,
            last_error: None,
            next_retry: Instant::now(),
        }
    }

    pub fn record_success(&mut self) {
        self.state = RouteState::Active;
        self.last_error = None;
    }

    pub fn record_share(&mut self, accepted: bool) {
        if accepted {
            self.accepted_shares = self.accepted_shares.saturating_add(1);
        } else {
            self.rejected_shares = self.rejected_shares.saturating_add(1);
        }
    }

    pub fn record_failure(&mut self, error: impl Into<String>, base: Duration, max: Duration) {
        self.attempts = self.attempts.saturating_add(1);
        self.state = RouteState::Backoff;
        self.last_error = Some(error.into());
        let shift = self.attempts.saturating_sub(1).min(31) as u32;
        let delay = base.checked_mul(1u32 << shift).unwrap_or(max).min(max);
        self.next_retry = Instant::now() + delay;
    }

    pub fn eligible(&self) -> bool {
        matches!(self.state, RouteState::Ready | RouteState::Backoff)
            && Instant::now() >= self.next_retry
    }
}

#[derive(Debug)]
pub struct PoolSupervisor {
    routes: HashMap<String, PoolRoute>,
    max_live_sessions: usize,
    live_sessions: usize,
    backoff_base: Duration,
    backoff_max: Duration,
}

impl PoolSupervisor {
    pub fn new(max_live_sessions: usize, backoff_base: Duration, backoff_max: Duration) -> Self {
        Self {
            routes: HashMap::new(),
            max_live_sessions: max_live_sessions.max(1),
            live_sessions: 0,
            backoff_base,
            backoff_max,
        }
    }

    pub fn add_route(&mut self, route: PoolRoute) {
        self.routes.insert(route.id.clone(), route);
    }

    pub fn route_count(&self) -> usize {
        self.routes.len()
    }

    pub fn live_sessions(&self) -> usize {
        self.live_sessions
    }

    pub fn try_acquire(&mut self) -> bool {
        if self.live_sessions >= self.max_live_sessions {
            return false;
        }
        self.live_sessions += 1;
        true
    }

    pub fn release(&mut self) {
        self.live_sessions = self.live_sessions.saturating_sub(1);
    }

    pub fn select_route(&mut self) -> Option<String> {
        self.routes.values()
            .filter(|r| r.eligible())
            .min_by_key(|r| (r.priority, r.attempts, r.id.clone()))
            .map(|r| r.id.clone())
    }

    pub fn mark_connected(&mut self, id: &str) -> bool {
        let Some(route) = self.routes.get_mut(id) else { return false; };
        route.record_success();
        true
    }

    pub fn mark_share(&mut self, id: &str, accepted: bool) -> bool {
        let Some(route) = self.routes.get_mut(id) else { return false; };
        route.record_share(accepted);
        true
    }

    pub fn mark_failed(&mut self, id: &str, error: impl Into<String>) -> bool {
        let Some(route) = self.routes.get_mut(id) else { return false; };
        route.record_failure(error, self.backoff_base, self.backoff_max);
        true
    }

    pub fn snapshot(&self) -> Vec<PoolRoute> {
        let mut routes: Vec<_> = self.routes.values().cloned().collect();
        routes.sort_by_key(|r| (r.priority, r.id.clone()));
        routes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounded_live_sessions() {
        let mut s = PoolSupervisor::new(2, Duration::from_millis(1), Duration::from_secs(1));
        assert!(s.try_acquire());
        assert!(s.try_acquire());
        assert!(!s.try_acquire());
        s.release();
        assert!(s.try_acquire());
    }

    #[test]
    fn route_failover_and_backoff() {
        let mut s = PoolSupervisor::new(1, Duration::from_millis(1), Duration::from_secs(1));
        s.add_route(PoolRoute::new("primary", "a:3333", 0));
        s.add_route(PoolRoute::new("backup", "b:3333", 1));
        assert_eq!(s.select_route().as_deref(), Some("primary"));
        s.mark_failed("primary", "connection refused");
        assert_eq!(s.select_route().as_deref(), Some("backup"));
    }

    #[test]
    fn share_accounting_is_per_route() {
        let mut s = PoolSupervisor::new(1, Duration::from_millis(1), Duration::from_secs(1));
        s.add_route(PoolRoute::new("p", "p:3333", 0));
        assert!(s.mark_share("p", true));
        assert!(s.mark_share("p", false));
        let r = &s.snapshot()[0];
        assert_eq!(r.accepted_shares, 1);
        assert_eq!(r.rejected_shares, 1);
    }
}
