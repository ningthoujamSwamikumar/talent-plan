//! Part 5: Audit Logging
//!
//! Structured audit logging: WHO did WHAT to WHICH resource WHEN from WHERE.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Represents an auditable action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditAction {
    Create,
    Read,
    Update,
    Delete,
    List,
}

/// A structured audit event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    /// Who performed the action (user ID, service name, or "anonymous").
    pub who: String,
    /// What action was performed.
    pub action: AuditAction,
    /// The type of resource acted upon (e.g., "note").
    pub resource_type: String,
    /// The ID of the specific resource (empty for list operations).
    pub resource_id: String,
    /// When the action occurred.
    pub when: DateTime<Utc>,
    /// Client IP address.
    pub ip_address: String,
    /// Optional additional context.
    pub details: Option<String>,
}

/// Trait for audit loggers.
///
/// TODO: Students implement this trait.
pub trait AuditLogger: Send + Sync {
    /// Log an audit event.
    fn log(&self, event: AuditEvent);

    /// Retrieve all logged events (for testing).
    fn events(&self) -> Vec<AuditEvent>;
}

/// In-memory audit logger for testing.
///
/// TODO: Implement `AuditLogger` for `InMemoryAuditLogger`.
/// Store events in an `Arc<RwLock<Vec<AuditEvent>>>`.
#[derive(Debug, Clone)]
pub struct InMemoryAuditLogger {
    pub events: Arc<RwLock<Vec<AuditEvent>>>,
}

impl InMemoryAuditLogger {
    pub fn new() -> Self {
        Self {
            events: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

impl Default for InMemoryAuditLogger {
    fn default() -> Self {
        Self::new()
    }
}

/// Record an audit event into the shared state's audit log.
///
/// TODO: Push the event onto `state.audit_log`.
pub async fn record_audit_event(
    _audit_log: &Arc<RwLock<Vec<AuditEvent>>>,
    _event: AuditEvent,
) {
    todo!("Record the audit event")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_record_and_retrieve_events() {
        let log = Arc::new(RwLock::new(Vec::<AuditEvent>::new()));
        let event = AuditEvent {
            who: "user-1".to_string(),
            action: AuditAction::Create,
            resource_type: "note".to_string(),
            resource_id: "abc-123".to_string(),
            when: Utc::now(),
            ip_address: "127.0.0.1".to_string(),
            details: None,
        };

        record_audit_event(&log, event).await;

        let events = log.read().await;
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].who, "user-1");
        assert_eq!(events[0].action, AuditAction::Create);
    }
}
