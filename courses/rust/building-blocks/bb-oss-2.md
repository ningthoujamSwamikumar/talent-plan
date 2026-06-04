# Building Block: Publishing Crates and Building Reputation

You've contributed to someone else's project. Now it's time to create something
of your own. Publishing a crate teaches you the full lifecycle of a Rust library:
API design, documentation, CI, versioning, and community maintenance.

## Readings

- [The Cargo Book: Publishing on crates.io](https://doc.rust-lang.org/cargo/reference/publishing.html)
  — the official guide. Read it end to end.
- [API Guidelines for Rust libraries](https://rust-lang.github.io/api-guidelines/)
  — the official Rust API design checklist. You'll use this as your quality bar.
- [How to write good crate documentation](https://blog.guillaume-gomez.fr/articles/2020-03-12+Guide+on+how+to+write+documentation+for+a+Rust+crate)
  — practical guide with examples.
- [Semver in the Rust ecosystem](https://doc.rust-lang.org/cargo/reference/semver.html)
  — what constitutes a breaking change in Rust.

## Key Concepts

**What makes a publishable crate?**

A crate worth publishing solves a specific, focused problem. It doesn't need to
be novel — a cleaner API for an existing problem is valuable. Look at your course
projects for extractable pieces:

- The rate limiter from advanced-2 could be a generic `tower` middleware crate
- The job queue from advanced-4 could be a standalone `pg-jobqueue` crate
- A utility you wrote multiple times (config loading, graceful shutdown) could
  be generalized

**Quality checklist before publishing:**

- [ ] `cargo clippy -- -D warnings` passes
- [ ] `cargo fmt --check` passes
- [ ] All public items have doc comments with examples
- [ ] `cargo doc --no-deps` generates clean documentation
- [ ] README.md with: what it does, install, quick example, license
- [ ] CI with GitHub Actions (test, clippy, fmt, doc)
- [ ] CHANGELOG.md
- [ ] LICENSE file (MIT or Apache-2.0, or dual-license both)
- [ ] Semantic versioning starting at 0.1.0

**Building an open source reputation over time:**

1. **Respond to issues within 48 hours** — even if just "Thanks, I'll look into
   this." Responsiveness is the #1 signal of a maintained project.
2. **Accept contributions gracefully** — review PRs constructively, merge good
   work, credit contributors.
3. **Write blog posts about your crate** — post on r/rust, Rust users forum, or
   your own blog. This drives adoption and builds your name.
4. **Keep dependencies updated** — `cargo outdated` and Dependabot/Renovate.

## Exercises

1. Pick one piece of your course work that could be extracted into a standalone
   crate. Write a 3-sentence pitch: what it does, who would use it, and why it's
   better than existing options (or why no alternative exists).

2. Run through the Rust API Guidelines checklist on your candidate crate. Note
   which guidelines you already satisfy and which need work.

3. Set up a GitHub Actions CI pipeline for a Rust library. It should run tests
   on stable, beta, and nightly; run clippy; check formatting; and build docs.
