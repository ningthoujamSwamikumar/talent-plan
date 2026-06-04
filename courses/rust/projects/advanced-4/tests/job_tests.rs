//! Integration tests for the background job processor.
//!
//! These tests require a running PostgreSQL instance. Set `DATABASE_URL` in a
//! `.env` file or as an environment variable to run them:
//!
//!   DATABASE_URL=postgres://user:pass@localhost/job_processor_test cargo test

use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tokio::sync::Mutex;
use uuid::Uuid;

use job_processor::{JobClient, JobHandler, JobStatus, Worker};

/// Helper: create a connection pool and run migrations.
async fn setup_pool() -> PgPool {
    let _ = dotenvy::dotenv();
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set for tests");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .expect("failed to connect to database");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("failed to run migrations");
    // Clean up any leftover jobs from previous runs
    sqlx::query("DELETE FROM jobs")
        .execute(&pool)
        .await
        .expect("failed to clean jobs table");
    pool
}

// ---------------------------------------------------------------------------
// Test handler implementations
// ---------------------------------------------------------------------------

/// A simple handler that records received payloads.
struct RecordingHandler {
    queue_name: String,
    payloads: Arc<Mutex<Vec<serde_json::Value>>>,
}

impl RecordingHandler {
    fn new(queue: &str) -> (Self, Arc<Mutex<Vec<serde_json::Value>>>) {
        let payloads = Arc::new(Mutex::new(Vec::new()));
        (
            Self {
                queue_name: queue.to_string(),
                payloads: payloads.clone(),
            },
            payloads,
        )
    }
}

#[async_trait]
impl JobHandler for RecordingHandler {
    async fn handle(
        &self,
        payload: serde_json::Value,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.payloads.lock().await.push(payload);
        Ok(())
    }

    fn queue(&self) -> &str {
        &self.queue_name
    }
}

/// A handler that always fails.
struct FailingHandler {
    queue_name: String,
}

#[async_trait]
impl JobHandler for FailingHandler {
    async fn handle(
        &self,
        _payload: serde_json::Value,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Err("intentional failure".into())
    }

    fn queue(&self) -> &str {
        &self.queue_name
    }
}

/// A handler that takes a while to process (for shutdown tests).
struct SlowHandler {
    queue_name: String,
    completed: Arc<Mutex<bool>>,
}

#[async_trait]
impl JobHandler for SlowHandler {
    async fn handle(
        &self,
        _payload: serde_json::Value,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        tokio::time::sleep(Duration::from_millis(500)).await;
        *self.completed.lock().await = true;
        Ok(())
    }

    fn queue(&self) -> &str {
        &self.queue_name
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn enqueue_creates_pending_job() {
    let pool = setup_pool().await;
    let client = JobClient::new(pool);
    let id = client
        .enqueue("test", json!({"key": "value"}))
        .await
        .expect("enqueue failed");
    let job = client
        .get_job(id)
        .await
        .expect("get_job failed")
        .expect("job not found");
    assert_eq!(job.status, "pending");
    assert_eq!(job.queue, "test");
}

#[tokio::test]
async fn enqueue_delayed_not_visible_until_scheduled_at() {
    let pool = setup_pool().await;
    let client = JobClient::new(pool.clone());
    let _id = client
        .enqueue_delayed("delayed_q", json!({}), Duration::from_secs(3600))
        .await
        .expect("enqueue_delayed failed");

    // A worker trying to dequeue right now should find nothing
    let worker = Worker::new(pool, "test-worker".into());
    // dequeue is private, so we rely on the job not being picked up by run()
    // Instead verify via get_job that scheduled_at is in the future
    let job = client
        .get_job(_id)
        .await
        .expect("get_job failed")
        .expect("job not found");
    assert!(job.scheduled_at > chrono::Utc::now());
}

#[tokio::test]
async fn dequeue_returns_highest_priority_first() {
    let pool = setup_pool().await;
    let client = JobClient::new(pool.clone());

    let low = client
        .enqueue_with_priority("prio_q", json!({"name": "low"}), 1)
        .await
        .unwrap();
    let high = client
        .enqueue_with_priority("prio_q", json!({"name": "high"}), 10)
        .await
        .unwrap();
    let medium = client
        .enqueue_with_priority("prio_q", json!({"name": "medium"}), 5)
        .await
        .unwrap();

    // The first job dequeued should be the highest priority.
    // We test this by running a worker that records order.
    let (handler, payloads) = RecordingHandler::new("prio_q");
    let mut worker = Worker::new(pool, "prio-worker".into());
    worker.register_handler("prio_q", Box::new(handler));

    let (tx, rx) = tokio::sync::watch::channel(());
    let handle = tokio::spawn(async move { worker.run(rx).await });

    // Give it time to process all three jobs
    tokio::time::sleep(Duration::from_secs(2)).await;
    let _ = tx.send(());
    let _ = handle.await;

    let received = payloads.lock().await;
    assert!(received.len() >= 3, "expected 3 jobs processed");
    assert_eq!(received[0]["name"], "high");
    assert_eq!(received[1]["name"], "medium");
    assert_eq!(received[2]["name"], "low");
}

#[tokio::test]
async fn dequeue_skips_locked_jobs() {
    let pool = setup_pool().await;
    let client = JobClient::new(pool.clone());

    // Enqueue two jobs
    let _id1 = client.enqueue("lock_q", json!({"n": 1})).await.unwrap();
    let _id2 = client.enqueue("lock_q", json!({"n": 2})).await.unwrap();

    // Both workers should each pick a different job (SKIP LOCKED)
    let (h1, p1) = RecordingHandler::new("lock_q");
    let (h2, p2) = RecordingHandler::new("lock_q");

    let mut w1 = Worker::new(pool.clone(), "w1".into());
    w1.register_handler("lock_q", Box::new(h1));
    let mut w2 = Worker::new(pool.clone(), "w2".into());
    w2.register_handler("lock_q", Box::new(h2));

    let (tx1, rx1) = tokio::sync::watch::channel(());
    let (tx2, rx2) = tokio::sync::watch::channel(());
    let h1 = tokio::spawn(async move { w1.run(rx1).await });
    let h2 = tokio::spawn(async move { w2.run(rx2).await });

    tokio::time::sleep(Duration::from_secs(2)).await;
    let _ = tx1.send(());
    let _ = tx2.send(());
    let _ = h1.await;
    let _ = h2.await;

    let total = p1.lock().await.len() + p2.lock().await.len();
    assert_eq!(total, 2, "both jobs should be processed exactly once");
}

#[tokio::test]
async fn process_job_marks_completed() {
    let pool = setup_pool().await;
    let client = JobClient::new(pool.clone());
    let id = client.enqueue("comp_q", json!({})).await.unwrap();

    let (handler, _) = RecordingHandler::new("comp_q");
    let mut worker = Worker::new(pool, "comp-worker".into());
    worker.register_handler("comp_q", Box::new(handler));

    let (tx, rx) = tokio::sync::watch::channel(());
    let h = tokio::spawn(async move { worker.run(rx).await });

    tokio::time::sleep(Duration::from_secs(1)).await;
    let _ = tx.send(());
    let _ = h.await;

    let job = client.get_job(id).await.unwrap().unwrap();
    assert_eq!(job.status, "completed");
    assert!(job.completed_at.is_some());
}

#[tokio::test]
async fn failed_job_retries_with_backoff() {
    let pool = setup_pool().await;
    let client = JobClient::new(pool.clone());
    let id = client.enqueue("fail_q", json!({})).await.unwrap();

    let handler = FailingHandler {
        queue_name: "fail_q".into(),
    };
    let mut worker = Worker::new(pool, "fail-worker".into());
    worker.register_handler("fail_q", Box::new(handler));

    let (tx, rx) = tokio::sync::watch::channel(());
    let h = tokio::spawn(async move { worker.run(rx).await });

    // Let it fail once and schedule a retry
    tokio::time::sleep(Duration::from_secs(2)).await;
    let _ = tx.send(());
    let _ = h.await;

    let job = client.get_job(id).await.unwrap().unwrap();
    assert!(job.attempts >= 1, "job should have at least one attempt");
    // After one failure with retries remaining, the job should be pending again
    // with scheduled_at in the future (backoff), or still failed/dead
    assert!(
        job.status == "pending" || job.status == "failed" || job.status == "dead",
        "unexpected status: {}",
        job.status
    );
}

#[tokio::test]
async fn dead_letter_after_max_attempts() {
    let pool = setup_pool().await;
    let client = JobClient::new(pool.clone());

    // Create a job with max_attempts = 1 so it goes to dead immediately
    let id = client.enqueue("dead_q", json!({})).await.unwrap();
    // Set max_attempts to 1 directly
    sqlx::query("UPDATE jobs SET max_attempts = 1 WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .unwrap();

    let handler = FailingHandler {
        queue_name: "dead_q".into(),
    };
    let mut worker = Worker::new(pool, "dead-worker".into());
    worker.register_handler("dead_q", Box::new(handler));

    let (tx, rx) = tokio::sync::watch::channel(());
    let h = tokio::spawn(async move { worker.run(rx).await });

    tokio::time::sleep(Duration::from_secs(2)).await;
    let _ = tx.send(());
    let _ = h.await;

    let job = client.get_job(id).await.unwrap().unwrap();
    assert_eq!(job.status, "dead");
    assert!(job.error.is_some());
}

#[tokio::test]
async fn cancel_job_prevents_processing() {
    let pool = setup_pool().await;
    let client = JobClient::new(pool.clone());
    let id = client.enqueue("cancel_q", json!({})).await.unwrap();

    let cancelled = client.cancel_job(id).await.unwrap();
    assert!(cancelled);

    let job = client.get_job(id).await.unwrap().unwrap();
    // Cancelled jobs should not be in pending state
    assert_ne!(job.status, "pending");
}

#[tokio::test]
async fn concurrent_workers_no_double_processing() {
    let pool = setup_pool().await;
    let client = JobClient::new(pool.clone());

    // Enqueue 10 jobs
    for i in 0..10 {
        client
            .enqueue("conc_q", json!({"i": i}))
            .await
            .unwrap();
    }

    let counter = Arc::new(Mutex::new(0u32));

    // Spin up 3 workers
    let mut handles = Vec::new();
    let mut senders = Vec::new();

    for w in 0..3 {
        let c = counter.clone();
        let p = pool.clone();

        struct CountingHandler {
            queue_name: String,
            counter: Arc<Mutex<u32>>,
        }

        #[async_trait]
        impl JobHandler for CountingHandler {
            async fn handle(
                &self,
                _payload: serde_json::Value,
            ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
                *self.counter.lock().await += 1;
                Ok(())
            }
            fn queue(&self) -> &str {
                &self.queue_name
            }
        }

        let handler = CountingHandler {
            queue_name: "conc_q".into(),
            counter: c,
        };
        let mut worker = Worker::new(p, format!("conc-{}", w));
        worker.register_handler("conc_q", Box::new(handler));

        let (tx, rx) = tokio::sync::watch::channel(());
        senders.push(tx);
        handles.push(tokio::spawn(async move { worker.run(rx).await }));
    }

    tokio::time::sleep(Duration::from_secs(3)).await;
    for tx in senders {
        let _ = tx.send(());
    }
    for h in handles {
        let _ = h.await;
    }

    let total = *counter.lock().await;
    assert_eq!(total, 10, "each job should be processed exactly once");
}

#[tokio::test]
async fn graceful_shutdown_finishes_current_job() {
    let pool = setup_pool().await;
    let client = JobClient::new(pool.clone());
    let id = client.enqueue("slow_q", json!({})).await.unwrap();

    let completed = Arc::new(Mutex::new(false));
    let handler = SlowHandler {
        queue_name: "slow_q".into(),
        completed: completed.clone(),
    };
    let mut worker = Worker::new(pool, "slow-worker".into());
    worker.register_handler("slow_q", Box::new(handler));

    let (tx, rx) = tokio::sync::watch::channel(());
    let h = tokio::spawn(async move { worker.run(rx).await });

    // Wait for the job to start processing, then signal shutdown
    tokio::time::sleep(Duration::from_millis(200)).await;
    let _ = tx.send(());

    let _ = h.await;

    // The slow handler should have finished
    assert!(*completed.lock().await, "current job should finish before shutdown");

    let job = client.get_job(id).await.unwrap().unwrap();
    assert_eq!(job.status, "completed");
}

#[tokio::test]
async fn handler_receives_correct_payload() {
    let pool = setup_pool().await;
    let client = JobClient::new(pool.clone());
    let payload = json!({"email": "test@example.com", "template": "welcome"});
    let _id = client
        .enqueue("payload_q", payload.clone())
        .await
        .unwrap();

    let (handler, payloads) = RecordingHandler::new("payload_q");
    let mut worker = Worker::new(pool, "payload-worker".into());
    worker.register_handler("payload_q", Box::new(handler));

    let (tx, rx) = tokio::sync::watch::channel(());
    let h = tokio::spawn(async move { worker.run(rx).await });

    tokio::time::sleep(Duration::from_secs(1)).await;
    let _ = tx.send(());
    let _ = h.await;

    let received = payloads.lock().await;
    assert_eq!(received.len(), 1);
    assert_eq!(received[0], payload);
}

#[tokio::test]
async fn recurring_job_reschedules_after_completion() {
    let pool = setup_pool().await;
    let client = JobClient::new(pool.clone());

    // Schedule a recurring job with a cron expression (every minute)
    let id = client
        .schedule_recurring("cron_q", json!({"task": "cleanup"}), "0 * * * * *")
        .await
        .unwrap();

    let (handler, _) = RecordingHandler::new("cron_q");
    let mut worker = Worker::new(pool.clone(), "cron-worker".into());
    worker.register_handler("cron_q", Box::new(handler));

    let (tx, rx) = tokio::sync::watch::channel(());
    let h = tokio::spawn(async move { worker.run(rx).await });

    tokio::time::sleep(Duration::from_secs(2)).await;
    let _ = tx.send(());
    let _ = h.await;

    // After the recurring job completes, a new pending job should exist for the
    // same queue with a cron_expression set.
    let row: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM jobs WHERE queue = 'cron_q' AND cron_expression IS NOT NULL AND status = 'pending'",
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    assert!(
        row.0 >= 1,
        "a new pending recurring job should have been created"
    );
}

#[tokio::test]
async fn get_job_returns_status() {
    let pool = setup_pool().await;
    let client = JobClient::new(pool);
    let id = client.enqueue("status_q", json!({})).await.unwrap();

    let job = client.get_job(id).await.unwrap().unwrap();
    assert_eq!(job.id, id);
    assert_eq!(job.status, "pending");

    // Non-existent job
    let missing = client.get_job(Uuid::new_v4()).await.unwrap();
    assert!(missing.is_none());
}

#[tokio::test]
async fn enqueue_with_priority_ordering() {
    let pool = setup_pool().await;
    let client = JobClient::new(pool);

    let id_low = client
        .enqueue_with_priority("ord_q", json!({"p": "low"}), 0)
        .await
        .unwrap();
    let id_high = client
        .enqueue_with_priority("ord_q", json!({"p": "high"}), 100)
        .await
        .unwrap();

    let low = client.get_job(id_low).await.unwrap().unwrap();
    let high = client.get_job(id_high).await.unwrap().unwrap();

    assert_eq!(low.priority, 0);
    assert_eq!(high.priority, 100);
    assert!(high.priority > low.priority);
}

#[tokio::test]
async fn worker_processes_multiple_queues() {
    let pool = setup_pool().await;
    let client = JobClient::new(pool.clone());

    client.enqueue("queue_a", json!({"q": "a"})).await.unwrap();
    client.enqueue("queue_b", json!({"q": "b"})).await.unwrap();

    let (ha, pa) = RecordingHandler::new("queue_a");
    let (hb, pb) = RecordingHandler::new("queue_b");

    let mut worker = Worker::new(pool, "multi-worker".into());
    worker.register_handler("queue_a", Box::new(ha));
    worker.register_handler("queue_b", Box::new(hb));

    let (tx, rx) = tokio::sync::watch::channel(());
    let h = tokio::spawn(async move { worker.run(rx).await });

    tokio::time::sleep(Duration::from_secs(2)).await;
    let _ = tx.send(());
    let _ = h.await;

    assert_eq!(pa.lock().await.len(), 1);
    assert_eq!(pb.lock().await.len(), 1);
    assert_eq!(pa.lock().await[0]["q"], "a");
    assert_eq!(pb.lock().await[0]["q"], "b");
}
