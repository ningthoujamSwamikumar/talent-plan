# Project: Advanced Async Deep Dive

**Phase 1 — Systems Programming**

You've been using `async/await` as a black box. In this project you open that
box. You will implement a custom `Future`, build a minimal async executor,
diagnose real async bugs with `tokio-console`, and build a backpressure-aware
pipeline. These skills are required when production async code misbehaves in
ways that surface-level knowledge can't explain.

## Prerequisites

- Completion of project-5 (Async KV Store)
- Building block bb-async-adv (Advanced Async)

## Deliverables

- [ ] A hand-written `Future` implementation (not using `async fn`)
- [ ] A working single-threaded async executor
- [ ] A diagnosis and fix for a blocking-in-async bug
- [ ] A backpressure-aware stream processing pipeline
- [ ] A concurrent task limiter using semaphores
- [ ] All tests passing

---

## Part 1: Implement Future by Hand

Implement a `TimerFuture` that completes after a specified duration.

**Requirements:**
- Do NOT use `tokio::time::sleep` or any async runtime timer
- Use `std::thread::spawn` to start a background thread that sleeps and then
  calls `waker.wake()`
- Implement `Future` trait manually with `fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()>`

**Skeleton:**
```rust
pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

struct SharedState {
    completed: bool,
    waker: Option<Waker>,
}

impl Future for TimerFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        todo!()
    }
}

impl TimerFuture {
    pub fn new(duration: Duration) -> Self {
        todo!()
    }
}
```

**Tests to pass:**
- `timer_future_completes` — future resolves after the specified duration
- `timer_future_wake` — waker is called exactly once
- `timer_future_is_send` — `TimerFuture: Send` (static assertion)

## Part 2: Build a Mini Executor

Build a single-threaded async executor that can run multiple futures
concurrently. This is a simplified version of what tokio does.

**Requirements:**

1. Implement a `Task` type that wraps a `Pin<Box<dyn Future<Output = ()>>>` and
   a `Waker`.

2. Implement a `Spawner` that accepts futures and sends them to the executor.

3. Implement an `Executor` that:
   - Receives tasks from the spawner via a channel
   - Polls each task when it's woken
   - Removes completed tasks
   - Sleeps when no tasks are ready (blocks the thread)

4. The executor should handle at least 10 concurrent tasks.

**Architecture:**
```
Spawner --[channel]--> Executor
                         |
                    polls tasks
                         |
                    tasks call waker.wake()
                         |
                    executor re-polls
```

**Tests to pass:**
- `executor_runs_single_task` — one future completes
- `executor_runs_concurrent_tasks` — 5 futures run concurrently
- `executor_handles_wake` — futures that return `Pending` then `Ready` work
- `executor_with_timer` — run `TimerFuture` from Part 1 on your executor

## Part 3: Diagnose Blocking-in-Async

This part gives you broken code to fix. The test suite includes an async
program with deliberate bugs.

**Bug 1: Blocking the runtime**
```rust
async fn process_batch(items: Vec<Item>) -> Vec<Result> {
    let mut results = Vec::new();
    for item in items {
        // BUG: std::fs::read_to_string blocks the async runtime
        let data = std::fs::read_to_string(&item.path)?;
        results.push(process(data).await);
    }
    results
}
```

Fix this using `tokio::fs` or `spawn_blocking`. Explain in a comment why the
original is broken and what happens to other tasks while this blocks.

**Bug 2: Task starvation**
```rust
async fn cpu_intensive_handler() -> Response {
    // BUG: This loop never yields, starving all other tasks on this thread
    let result = (0..10_000_000).fold(0u64, |acc, x| acc.wrapping_add(x * x));
    Json(result).into_response()
}
```

Fix this using `spawn_blocking` or `yield_now()`. Explain why this causes
starvation in a multi-task executor.

**Bug 3: Unbounded spawning**
```rust
async fn handle_events(mut rx: mpsc::UnboundedReceiver<Event>) {
    while let Some(event) = rx.recv().await {
        // BUG: spawns unbounded tasks, each holding memory
        tokio::spawn(async move {
            process_event(event).await;
        });
    }
}
```

Fix this using a semaphore or `FuturesUnordered` with a concurrency limit.
Explain the OOM risk.

**Tests to pass:**
- Each bugfix has a test verifying the fix works
- Each bugfix has a comment explaining the root cause

## Part 4: Backpressure-Aware Stream Processing

Build a stream processor that reads events from an async channel, processes
them with bounded concurrency, and writes results to an output channel.

**Requirements:**

1. Input: `tokio::sync::mpsc::Receiver<Event>` (bounded channel, capacity 100)
2. Processing: Each event takes 10-100ms to process (simulated)
3. Output: `tokio::sync::mpsc::Sender<Result>` (bounded channel, capacity 50)
4. Concurrency: At most 10 events processed simultaneously
5. Backpressure: When the output channel is full, processing pauses (no drops)
6. Graceful shutdown: When the input channel closes, drain all in-flight work

**Implementation:**

Use `tokio::sync::Semaphore` to limit concurrency. Use `tokio::select!` for
shutdown coordination. Track in-flight tasks with a `JoinSet` or
`FuturesUnordered`.

**Tests to pass:**
- `pipeline_processes_all_events` — every input event produces an output
- `pipeline_limits_concurrency` — at most 10 concurrent operations
- `pipeline_backpressure` — when output is full, pipeline slows down (doesn't OOM)
- `pipeline_graceful_shutdown` — closing input drains in-flight work
- `pipeline_throughput` — benchmark: processes at least 100 events/second

## Part 5: Rate-Limited Concurrent Fetcher

Build a function that fetches N URLs concurrently with:
- Maximum M concurrent requests
- Maximum R requests per second
- Timeout per request
- Retry with exponential backoff (max 3 retries)

```rust
pub async fn fetch_all(
    urls: Vec<String>,
    max_concurrent: usize,
    max_per_second: u32,
    timeout: Duration,
) -> Vec<Result<String, FetchError>> {
    todo!()
}
```

**Implementation should compose:**
- `Semaphore` for concurrency limiting
- `tokio::time::Interval` for rate limiting
- `tokio::time::timeout` for per-request timeout
- Retry logic with `tokio::time::sleep` for backoff

**Tests to pass:**
- `fetcher_respects_concurrency_limit` — never exceeds M concurrent
- `fetcher_respects_rate_limit` — never exceeds R per second
- `fetcher_retries_on_failure` — transient failures are retried
- `fetcher_timeout` — hung requests are abandoned after timeout
- `fetcher_all_results` — returns one result per input URL

## Testing

```
cargo test
```

The tests use `tokio::time::pause()` for deterministic timing where possible.
Some tests use real time and may take a few seconds.

## What You Will Learn

- How `Future`, `Poll`, `Pin`, `Waker`, and `Context` work together
- How an async executor polls and schedules tasks
- Why blocking in async context is catastrophic and how to detect it
- How to design backpressure-aware async pipelines
- How to compose semaphores, intervals, and timeouts for rate limiting
- How to diagnose async bugs with structured thinking
