//! # Background Job Processor
//!
//! A reliable background job processing system backed by PostgreSQL.
//!
//! This crate provides:
//! - A [`JobClient`] for enqueuing immediate, delayed, prioritized, and recurring jobs
//! - A [`Worker`] that polls for jobs and dispatches them to registered [`JobHandler`] implementations
//! - Automatic retries with exponential backoff and dead-letter handling
//! - Priority-based ordering and `SELECT ... FOR UPDATE SKIP LOCKED` for safe concurrent dequeuing

pub mod client;
pub mod error;
pub mod handler;
pub mod models;
pub mod worker;

pub use client::JobClient;
pub use error::{JobError, Result};
pub use handler::JobHandler;
pub use models::{CreateJob, Job, JobResult, JobStatus};
pub use worker::Worker;
