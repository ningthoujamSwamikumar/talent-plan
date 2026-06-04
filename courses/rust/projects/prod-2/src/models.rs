use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;
use uuid::Uuid;

/// The status of a task in its lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
}

impl TaskStatus {
    /// Returns whether transitioning from `self` to `target` is a valid
    /// status transition.
    ///
    /// Valid transitions:
    ///   Todo -> InProgress
    ///   InProgress -> Done
    ///
    /// All other transitions are invalid (including self-transitions).
    pub fn can_transition_to(&self, target: &TaskStatus) -> bool {
        // BUG: This also allows Done -> Todo, which should be invalid.
        matches!(
            (self, target),
            (TaskStatus::Todo, TaskStatus::InProgress)
                | (TaskStatus::InProgress, TaskStatus::Done)
                | (TaskStatus::Done, TaskStatus::Todo) // <-- intentional bug
        )
    }
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskStatus::Todo => write!(f, "Todo"),
            TaskStatus::InProgress => write!(f, "In Progress"),
            TaskStatus::Done => write!(f, "Done"),
        }
    }
}

/// Task priority levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

impl Priority {
    /// Returns the numeric weight of this priority for ordering purposes.
    pub fn weight(&self) -> u8 {
        match self {
            Priority::Low => 1,
            Priority::Medium => 2,
            Priority::High => 3,
            Priority::Critical => 4,
        }
    }
}

impl PartialOrd for Priority {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Priority {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight().cmp(&other.weight())
    }
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Priority::Low => write!(f, "Low"),
            Priority::Medium => write!(f, "Medium"),
            Priority::High => write!(f, "High"),
            Priority::Critical => write!(f, "Critical"),
        }
    }
}

/// A task in the task management system.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub priority: Priority,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Request to create a new task.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTask {
    pub title: String,
    pub description: Option<String>,
    pub priority: Priority,
}

/// Request to update an existing task.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTask {
    pub title: Option<String>,
    pub description: Option<Option<String>>,
    pub priority: Option<Priority>,
}

/// Parameters for listing tasks.
#[derive(Debug, Clone)]
pub struct ListParams {
    pub status: Option<TaskStatus>,
    pub priority: Option<Priority>,
    pub offset: usize,
    pub limit: usize,
}

impl Default for ListParams {
    fn default() -> Self {
        Self {
            status: None,
            priority: None,
            offset: 0,
            limit: 20,
        }
    }
}
