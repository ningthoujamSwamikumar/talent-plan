use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Possible states a job can be in.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum JobStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Dead,
}

impl JobStatus {
    /// Convert to the string stored in the database.
    pub fn as_str(&self) -> &'static str {
        match self {
            JobStatus::Pending => "pending",
            JobStatus::Running => "running",
            JobStatus::Completed => "completed",
            JobStatus::Failed => "failed",
            JobStatus::Dead => "dead",
        }
    }

    /// Parse from the database string representation.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "pending" => Some(JobStatus::Pending),
            "running" => Some(JobStatus::Running),
            "completed" => Some(JobStatus::Completed),
            "failed" => Some(JobStatus::Failed),
            "dead" => Some(JobStatus::Dead),
            _ => None,
        }
    }
}

/// A job record as stored in the database.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub id: Uuid,
    pub queue: String,
    pub payload: serde_json::Value,
    pub status: String,
    pub priority: i32,
    pub attempts: i32,
    pub max_attempts: i32,
    pub scheduled_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub failed_at: Option<DateTime<Utc>>,
    pub locked_by: Option<String>,
    pub locked_at: Option<DateTime<Utc>>,
    pub error: Option<String>,
    pub cron_expression: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Job {
    /// Return the parsed status enum.
    pub fn job_status(&self) -> Option<JobStatus> {
        JobStatus::from_str(&self.status)
    }
}

/// Parameters for creating a new job.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateJob {
    pub queue: String,
    pub payload: serde_json::Value,
    pub priority: Option<i32>,
    pub max_attempts: Option<i32>,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub cron_expression: Option<String>,
}

/// The outcome of processing a job.
#[derive(Debug, Clone)]
pub enum JobResult {
    Success,
    Failure(String),
}
