# Building Block: Advanced Async — Futures, Pin, and Executors

You've been using `async/await` and tokio since Phase 1. Now you need to
understand what happens underneath. When a production service has mysterious
hangs, unbounded memory growth from spawned tasks, or performance cliffs at
scale — the answer is always in the async internals. This building block takes
you from async user to async expert.

## Readings

- [Asynchronous Programming in Rust (Async Book)](https://rust-lang.github.io/async-book/)
  — read chapters 1-4 end-to-end. Pay special attention to Chapter 2 (Under the
  Hood) and Chapter 4 (Pinning).
- [Pin, Unpin, and why Rust needs them](https://blog.cloudflare.com/pin-and-unpin-in-rust/)
  — Cloudflare's practical explanation. The best Pin tutorial available.
- [How Tokio works: a deep dive](https://tokio.rs/tokio/tutorial) — re-read the
  tutorial, but this time focus on the "What is a Runtime" section and
  the mini-tokio example.
- [Writing an async runtime from scratch (part 1)](https://www.youtube.com/watch?v=9_3krAQtD2k)
  — Phil Opp or equivalent resource on building a minimal executor.
- [tokio-console](https://github.com/tokio-rs/console) — the async debugger.
  Install it and learn to read its output.
- [FuturesUnordered and its performance pitfalls](https://blog.yoshuawuyts.com/futures-concurrency/)
  — why buffered streams matter and how to size them.

## Key Concepts

**What `async fn` compiles to:**

```rust
// You write:
async fn fetch(url: &str) -> String { /* ... */ }

// The compiler generates (approximately):
fn fetch(url: &str) -> impl Future<Output = String> {
    // A state machine enum with one variant per .await point
    enum FetchFuture {
        Start { url: String },
        AwaitingResponse { /* saved state */ },
        Done,
    }
    // impl Future for FetchFuture { fn poll(...) -> Poll<String> { ... } }
}
```

Understanding this is not academic. It explains:
- Why futures are zero-cost (no heap allocation for the state machine itself)
- Why `async fn` return types implement `Future` not `AsyncFn`
- Why you can't return references across `.await` points
- Why `Send` bounds on futures care about what you hold across `.await`

**Pin and why it exists:**

The state machine above may contain self-references (a reference to data stored
in an earlier variant). If you move the future in memory, those references
become dangling. `Pin<&mut T>` is a contract: "I promise not to move this value."

When you need to care about Pin:
- Implementing `Future` manually
- Working with `Stream` implementations
- Understanding why `tokio::pin!` exists
- Debugging "the trait bound `T: Unpin` is not satisfied" errors

**Executor mechanics:**

An async runtime has three components:
1. **Reactor** — watches I/O sources (epoll/kqueue) and wakes tasks when ready
2. **Scheduler** — decides which woken task to poll next (work-stealing in tokio)
3. **Task** — a `Pin<Box<dyn Future>>` with a waker

Understanding this explains:
- Why blocking in async context is catastrophic (blocks the scheduler thread)
- Why `spawn_blocking` exists (moves work off the scheduler)
- Why `tokio::task::yield_now()` helps fairness
- How to diagnose "all tasks are blocked" scenarios with tokio-console

**Backpressure in async pipelines:**

Unbounded channels and unbounded spawning are the most common async bugs in
production. A fast producer + slow consumer + unbounded channel = OOM.

Patterns:
- `tokio::sync::mpsc::channel(bound)` — bounded channels
- `tokio::sync::Semaphore` — limit concurrent operations
- `futures::stream::StreamExt::buffered(n)` — limit concurrent futures in a stream
- `FuturesUnordered` with manual capacity management

## Exercises

1. **Implement a minimal Future by hand.** Write a `Delay` future that
   completes after a specified duration without using `tokio::time::sleep`.
   Use `std::task::Waker` to re-poll after the duration. This teaches you the
   `poll()` → `Pending`/`Ready` lifecycle.

2. **Build a mini executor.** Write a single-threaded executor that can run
   one future to completion. It should: create a waker, poll the future, sleep
   if pending, wake and re-poll. Under 100 lines of code. Then extend it to
   run multiple futures concurrently (not in parallel).

3. **Diagnose a blocking-in-async bug.** Write a tokio program that accidentally
   calls `std::thread::sleep` inside an async task. Use `tokio-console` to
   observe the blocked task. Then fix it with `spawn_blocking`. Note the
   difference in tokio-console output.

4. **Demonstrate backpressure failure.** Write a producer-consumer with an
   unbounded channel where the producer is 10x faster than the consumer.
   Observe memory growth. Then fix it with a bounded channel and measure the
   throughput difference.

5. **Implement a rate-limited stream processor.** Given an async stream of
   items, process at most N items concurrently, with a maximum rate of R
   items/second. Use `tokio::sync::Semaphore` + `tokio::time::Interval`.

---

## Checklist

- [ ] You can implement `Future` manually with `poll()`, `Pin`, and `Waker`
- [ ] You understand why `Pin` prevents moving self-referential futures
- [ ] You can explain what `Unpin` means and when a type is `Unpin`
- [ ] You can build a minimal single-threaded executor
- [ ] You can diagnose blocking-in-async with tokio-console
- [ ] You understand bounded vs unbounded channels and their memory implications
- [ ] You can use `Semaphore` and `buffered()` for backpressure control

Next: [Project: Advanced Async Deep Dive](../projects/project-6/README.md)
