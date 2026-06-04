pub mod cache;
pub mod optimized;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A task in the task management service.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub completed: bool,
}

impl Task {
    /// Create a new task with the given title, description, and tags.
    pub fn new(title: String, description: String, tags: Vec<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            description,
            tags,
            created_at: Utc::now(),
            completed: false,
        }
    }

    /// Create a sample task for benchmarking.
    pub fn sample() -> Self {
        Self {
            id: Uuid::new_v4(),
            title: "Sample Task".to_string(),
            description: "A sample task used for benchmarking and testing.".to_string(),
            tags: vec![
                "benchmark".to_string(),
                "sample".to_string(),
                "performance".to_string(),
            ],
            created_at: Utc::now(),
            completed: false,
        }
    }
}
