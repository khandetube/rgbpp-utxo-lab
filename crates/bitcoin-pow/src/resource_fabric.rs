use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceKind {
    OwnedWorker,
    AuthorizedWorker,
    HashpowerMarket,
    PublicPool,
    Testnet,
    Regtest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceState {
    Discovered,
    Authorized,
    Connected,
    Verified,
    Rejected,
}

#[derive(Debug, Clone)]
pub struct ResourceOffer {
    pub id: String,
    pub kind: ResourceKind,
    pub endpoint: String,
    pub advertised_hashrate_hs: Option<u64>,
    pub state: ResourceState,
    pub operator_approved: bool,
}

impl ResourceOffer {
    pub fn eligible_for_verified_hashrate(&self, observed_hashrate_hs: u64) -> bool {
        self.operator_approved
            && matches!(self.state, ResourceState::Verified)
            && observed_hashrate_hs > 0
    }
}

#[derive(Debug, Default)]
pub struct ResourceFabric {
    resources: BTreeMap<String, ResourceOffer>,
}

impl ResourceFabric {
    pub fn register(&mut self, resource: ResourceOffer) {
        self.resources.insert(resource.id.clone(), resource);
    }

    pub fn set_state(&mut self, id: &str, state: ResourceState) -> bool {
        if let Some(resource) = self.resources.get_mut(id) {
            let valid = match state {
                ResourceState::Discovered => true,
                ResourceState::Authorized => resource.operator_approved,
                ResourceState::Connected => {
                    resource.operator_approved
                        && matches!(resource.state, ResourceState::Authorized | ResourceState::Connected)
                }
                ResourceState::Verified => {
                    resource.operator_approved
                        && matches!(resource.state, ResourceState::Connected | ResourceState::Verified)
                }
                ResourceState::Rejected => true,
            };
            if !valid {
                return false;
            }
            resource.state = state;
            true
        } else {
            false
        }
    }

    pub fn authorize(&mut self, id: &str) -> bool {
        if let Some(resource) = self.resources.get_mut(id) {
            resource.operator_approved = true;
            if resource.state == ResourceState::Discovered {
                resource.state = ResourceState::Authorized;
            }
            true
        } else {
            false
        }
    }

    pub fn verified_hashrate(&self, observed: &BTreeMap<String, u64>) -> u64 {
        self.resources.iter().fold(0u64, |total, (id, resource)| {
            let observed_hs = observed.get(id).copied().unwrap_or(0);
            if resource.eligible_for_verified_hashrate(observed_hs) {
                total.saturating_add(observed_hs)
            } else {
                total
            }
        })
    }

    pub fn len(&self) -> usize {
        self.resources.len()
    }

    pub fn is_empty(&self) -> bool {
        self.resources.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discovered_capacity_never_counts_as_verified() {
        let mut fabric = ResourceFabric::default();
        fabric.register(ResourceOffer {
            id: "market-1".into(),
            kind: ResourceKind::HashpowerMarket,
            endpoint: "example.invalid:3333".into(),
            advertised_hashrate_hs: Some(100),
            state: ResourceState::Discovered,
            operator_approved: false,
        });
        let mut observed = BTreeMap::new();
        observed.insert("market-1".into(), 100);
        assert_eq!(fabric.verified_hashrate(&observed), 0);
    }

    #[test]
    fn invalid_state_transitions_are_rejected() {
        let mut fabric = ResourceFabric::default();
        fabric.register(ResourceOffer {
            id: "worker-1".into(),
            kind: ResourceKind::OwnedWorker,
            endpoint: "127.0.0.1:3333".into(),
            advertised_hashrate_hs: None,
            state: ResourceState::Discovered,
            operator_approved: false,
        });
        assert!(!fabric.set_state("worker-1", ResourceState::Connected));
        assert!(!fabric.set_state("worker-1", ResourceState::Verified));
        assert!(fabric.authorize("worker-1"));
        assert!(fabric.set_state("worker-1", ResourceState::Connected));
        assert!(fabric.set_state("worker-1", ResourceState::Verified));
    }

    #[test]
    fn only_authorized_verified_workers_count() {
        let mut fabric = ResourceFabric::default();
        fabric.register(ResourceOffer {
            id: "worker-1".into(),
            kind: ResourceKind::OwnedWorker,
            endpoint: "127.0.0.1:3333".into(),
            advertised_hashrate_hs: None,
            state: ResourceState::Discovered,
            operator_approved: false,
        });
        assert!(fabric.authorize("worker-1"));
        assert!(!fabric.set_state("worker-1", ResourceState::Verified));
        assert!(fabric.set_state("worker-1", ResourceState::Connected));
        assert!(fabric.set_state("worker-1", ResourceState::Verified));

        let mut observed = BTreeMap::new();
        observed.insert("worker-1".into(), 1_000);
        assert_eq!(fabric.verified_hashrate(&observed), 1_000);
    }
}
