# Project: Production Debugging

**Phase 3 — Production Engineering**

This project gives you broken code and production-like symptoms. You will
diagnose and fix real bugs using the tools production engineers use: backtraces,
debuggers, profilers, memory analyzers, system call tracers, and async debuggers.
No reading about tools — you use them.

## Prerequisites

- Completion of prod-1 through prod-4
- Building block bb-debug (Production Debugging)
- Linux environment (WSL2 or native — needed for `perf`, `strace`)

## Deliverables

- [ ] 6 bugs diagnosed and fixed using different debugging tools
- [ ] Flamegraph analysis with identified hotspot
- [ ] Memory leak found and fixed using heap profiling
- [ ] Async bug diagnosed using tokio-console
- [ ] A debugging runbook documenting your approach for each bug

---

## Part 1: Panic Investigation

The test suite contains a service that panics intermittently under concurrent
load. The panic is NOT obvious from reading the code.

**Setup:** `src/bug1_panic.rs` implements a concurrent cache with a panic that
occurs roughly 1 in 100 operations.

**Your task:**
1. Run the program with `RUST_BACKTRACE=full` and capture the backtrace
2. Identify the exact line causing the panic from the backtrace
3. Use `rust-gdb` (or `rust-lldb`) to set a breakpoint at the panicking function
4. Inspect the variable state at the time of panic
5. Identify the root cause (hint: it's a logic error in concurrent access)
6. Write a test that reliably reproduces the panic
7. Fix the bug
8. Document: what was the bug, how did you find it, how did you fix it

**Test to pass:** `bug1_no_panic_under_load` — 1000 concurrent operations, zero panics.

## Part 2: CPU Profiling with Flamegraph

The service in `src/bug2_slow.rs` is functional but slow. Response times are
10x what they should be. Your job: find the bottleneck.

**Your task:**
1. Run the service under `cargo flamegraph`:
   ```bash
   cargo flamegraph --bin bug2_slow -- --port 8080
   # In another terminal: send 1000 requests with wrk or hey
   ```
2. Open the SVG flamegraph in a browser
3. Identify the widest frame (the bottleneck)
4. Explain WHY this function is slow (it's not always obvious)
5. Fix the performance issue
6. Generate a new flamegraph and verify the bottleneck is gone
7. Measure before/after: latency p50, p99, and throughput

**Possible bottlenecks (you'll find one of these):**
- Unnecessary allocation in a hot loop (cloning where a reference works)
- O(n^2) algorithm where O(n log n) is possible
- Lock contention (holding a mutex across an I/O operation)
- Serialization in a hot path (JSON encoding that could be cached)

**Test to pass:** `bug2_performance` — p99 latency under 10ms for 100 concurrent
requests.

## Part 3: Memory Leak Detection

The service in `src/bug3_leak.rs` runs fine for a few minutes but its memory
usage grows unboundedly. After an hour it's using 2GB. Find the leak.

**Your task:**
1. Add `dhat` instrumentation:
   ```rust
   #[global_allocator]
   static ALLOC: dhat::Alloc = dhat::Alloc;

   fn main() {
       let _profiler = dhat::Profiler::new_heap();
       // ... rest of main
   }
   ```
2. Run the program and let it process events for 60 seconds
3. Analyze `dhat` output: which allocation site is growing without bound?
4. Identify the root cause (common causes: unbounded cache, `Arc` cycle,
   forgotten `JoinHandle`, growing `Vec` without cleanup)
5. Fix the leak
6. Re-run `dhat` and verify allocations are stable (total live bytes doesn't
   grow)

**Test to pass:** `bug3_memory_stable` — after processing 10,000 events, memory
usage is within 2x of the baseline (not growing linearly).

## Part 4: Async Task Debugging

The service in `src/bug4_async.rs` works under light load but hangs under heavy
load. Requests stop being processed even though the server is still running.

**Your task:**
1. Add `console-subscriber` to the service:
   ```rust
   console_subscriber::init();
   ```
2. Run with `RUSTFLAGS="--cfg tokio_unstable" cargo run`
3. In another terminal: `tokio-console`
4. Send increasing load until the service hangs
5. In tokio-console, identify:
   - How many tasks are alive?
   - How many are blocked/idle?
   - What's the longest poll duration?
   - Are any tasks starving others?
6. Identify the root cause (blocking call in async context, unbounded task
   spawning, channel deadlock, or semaphore exhaustion)
7. Fix the bug
8. Verify with tokio-console that tasks are healthy under load

**Test to pass:** `bug4_handles_load` — 500 concurrent requests complete within
5 seconds.

## Part 5: System Call Investigation

The service in `src/bug5_syscall.rs` is slower than expected, and the bottleneck
is NOT in Rust code — it's in how the program interacts with the OS.

**Your task:**
1. Run the service under `strace`:
   ```bash
   strace -f -c -S calls target/debug/bug5_syscall
   # Then send requests
   # -f = follow child threads, -c = summary, -S calls = sort by count
   ```
2. Identify which system call is called far too many times
3. Also run `strace -f -e trace=write` to see individual calls
4. Identify the root cause (common: unbuffered I/O, excessive `fsync`,
   per-byte reads, too many small `write` calls)
5. Fix by adding buffering, batching, or reducing syscall frequency
6. Re-run `strace -c` and compare syscall counts

**Test to pass:** `bug5_efficient_io` — throughput at least 5x improved after fix.

## Part 6: Race Condition with Miri

The code in `src/bug6_race.rs` contains unsafe code with a subtle memory safety
bug that doesn't crash on most runs but is technically undefined behavior.

**Your task:**
1. Run the tests under Miri:
   ```bash
   cargo +nightly miri test bug6
   ```
2. Read the Miri error output carefully
3. Understand the UB: what invariant is violated? (stacked borrows, data race,
   use after free, etc.)
4. Fix the unsafe code
5. Re-run Miri and verify it passes
6. Write a comment explaining the soundness invariant

**Test to pass:** `bug6_sound` — passes under both normal `cargo test` and
`cargo +nightly miri test`.

## Debugging Runbook

After fixing all 6 bugs, write `DEBUGGING_RUNBOOK.md` documenting:

For each bug:
1. **Symptom** — what did the user observe?
2. **Tool used** — what debugging tool found the root cause?
3. **Root cause** — what was actually wrong?
4. **Fix** — what did you change?
5. **Prevention** — how would you catch this earlier in the future?

This runbook is a real deliverable — you'll reference it in interviews and
on the job.

## Testing

```
cargo test
```

Some tests require `perf` and `strace` (Linux). If on macOS, use `dtrace`
and `Instruments` equivalents (the README for each bug file notes alternatives).

## What You Will Learn

- How to read Rust panic backtraces and find root causes
- How to use GDB/LLDB to step through Rust code
- How to generate and interpret CPU flamegraphs
- How to detect memory leaks with DHAT heap profiling
- How to diagnose async bugs with tokio-console
- How to trace system calls with strace
- How to detect undefined behavior with Miri
- A systematic approach to production debugging
