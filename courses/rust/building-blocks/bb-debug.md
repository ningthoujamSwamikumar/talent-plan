# Building Block: Production Debugging

The difference between a mid-level and a senior engineer is what happens when
something breaks at 2am. Mid-level engineers read logs and guess. Senior engineers
use systematic tools to find root causes in minutes. This building block teaches
you the debugging toolkit that production Rust engineers use daily.

## Readings

- [Rust debugging with GDB/LLDB](https://rustc-dev-guide.rust-lang.org/debugging-support-in-rustc.html)
  — how Rust's debug info works with native debuggers.
- [perf examples for Rust](https://nnethercote.github.io/perf-book/) — the Rust
  Performance Book. Focus on the profiling chapters.
- [Flamegraph](https://github.com/flamegraph-rs/flamegraph) — the `cargo flamegraph`
  tool. Install it and learn to read flamegraphs.
- [tokio-console](https://github.com/tokio-rs/console) — async task debugging.
  Learn to identify blocked tasks, slow polls, and task starvation.
- [strace for Rust developers](https://jvns.ca/blog/2015/04/14/strace-zine/) —
  Julia Evans' strace zine. Short, practical, essential.
- [DHAT (dynamic heap analysis)](https://docs.rs/dhat/latest/dhat/) — Rust-native
  heap profiler for finding allocation hotspots.

## Key Concepts

**The debugging hierarchy (try in order):**

1. **Read the error message carefully.** Rust's error messages are better than
   most languages. The backtrace (`RUST_BACKTRACE=1`) tells you exactly where
   a panic happened.

2. **Reproduce first, debug second.** Write a test that triggers the bug before
   you try to fix it. If you can't reproduce it, you can't verify your fix.

3. **Add tracing instrumentation.** Before reaching for a debugger, add
   `tracing::debug!` at key decision points. Most production bugs are found
   through structured logs, not debuggers.

4. **Use specialized tools for specific problems:**

   | Problem | Tool | What It Shows |
   |---------|------|---------------|
   | Panic/crash | `RUST_BACKTRACE=full` | Stack trace with file:line |
   | Logic bug | `gdb`/`lldb` | Step through code, inspect variables |
   | Memory leak | `dhat`, `valgrind --tool=massif` | Allocation sites, growth over time |
   | CPU hotspot | `perf record` + `flamegraph` | Where CPU time is spent |
   | Async hang | `tokio-console` | Blocked tasks, slow polls, task counts |
   | Syscall issue | `strace -f -e trace=network` | What the OS sees |
   | Data race | `cargo +nightly miri test` | Undefined behavior detection |
   | Slow query | `EXPLAIN ANALYZE` (PostgreSQL) | Query plan, actual times |

**Using GDB/LLDB with Rust:**

```bash
# Build with debug info (default in dev profile)
cargo build

# Launch in debugger
rust-gdb target/debug/my_program
# or
rust-lldb target/debug/my_program

# Useful commands:
# break my_module::my_function    — set breakpoint
# run                             — start program
# bt                              — backtrace
# print variable_name             — inspect variable
# next / step                     — step over / step into
# watch variable_name             — break when variable changes
```

**Reading flamegraphs:**

A flamegraph is a visualization of stack traces sampled during execution. The
x-axis is NOT time — it's alphabetically sorted stack frames. The y-axis is
call depth. Width represents proportion of samples.

What to look for:
- **Wide plateaus at the bottom** — functions that consume the most CPU
- **Tall, narrow towers** — deep call stacks (may indicate excessive recursion)
- **Unexpected functions** — allocator calls in a hot path, unnecessary clones
- **Missing functions** — if your function doesn't appear, it's not the bottleneck

```bash
# Generate a flamegraph
cargo flamegraph --bin my_program -- --my-args

# For specific benchmarks
cargo flamegraph --bench my_benchmark
```

**Debugging async code:**

Async bugs are harder because:
- Stack traces show the executor, not your code
- Tasks may be on different threads at different times
- Deadlocks look like "nothing is happening"

Use `tokio-console`:
```bash
# Add to Cargo.toml
# [dependencies]
# console-subscriber = "0.2"

# In main.rs:
# console_subscriber::init();

# Run with:
RUSTFLAGS="--cfg tokio_unstable" cargo run

# In another terminal:
tokio-console
```

## Exercises

1. **Debug a panic.** Given a program that panics under specific input, use
   `RUST_BACKTRACE=full` and `gdb` to find the exact line and variable state
   that causes the panic. Write a test that reproduces it.

2. **Profile with flamegraph.** Run `cargo flamegraph` on one of your earlier
   course projects (the KV store from Phase 1 is ideal). Identify the hottest
   function. Can you optimize it?

3. **Find a memory leak.** Write a program that leaks memory (e.g., an
   `Arc` cycle or a `Vec` that grows without bound). Use `dhat` to identify
   the allocation site. Fix the leak and verify with `dhat` that allocations
   are stable.

4. **Use strace.** Run `strace -e trace=network` on a running HTTP server.
   Make a request and read the syscall trace. Identify: the `accept`, `read`,
   `write`, and `close` calls. What socket options are set?

5. **Diagnose with tokio-console.** Write an async program with a deliberate
   performance problem (e.g., a task that calls `std::thread::sleep` instead
   of `tokio::time::sleep`). Use tokio-console to identify the problematic
   task.

---

## Checklist

- [ ] You can read a Rust panic backtrace and find the root cause
- [ ] You can use `gdb`/`lldb` to set breakpoints and inspect variables
- [ ] You can generate and interpret flamegraphs
- [ ] You can use `dhat` or `valgrind` to find memory allocation issues
- [ ] You can use `strace` to trace system calls
- [ ] You can use `tokio-console` to debug async task problems
- [ ] You default to structured logging (`tracing`) before reaching for debuggers

Next: [Project: Production Debugging](../projects/prod-5/README.md)
