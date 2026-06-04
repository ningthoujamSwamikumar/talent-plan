# Project: Performance and Profiling

**Task:** Profile and optimize a web service, adding caching and benchmarks.

In this project you will take a working but unoptimized task management service and
systematically measure, profile, and improve its performance. You will learn to write
benchmarks before optimizing, use profiling tools to find hot paths, add caching for
read-heavy endpoints, and verify your improvements with load tests.

## Part 1: Benchmark First

Before changing any code, establish a performance baseline using Criterion benchmarks.

- Read through `benches/benchmarks.rs` and implement the four benchmark stubs.
- `bench_cache_hit`: measure the latency of reading a key that exists in the cache.
- `bench_cache_miss`: measure the latency of reading a key that does not exist.
- `bench_serialization`: measure serializing a `Task` struct to JSON bytes.
- `bench_concurrent_reads`: measure throughput when many tasks read from the cache
  at the same time using `tokio::spawn`.

Run benchmarks with:

```
cargo bench
```

Record the baseline numbers. You will compare against these after each optimization.

## Part 2: Profiling

Use profiling tools to identify where the service spends its time and memory.

### CPU profiling with flamegraph

Install and run `cargo flamegraph`:

```
cargo install flamegraph
cargo flamegraph --bench benchmarks
```

Open the generated `flamegraph.svg` in a browser. Look for:
- Unexpectedly wide frames (functions that consume a large share of CPU time).
- Deep call stacks that could be flattened.
- Allocation-heavy paths (`alloc::`, `__rust_alloc`).

### Heap profiling with dhat

Add `dhat` as a dev-dependency and annotate your benchmark binary:

```rust
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;
```

Run the benchmarks and open the generated `dhat-heap.json` in the
[dhat viewer](https://nnethercote.github.io/dh_view/dh_view.html).

Identify the top allocation sites. These are candidates for optimization in Parts 5
and 6.

## Part 3: In-Memory Caching with Moka

Implement the `TaskCache` struct in `src/cache.rs`.

- Wrap a `moka::future::Cache<String, String>` with a configured max capacity and
  time-to-live (TTL).
- Implement `get`, `set`, `invalidate`, and `invalidate_prefix`.
- `invalidate_prefix` should remove all entries whose key starts with the given
  prefix. This is useful for clearing all cached responses related to a specific
  resource (e.g., all task-list pages when a task is created).

Integrate the cache into the Axum service so that `GET` endpoints check the cache
before hitting the data store, and return cached JSON when available.

## Part 4: Cache Invalidation

Correct cache invalidation is critical. Implement the following strategy:

- **On create/update/delete:** invalidate the specific entry (`task:{id}`) and also
  invalidate all list entries (`tasks:list:*`) because the collection has changed.
- **On read (cache miss):** fetch from the data store, serialize, store in the cache,
  then return the response.
- **TTL as a safety net:** even if invalidation is missed, entries expire after the
  configured TTL so stale data is bounded.

Write tests that verify:
1. A read after a write returns the updated value (not a stale cached copy).
2. Deleting a task removes it from both the store and the cache.
3. List endpoints reflect newly created tasks even when the list was cached.

## Part 5: Avoiding Allocations

Open `src/optimized.rs`. You will find three "slow" baseline functions paired with
`todo!()` optimized versions.

### 5a: Clone-heavy string processing

The baseline `process_tags_slow` clones every string. Rewrite
`process_tags_optimized` using `Cow<str>` so that strings are only cloned when they
actually need to be modified.

### 5b: Vec without capacity

The baseline `collect_ids_slow` pushes into a `Vec` without pre-allocating. Rewrite
`collect_ids_optimized` using `Vec::with_capacity` to avoid repeated reallocations.

### 5c: Serialization via String

The baseline `serialize_slow` serializes to a `String` and then converts to bytes.
Rewrite `serialize_optimized` to serialize directly to `Vec<u8>` using
`serde_json::to_vec`.

Run the benchmarks after each change and compare against the baseline.

## Part 6: Response Compression

Add gzip compression for JSON responses larger than a threshold.

- Implement `compress_response` in `src/optimized.rs`. Use `flate2` (add it as a
  dependency) to gzip-compress a byte slice.
- Implement `decompress_response` for testing.
- Only compress responses above a configurable size threshold (e.g., 1024 bytes).
- Set the `Content-Encoding: gzip` header when returning compressed responses.

Verify with a test that compressing and then decompressing a payload returns the
original bytes.

## Part 7: Load Testing

Write a load-test client that measures latency percentiles under concurrent load.

- Implement `calculate_percentile` in `src/optimized.rs` to compute p50, p95, and
  p99 from a sorted slice of durations.
- Write an async load test (in `tests/` or as an example binary) that:
  1. Spawns N concurrent Tokio tasks, each making M sequential HTTP requests.
  2. Records the duration of each request.
  3. Sorts the durations and prints p50, p95, and p99 latency.
- Ensure the test can run against a local instance of the service and produces
  a clear summary.

Target: the p99 latency for cached reads should be under 10 ms on a warm cache.
