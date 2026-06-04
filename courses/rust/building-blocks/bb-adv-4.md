# Building Block Adv-4: Background Job Processing

**Prerequisites**: [Project: WebSocket Real-Time Service](../projects/advanced-3/README.md).

Before starting [Project: Background Job Processor](../projects/advanced-4/README.md),
complete the readings and exercises below.

## What to read

- [PostgreSQL Advisory Locks and SKIP LOCKED](https://www.2ndquadrant.com/en/blog/what-is-select-skip-locked-for-in-postgresql-9-5/).
  How to use PostgreSQL as a job queue without race conditions. `SKIP LOCKED`
  ensures only one worker processes each job.

- [Background Job Architectures Comparison](https://brandur.org/job-drain).
  Comparison of approaches: database-backed, Redis-backed, dedicated queue (RabbitMQ).
  Database-backed is simplest and sufficient for most use cases.

- [Cron Expression Format](https://crontab.guru/).
  The standard format for recurring schedules. Learn the 5-field syntax:
  minute hour day-of-month month day-of-week.

- [The Outbox Pattern](https://microservices.io/patterns/data/transactional-outbox.html).
  Reliably publishing events alongside database transactions.

## Key concepts

### Database-Backed Job Queue

```sql
CREATE TABLE jobs (
    id UUID PRIMARY KEY,
    queue VARCHAR(100) NOT NULL,
    payload JSONB NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    priority INTEGER NOT NULL DEFAULT 0,
    attempts INTEGER NOT NULL DEFAULT 0,
    max_attempts INTEGER NOT NULL DEFAULT 3,
    scheduled_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    locked_at TIMESTAMPTZ,
    locked_by VARCHAR(100),
    completed_at TIMESTAMPTZ,
    failed_at TIMESTAMPTZ,
    error TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Dequeue: atomically claim the next job
SELECT * FROM jobs
WHERE status = 'pending'
  AND scheduled_at <= NOW()
ORDER BY priority DESC, created_at ASC
LIMIT 1
FOR UPDATE SKIP LOCKED;
```

### Job State Machine

```
pending → running → completed
                  → failed → pending (retry)
                           → dead (max retries exceeded)
```

### Worker Lifecycle

```
1. Poll for available jobs
2. Claim a job (UPDATE with SKIP LOCKED)
3. Execute the job handler
4. On success: mark completed
5. On failure: increment attempts, schedule retry with backoff
6. If max attempts exceeded: move to dead letter
7. On shutdown signal: finish current job, stop polling
```

## Exercises

**Exercise 1**: Implement a simple job state machine as a Rust enum with valid
transitions.

**Exercise 2**: Write the SQL for a job queue using SKIP LOCKED. Test that
two concurrent workers don't claim the same job.

**Exercise 3**: Implement a worker loop with graceful shutdown using
`tokio::select!` on the shutdown signal and the next job.

## You're ready when...

- [ ] You understand SKIP LOCKED and why it prevents double-processing
- [ ] You can design a job state machine with retries
- [ ] You know the difference between scheduled, delayed, and recurring jobs
- [ ] You can implement graceful worker shutdown

Next: [Project: Background Job Processor](../projects/advanced-4/README.md)
