use async_trait::async_trait;
use dashmap::DashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use crate::registry::ServiceInstance;

/// Trait for selecting the next service instance from a list of candidates.
#[async_trait]
pub trait LoadBalancer: Send + Sync {
    async fn next_instance(&self, instances: &[ServiceInstance]) -> Option<ServiceInstance>;
}

// ---------------------------------------------------------------------------
// Round-Robin
// ---------------------------------------------------------------------------

/// Cycles through instances in order.
pub struct RoundRobin {
    counter: AtomicUsize,
}

impl RoundRobin {
    pub fn new() -> Self {
        Self {
            counter: AtomicUsize::new(0),
        }
    }
}

impl Default for RoundRobin {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl LoadBalancer for RoundRobin {
    async fn next_instance(&self, instances: &[ServiceInstance]) -> Option<ServiceInstance> {
        // TODO: use self.counter to pick the next instance in round-robin order
        let _ = instances;
        let _ = self.counter.fetch_add(1, Ordering::SeqCst);
        todo!("RoundRobin::next_instance")
    }
}

// ---------------------------------------------------------------------------
// Random
// ---------------------------------------------------------------------------

/// Picks a random instance from the list.
pub struct Random;

impl Random {
    pub fn new() -> Self {
        Self
    }
}

impl Default for Random {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl LoadBalancer for Random {
    async fn next_instance(&self, instances: &[ServiceInstance]) -> Option<ServiceInstance> {
        // TODO: pick a random instance
        let _ = instances;
        todo!("Random::next_instance")
    }
}

// ---------------------------------------------------------------------------
// Least Connections
// ---------------------------------------------------------------------------

/// Picks the instance with the fewest active connections.
pub struct LeastConnections {
    /// Maps instance ID to active connection count.
    pub connections: Arc<DashMap<String, usize>>,
}

impl LeastConnections {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(DashMap::new()),
        }
    }

    /// Increment the connection count for an instance.
    pub fn connect(&self, instance_id: &str) {
        self.connections
            .entry(instance_id.to_string())
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }

    /// Decrement the connection count for an instance.
    pub fn disconnect(&self, instance_id: &str) {
        self.connections.entry(instance_id.to_string()).and_modify(|c| {
            *c = c.saturating_sub(1);
        });
    }
}

impl Default for LeastConnections {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl LoadBalancer for LeastConnections {
    async fn next_instance(&self, instances: &[ServiceInstance]) -> Option<ServiceInstance> {
        // TODO: pick the instance with the fewest connections
        let _ = instances;
        todo!("LeastConnections::next_instance")
    }
}
