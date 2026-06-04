use chrono::{DateTime, Utc};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;

/// A single instance of a service registered in the mesh.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInstance {
    pub id: String,
    pub service_name: String,
    pub address: String,
    pub last_heartbeat: DateTime<Utc>,
}

/// In-memory service registry backed by `DashMap` for lock-free concurrent
/// access.
#[derive(Debug, Clone)]
pub struct ServiceRegistry {
    instances: Arc<DashMap<String, ServiceInstance>>,
    heartbeat_timeout: Duration,
}

impl ServiceRegistry {
    /// Create a new registry. Instances that have not sent a heartbeat within
    /// `heartbeat_timeout` are considered unhealthy.
    pub fn new(heartbeat_timeout: Duration) -> Self {
        Self {
            instances: Arc::new(DashMap::new()),
            heartbeat_timeout,
        }
    }

    /// Register (or re-register) a service instance.
    pub fn register(&self, instance: ServiceInstance) {
        // TODO: insert the instance into the map keyed by its id
        let _ = instance;
        todo!("register")
    }

    /// Remove a service instance by ID.
    pub fn deregister(&self, instance_id: &str) {
        // TODO: remove the instance from the map
        let _ = instance_id;
        todo!("deregister")
    }

    /// Return all healthy instances whose `service_name` matches.
    pub fn discover(&self, service_name: &str) -> Vec<ServiceInstance> {
        // TODO: iterate the map, filter by service_name, collect
        let _ = service_name;
        todo!("discover")
    }

    /// Remove any instance whose `last_heartbeat` is older than the configured
    /// timeout.
    pub fn health_check(&self) {
        // TODO: iterate the map, remove stale entries
        let _ = self.heartbeat_timeout;
        todo!("health_check")
    }
}
