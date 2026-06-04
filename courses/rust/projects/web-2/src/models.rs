use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

// ---------------------------------------------------------------------------
// Domain enums
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "varchar")]
#[sqlx(rename_all = "snake_case")]
pub enum TaskStatus {
    #[serde(rename = "todo")]
    #[sqlx(rename = "todo")]
    Todo,
    #[serde(rename = "in_progress")]
    #[sqlx(rename = "in_progress")]
    InProgress,
    #[serde(rename = "done")]
    #[sqlx(rename = "done")]
    Done,
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskStatus::Todo => write!(f, "todo"),
            TaskStatus::InProgress => write!(f, "in_progress"),
            TaskStatus::Done => write!(f, "done"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "varchar")]
#[sqlx(rename_all = "snake_case")]
pub enum Priority {
    #[serde(rename = "low")]
    #[sqlx(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    #[sqlx(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    #[sqlx(rename = "high")]
    High,
    #[serde(rename = "critical")]
    #[sqlx(rename = "critical")]
    Critical,
}

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Priority::Low => write!(f, "low"),
            Priority::Medium => write!(f, "medium"),
            Priority::High => write!(f, "high"),
            Priority::Critical => write!(f, "critical"),
        }
    }
}

// ---------------------------------------------------------------------------
// Database row types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Task {
    pub id: Uuid,
    pub project_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub priority: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// A task joined with its parent project name — useful for list views.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TaskWithProject {
    pub id: Uuid,
    pub project_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub priority: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub project_name: String,
}

/// Aggregation row returned by count-by-status queries.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct StatusCount {
    pub status: String,
    pub count: i64,
}

// ---------------------------------------------------------------------------
// Request / input types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize, Validate)]
pub struct CreateProject {
    #[validate(length(min = 1, max = 255, message = "name must be 1-255 characters"))]
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProject {
    #[validate(length(min = 1, max = 255, message = "name must be 1-255 characters"))]
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateTask {
    pub project_id: Uuid,
    #[validate(length(min = 1, max = 500, message = "title must be 1-500 characters"))]
    pub title: String,
    pub description: Option<String>,
    pub status: Option<String>,
    pub priority: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateTask {
    #[validate(length(min = 1, max = 500, message = "title must be 1-500 characters"))]
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub priority: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

impl PaginationParams {
    pub fn offset(&self) -> i64 {
        self.offset.unwrap_or(0)
    }
    pub fn limit(&self) -> i64 {
        self.limit.unwrap_or(20).min(100)
    }
}

#[derive(Debug, Deserialize)]
pub struct TaskFilter {
    pub status: Option<String>,
    pub priority: Option<String>,
    pub search: Option<String>,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

impl TaskFilter {
    pub fn offset(&self) -> i64 {
        self.offset.unwrap_or(0)
    }
    pub fn limit(&self) -> i64 {
        self.limit.unwrap_or(20).min(100)
    }
}

#[derive(Debug, Deserialize)]
pub struct MoveTasksRequest {
    pub from_project_id: Uuid,
    pub to_project_id: Uuid,
}
