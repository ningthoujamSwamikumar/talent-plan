# Building Block Prod-3: Performance Profiling and Optimization

**Prerequisites**: [Project: Testing Mastery](../projects/prod-2/README.md).

Before starting [Project: Performance & Profiling](../projects/prod-3/README.md),
complete the readings and exercises below.

## What to read

- [The Rust Performance Book](https://nnethercote.github.io/perf-book/).
  The definitive guide. Covers profiling tools, optimization techniques, and
  common performance pitfalls in Rust.

- [Criterion User Guide](https://bheisler.github.io/criterion.rs/book/).
  How to write reliable benchmarks with statistical analysis.

- [Flamegraph interpretation](https://www.brendangregg.com/flamegraphs.html).
  How to read flamegraphs and identify hot code paths.

- [DHAT documentation](https://docs.rs/dhat/latest/dhat/).
  Dynamic heap analysis tool for tracking allocations.

- [Moka crate documentation](https://docs.rs/moka/latest/moka/).
  A high-performance concurrent cache for Rust.

## Key concepts

### Profiling Before Optimizing

Never optimize without profiling first. The bottleneck is rarely where you think.

Tools:
- **flamegraph**: CPU time visualization → identifies hot functions
- **dhat**: Heap allocation tracking → identifies unnecessary allocations
- **criterion**: Benchmarking → measures before/after impact

### Common Rust Optimizations

| Technique | When | Example |
|-----------|------|---------|
| `Cow<str>` | Avoid cloning when not always needed | Function that sometimes modifies input |
| `String` → `&str` | Avoid ownership where borrowing suffices | Function parameters |
| `Vec::with_capacity` | Known collection size | Building a Vec in a loop |
| `collect::<Vec<_>>()` hints | Help the optimizer | Iterator chains |
| Zero-copy deserialization | Parsing large payloads | `serde_json::from_slice` |

### Caching Strategies

| Strategy | Description | Use When |
|----------|-------------|----------|
| Cache-aside | App checks cache → miss → load from DB → write cache | Most common |
| Write-through | Write to cache and DB simultaneously | Strong consistency needed |
| Write-behind | Write to cache, async write to DB | Write-heavy, eventual consistency OK |

## Exercises

**Exercise 1**: Write a criterion benchmark for a function. Run it, then optimize
the function. Verify the benchmark shows improvement.

**Exercise 2**: Profile a provided slow program with `cargo flamegraph` and
identify the bottleneck.

**Exercise 3**: Use `Cow<str>` to avoid allocations in a function that only
sometimes modifies its input.

## You're ready when...

- [ ] You can write criterion benchmarks and interpret results
- [ ] You can read a flamegraph and identify hot paths
- [ ] You know common Rust optimization techniques
- [ ] You understand caching strategies and tradeoffs

Next: [Project: Performance & Profiling](../projects/prod-3/README.md)
