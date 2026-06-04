# What's Next After This Course

You've completed the course and (ideally) landed a job. Congratulations. But
your learning doesn't stop — if anything, it accelerates now that you're working
on real production systems. This guide covers specialization paths, advanced
topics, and how to keep growing.

## Your First 90 Days on the Job

The course taught you how to build. The job will teach you how to operate. Focus
on these in your first three months:

1. **Learn the codebase** — Read more code than you write for the first 2-4
   weeks. Understand how the team's Rust idioms differ from what you learned.
2. **Ship small wins early** — Fix bugs, improve error messages, add tests.
   Build trust before proposing big changes.
3. **Ask questions** — "Why does this use `Arc<Mutex<>>` instead of `dashmap`?"
   is a great question. Experienced teammates will appreciate your curiosity.
4. **Write things down** — Document what you learn. The codebase wiki that
   nobody updates? Be the person who updates it.
5. **Set up your dev environment properly** — rust-analyzer, clippy config,
   test shortcuts. You'll be in this codebase for a while.

## Specialization Paths

After building a solid backend foundation, you can specialize. Pick the path
that aligns with the work your team does or the direction you want to go.

### Path A: Distributed Systems & Infrastructure
**For**: Platform engineering, database internals, cloud infrastructure

- **Study**: Designing Data-Intensive Applications (Kleppmann), the Raft paper
  in depth, CockroachDB architecture docs
- **Build**: A simple distributed KV store with sharding, a gossip protocol
  implementation, a log-structured merge tree (LSM)
- **Contribute to**: tikv, surrealdb, materialize, neon, or other Rust databases
- **Read**: jepsen.io for distributed systems testing, Aphyr's blog

### Path B: Performance Engineering
**For**: Low-latency systems, trading, game servers, embedded

- **Study**: "Performance Analysis and Tuning on Modern CPUs" (Easyperf), cache
  behavior, SIMD, allocation strategies
- **Build**: A memory allocator, a zero-copy parser, a lock-free data structure
- **Tools**: `perf`, `flamegraph`, `cargo-asm`, `criterion`, `dhat`
- **Contribute to**: tokio (performance PRs), rayon, crossbeam

### Path C: WebAssembly (Wasm)
**For**: Edge computing, browser-side Rust, plugin systems

- **Study**: The WebAssembly spec, WASI, component model
- **Build**: A Wasm plugin system, a browser-side data processing library
- **Frameworks**: wasmtime, wasmer, wasm-bindgen, leptos, yew
- **Contribute to**: wasmtime, fermyon/spin, bytecodealliance projects

### Path D: Security & Cryptography
**For**: Security-critical systems, crypto, authentication

- **Study**: Cryptography Engineering (Ferguson), the RustCrypto project, formal
  verification basics
- **Build**: A TLS implementation study, a secure enclave interface, an audit
  tool
- **Contribute to**: rustls, ring, RustCrypto, cargo-audit
- **Certifications**: Consider OSCP or similar if moving into security

### Path E: Embedded & Systems
**For**: IoT, firmware, operating systems, real-time systems

- **Study**: "The Embedded Rust Book", RTIC framework, no_std development
- **Build**: An embedded project on a real board (STM32, ESP32), a driver
- **Frameworks**: embassy, RTIC, probe-rs
- **Contribute to**: embedded-hal, embassy, probe-rs

## Advanced Rust Topics

Regardless of specialization, these topics will deepen your Rust expertise:

- **Unsafe Rust in depth**: The Rustonomicon, when and how to write sound
  unsafe code, Miri for verification
- **Async internals**: How the executor works, writing your own Future,
  understanding Pin in depth
- **Procedural macros**: Writing derive macros, attribute macros, macro
  hygiene
- **Compiler internals**: Contributing to rustc, understanding MIR and
  codegen
- **Type system tricks**: GATs, higher-ranked trait bounds, type-level
  programming

## Community Involvement

Keep participating in the community. As you gain experience, shift from
learning to teaching:

- **Answer questions** on Stack Overflow, Rust Discord, and the users forum
- **Write blog posts** about problems you solve at work (check with your
  employer first)
- **Give talks** at local meetups or conferences — start with lightning talks
  (5 min)
- **Maintain your crate** — respond to issues, accept PRs, release updates
- **Mentor someone** — help the next person through this course or a similar
  path

## Conferences

- **RustConf** — the main Rust conference
- **EuroRust** — European Rust conference
- **Rust Nation** — UK-based Rust conference
- **RustLab** — Italy-based Rust conference
- **Local meetups** — search meetup.com for your area

Attending conferences (even virtually) keeps you connected to the ecosystem
and exposes you to ideas and people outside your immediate team.

## Continuous Learning Resources

- [This Week in Rust](https://this-week-in-rust.org/) — weekly newsletter
- [Rust Blog](https://blog.rust-lang.org/) — official announcements
- [Inside Rust Blog](https://blog.rust-lang.org/inside-rust/) — development updates
- [Rust RFC repository](https://github.com/rust-lang/rfcs) — upcoming language changes
- [Jon Gjengset's YouTube channel](https://www.youtube.com/c/JonGjengset) — advanced Rust
- [Rust for Rustaceans](https://rust-for-rustaceans.com/) (book) — intermediate to advanced

## The Long Game

Senior engineering is a career, not a destination. The skills that matter most
in years 3-5+ are not technical:

- **Mentoring**: Can you make other engineers better?
- **Technical leadership**: Can you drive technical direction for a team?
- **Communication**: Can you explain complex systems to non-technical stakeholders?
- **Judgment**: Can you decide what NOT to build?

The best investment you can make after getting hired is investing in these skills.
Read "The Staff Engineer's Path" by Tanya Reilly and "An Elegant Puzzle" by Will
Larson for what comes next.
