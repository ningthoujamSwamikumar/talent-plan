# Project: Publish a Crate

**Phase 8 — Open Source & Community**

In this project you will extract a reusable piece of functionality from your
course work, polish it into a proper library, and publish it to crates.io.

Publishing a crate demonstrates that you can design a public API, write
documentation, set up CI, manage versions, and think about downstream users.
These are senior engineering skills that most job candidates never demonstrate.

## Deliverables

By the end of this project you will have:

- [ ] A published crate on crates.io
- [ ] A GitHub repository with CI (tests, clippy, fmt, docs)
- [ ] Complete documentation with examples for all public items
- [ ] A README with installation, usage, and examples
- [ ] A CHANGELOG.md with your initial release notes

## Part 1: Choose What to Extract

Look through your course projects for code that could be useful to others.
Good candidates:

| Source Project | Candidate | Crate Idea |
|---------------|-----------|------------|
| advanced-2 | Rate limiter | Generic `tower` rate-limiting middleware |
| advanced-2 | Circuit breaker | Configurable circuit breaker for any async fn |
| advanced-4 | Job queue | PostgreSQL-backed job queue with `SKIP LOCKED` |
| prod-1 | Health check framework | Composable health checks for axum services |
| prod-3 | Benchmark harness helpers | Criterion helpers for async benchmarks |
| project-2 | Log-structured storage | Bitcask-style storage engine library |
| dist-3 | Service registry | In-process service discovery for testing |

**Selection criteria:**

- Does this solve a real problem? Search crates.io — is there an existing crate?
  If so, can you do it better, simpler, or differently?
- Is it self-contained? Can you extract it without pulling in your entire project?
- Is it small enough to maintain? Start with something you can document in a
  single README. 200-500 lines of library code is ideal for a first crate.

## Part 2: Extract and Refactor

1. **Create a new Cargo library project:**
   ```sh
   cargo new --lib my-crate-name
   cd my-crate-name
   ```

2. **Copy the relevant code** from your course project. Then refactor:
   - Remove any dependencies on your course project's types
   - Make the API generic where it makes sense (e.g., accept `impl Tower` not
     a specific middleware stack)
   - Remove hard-coded configuration — use builder patterns or config structs

3. **Design your public API.** Run through the
   [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/checklist.html):
   - [ ] Types eagerly implement common traits (C-COMMON-TRAITS)
   - [ ] Functions validate arguments (C-VALIDATE)
   - [ ] Types are `Send` and `Sync` where possible (C-SEND-SYNC)
   - [ ] API uses the type system to prevent misuse (C-CUSTOM-TYPE)
   - [ ] Constructor follows `Type::new()` convention (C-CTOR)

4. **Write error types** using `thiserror`. No `anyhow` in a library crate —
   downstream users need structured errors they can match on.

## Part 3: Documentation

Every public item must have a doc comment. The standard:

```rust
/// A rate limiter that uses the token bucket algorithm.
///
/// # Examples
///
/// ```
/// use my_crate::RateLimiter;
///
/// let limiter = RateLimiter::builder()
///     .max_tokens(100)
///     .refill_rate(10)
///     .build();
///
/// assert!(limiter.try_acquire().is_ok());
/// ```
///
/// # Panics
///
/// Panics if `max_tokens` is zero.
pub struct RateLimiter { /* ... */ }
```

**Documentation checklist:**

- [ ] Crate-level doc comment in `lib.rs` with overview and example
- [ ] Every public struct, enum, trait, and function has a doc comment
- [ ] At least one `# Examples` section per major type
- [ ] `# Panics` section where applicable
- [ ] `# Errors` section for functions returning `Result`

Run `cargo doc --open` and read through the generated documentation as a user
would. Is it clear? Can someone use your crate without reading the source?

## Part 4: Testing and CI

1. **Write comprehensive tests:**
   - Unit tests in `src/` modules
   - Integration tests in `tests/`
   - Doc tests (the examples in your doc comments — these run automatically)

2. **Set up GitHub Actions CI** — create `.github/workflows/ci.yml`:

   ```yaml
   name: CI
   on: [push, pull_request]
   jobs:
     test:
       runs-on: ubuntu-latest
       strategy:
         matrix:
           rust: [stable, beta, nightly]
       steps:
         - uses: actions/checkout@v4
         - uses: dtolnay/rust-toolchain@master
           with:
             toolchain: ${{ matrix.rust }}
             components: clippy, rustfmt
         - run: cargo test --all-features
         - run: cargo clippy --all-features -- -D warnings
         - run: cargo fmt --check
         - run: cargo doc --no-deps
   ```

3. **Add a CI badge** to your README.

## Part 5: Prepare for Publishing

1. **Fill out `Cargo.toml` metadata:**

   ```toml
   [package]
   name = "my-crate-name"
   version = "0.1.0"
   edition = "2021"
   description = "A one-sentence description of what this crate does"
   license = "MIT OR Apache-2.0"
   repository = "https://github.com/yourusername/my-crate-name"
   documentation = "https://docs.rs/my-crate-name"
   readme = "README.md"
   keywords = ["rust", "relevant", "keywords"]
   categories = ["relevant-category"]
   ```

2. **Create README.md** with:
   - What the crate does (1-2 sentences)
   - Installation (`cargo add my-crate-name`)
   - Quick start example
   - Link to full documentation
   - License

3. **Create CHANGELOG.md:**
   ```markdown
   # Changelog

   ## 0.1.0 — YYYY-MM-DD

   Initial release.

   - Feature 1
   - Feature 2
   ```

4. **Dry run:**
   ```sh
   cargo publish --dry-run
   ```
   Fix any warnings before publishing for real.

## Part 6: Publish

1. Create an account on [crates.io](https://crates.io/) (log in with GitHub).
2. Get your API token from crates.io account settings.
3. Log in from the CLI:
   ```sh
   cargo login YOUR_API_TOKEN
   ```
4. Publish:
   ```sh
   cargo publish
   ```

Your crate is now live. Share it on r/rust, the Rust users forum, or your blog.

## Success Criteria

- Crate is published on crates.io and installable via `cargo add`
- All public items are documented with examples
- CI passes on stable, beta, and nightly
- README clearly explains what the crate does and how to use it
- At least 10 tests covering the main functionality
