# Project: Background Job Processor

**Phase 4, Project 4 — Building a reliable background job processing system backed by PostgreSQL.**

In this project you will build a production-quality background job processor that
uses PostgreSQL as its backing store. Jobs are enqueued by clients and processed
by one or more workers. The system supports priorities, delayed scheduling,
automatic retries with exponential backoff, dead-lettering, and recurring
(cron-based) jobs.

## Prerequisites

- Rust (edition 2021)
- A running PostgreSQL instance
- `DATABASE_URL` environment variable set to a valid connection string
  (e.g. `postgres://user:pass@localhost/job_processor`)

## Getting Started

```bash
# Set up the database
export DATABASE_URL=postgres://user:pass@localhost/job_processor

# Run migrations (handled automatically by the binaries and tests)
cargo run --bin worker   # starts a worker
cargo run --bin enqueue -- email '{"to":"user@example.com"}'   # enqueue a test job
```

---

## Curriculum

### Part 1: Job Schema

Design the PostgreSQL table that stores jobs. Each job row must track:

| Column            | Type          | Purpose                                      |
|-------------------|---------------|----------------------------------------------|
| `id`              | `UUID`        | Primary key                                  |
| `queue`           | `VARCHAR(100)`| Logical queue name (e.g. `"email"`)          |
| `payload`         | `JSONB`       | Arbitrary data consumed by the handler       |
| `status`          | `VARCHAR(20)` | One of `pending`, `running`, `completed`, `failed`, `dead` |
| `priority`        | `INTEGER`     | Higher values are dequeued first             |
| `attempts`        | `INTEGER`     | How many times this job has been attempted   |
| `max_attempts`    | `INTEGER`     | Upper bound before dead-lettering            |
| `scheduled_at`    | `TIMESTAMPTZ` | Earliest time the job is eligible for pickup |
| `started_at`      | `TIMESTAMPTZ` | When processing began                        |
| `completed_at`    | `TIMESTAMPTZ` | When the job finished successfully           |
| `failed_at`       | `TIMESTAMPTZ` | When the last failure occurred               |
| `locked_by`       | `VARCHAR(255)`| Worker id holding the lock                   |
| `locked_at`       | `TIMESTAMPTZ` | When the lock was acquired                   |
| `error`           | `TEXT`        | Last error message                           |
| `cron_expression` | `VARCHAR(100)`| Optional cron string for recurring jobs      |
| `created_at`      | `TIMESTAMPTZ` | Row creation timestamp                       |
| `updated_at`      | `TIMESTAMPTZ` | Last modification timestamp                  |

Create a partial index on `(priority DESC, created_at ASC)` filtered to
`status = 'pending' AND scheduled_at <= NOW()` so the dequeue query is fast.

See `migrations/001_jobs.sql` for the reference schema.

**Tasks:**
1. Read and understand the migration file.
2. Think about why `JSONB` is chosen over `JSON`.
3. Explain the purpose of the partial index.

---

### Part 2: Job Client

Implement `JobClient` in `src/client.rs`. The client provides a clean API for
producers to create jobs:

- `enqueue(queue, payload)` -- creates a job for immediate processing.
- `enqueue_delayed(queue, payload, delay)` -- sets `scheduled_at` in the future.
- `enqueue_with_priority(queue, payload, priority)` -- sets a non-default priority.
- `schedule_recurring(queue, payload, cron)` -- stores a cron expression so the
  worker can reschedule the job after each completion.
- `get_job(id)` -- fetches a job by id.
- `cancel_job(id)` -- marks a pending job so it will not be processed.

**Tasks:**
1. Implement `JobClient::new` to store the pool.
2. Implement each method using `sqlx::query` / `sqlx::query_as`.
3. Validate cron expressions in `schedule_recurring` using the `cron` crate.
4. Decide how `cancel_job` should change the job status.

---

### Part 3: Worker Loop

Implement the polling loop in `Worker::run` (`src/worker.rs`):

```
loop {
    if shutdown signaled: break
    job = dequeue()
    if job is Some:
        process_job(job)
    else:
        sleep briefly to avoid busy-waiting
}
```

The `dequeue` method must use:

```sql
SELECT * FROM jobs
WHERE status = 'pending'
  AND scheduled_at <= NOW()
ORDER BY priority DESC, created_at ASC
LIMIT 1
FOR UPDATE SKIP LOCKED
```

This ensures that concurrent workers never pick the same job.

**Tasks:**
1. Implement `dequeue` with the locking query.
2. After selecting, update the job to `status = 'running'`, set `locked_by` and
   `locked_at`.
3. Implement the polling loop with a configurable sleep interval.
4. Respect the shutdown signal between iterations.

---

### Part 4: Job Handlers

The `JobHandler` trait (`src/handler.rs`) lets users define custom processing
logic per queue:

```rust
#[async_trait]
pub trait JobHandler: Send + Sync {
    async fn handle(&self, payload: serde_json::Value)
        -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    fn queue(&self) -> &str;
}
```

Workers register handlers via `register_handler`. When a job is dequeued the
worker looks up the handler by `job.queue` and calls `handler.handle(job.payload)`.

**Tasks:**
1. Store handlers in a `HashMap<String, Box<dyn JobHandler>>`.
2. In `process_job`, look up the handler and call it.
3. If no handler is registered for a queue, treat it as an error.

---

### Part 5: Retry and Backoff

When a handler returns an error:

1. Increment `attempts`.
2. If `attempts < max_attempts`, set `status = 'pending'` and push
   `scheduled_at` into the future using exponential backoff:
   `scheduled_at = NOW() + base * 2^(attempts - 1)` (e.g. base = 5 seconds).
3. Store the error message.

**Tasks:**
1. Implement `schedule_retry`.
2. Implement `fail_job` to decide between retry and dead-letter.
3. Write tests that verify the backoff delay grows with each attempt.

---

### Part 6: Dead Letter

After a job exhausts `max_attempts`:

1. Set `status = 'dead'`.
2. Record the final error in the `error` column.
3. Set `failed_at` to `NOW()`.

Dead jobs remain in the table for inspection and can be manually retried later.

**Tasks:**
1. Implement `dead_letter`.
2. Add a test that sets `max_attempts = 1` and verifies the job moves to `dead`
   after a single failure.

---

### Part 7: Priority Queue

Jobs with higher `priority` values are dequeued before lower ones. Ties are
broken by `created_at ASC` (FIFO within a priority level).

The partial index `idx_jobs_dequeue` makes this ordering efficient.

**Tasks:**
1. Verify that `dequeue` uses `ORDER BY priority DESC, created_at ASC`.
2. Write a test that enqueues three jobs at different priorities and checks they
   are processed in the correct order.

---

### Part 8: Graceful Shutdown

When the worker receives a shutdown signal (e.g. Ctrl-C via `tokio::signal`):

1. Stop polling for new jobs.
2. Finish processing the current job (do not abort mid-handler).
3. Release any locks.

Use a `tokio::sync::watch` channel to propagate the shutdown signal.

**Tasks:**
1. Check `shutdown.has_changed()` at the top of each loop iteration.
2. After the loop exits, ensure no locks are left behind (update any jobs still
   marked as `locked_by` this worker back to `pending`).
3. Write a test that sends a shutdown signal while a slow handler is running and
   verify the job still completes.

---

### Part 9: Recurring Jobs

A recurring job has a `cron_expression` (e.g. `"0 * * * * *"` for every minute).
After a recurring job completes successfully:

1. Parse the cron expression using the `cron` crate.
2. Compute the next fire time.
3. Insert a new `pending` job with the same queue, payload, and cron expression,
   with `scheduled_at` set to the next fire time.

**Tasks:**
1. In `complete_job`, check if `cron_expression` is set.
2. If so, compute the next occurrence and enqueue a new job.
3. Write a test that verifies a new pending job is created after a cron job
   completes.

---

## Running Tests

Tests require a running PostgreSQL database:

```bash
export DATABASE_URL=postgres://user:pass@localhost/job_processor_test
cargo test
```

All tests clean up after themselves by deleting rows from the `jobs` table.

## Project Structure

```
advanced-4/
  Cargo.toml
  README.md
  migrations/
    001_jobs.sql
  src/
    lib.rs          -- module declarations and re-exports
    models.rs       -- Job, JobStatus, CreateJob, JobResult
    client.rs       -- JobClient (enqueue, get, cancel)
    worker.rs       -- Worker (dequeue, process, retry, dead-letter)
    handler.rs      -- JobHandler trait
    error.rs        -- JobError, Result
    bin/
      worker.rs     -- worker binary
      enqueue.rs    -- CLI enqueue tool
  tests/
    job_tests.rs    -- integration tests
```
