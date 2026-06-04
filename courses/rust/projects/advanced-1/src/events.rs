use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Envelope that wraps every event with metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEnvelope {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub event: Event,
}

/// Domain events produced by the task management system.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Event {
    TaskCreated {
        task_id: Uuid,
        title: String,
        project_id: Uuid,
    },
    TaskUpdated {
        task_id: Uuid,
        field: String,
        old_value: String,
        new_value: String,
    },
    TaskDeleted {
        task_id: Uuid,
    },
}

impl Event {
    /// Return the task ID regardless of variant.
    pub fn task_id(&self) -> Uuid {
        match self {
            Event::TaskCreated { task_id, .. } => *task_id,
            Event::TaskUpdated { task_id, .. } => *task_id,
            Event::TaskDeleted { task_id } => *task_id,
        }
    }

    /// Wrap this event in an envelope with a fresh ID and the current timestamp.
    pub fn into_envelope(self) -> EventEnvelope {
        EventEnvelope {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            event: self,
        }
    }
}
