# Building Block: Contributing to Open Source Rust Projects

Before starting your first open source contribution, you need to understand how
the Rust open source ecosystem works and how to find the right project to
contribute to.

## Readings

Read these before starting the project:

- [How to Contribute to Open Source](https://opensource.guide/how-to-contribute/)
  — the canonical guide. Read sections 1-4.
- [The Rust community's contributing guidelines](https://rustc-dev-guide.rust-lang.org/)
  — skim the structure to see how a large Rust project organizes contributions.
- [Finding good first issues on GitHub](https://github.com/topics/good-first-issue?l=rust)
  — browse this filtered list. Notice how different projects label issues.
- Read the CONTRIBUTING.md of three Rust projects you've used in this course:
  [tokio](https://github.com/tokio-rs/tokio/blob/master/CONTRIBUTING.md),
  [axum](https://github.com/tokio-rs/axum/blob/main/CONTRIBUTING.md), and
  [serde](https://github.com/serde-rs/serde/blob/master/CONTRIBUTING.md).

## Key Concepts

**How to evaluate a project for your first contribution:**

1. **Activity**: Check the last commit date and issue response time. A project
   with no commits in 6 months is likely unmaintained.
2. **Friendliness**: Look for "good first issue" labels, a CONTRIBUTING.md, and
   polite responses to newcomer PRs.
3. **Size**: Start with mid-size projects (100-10K stars). Mega-projects (tokio,
   rustc) have complex CI and review processes. Tiny projects may not review your
   PR for months.
4. **Relevance**: Contribute to something you've actually used. You'll understand
   the codebase faster and care about the outcome.

**Types of contributions that build credibility (ranked by impact):**

1. Bug fixes with tests — the gold standard.
2. Documentation improvements with examples — always welcome, undervalued.
3. Adding or improving test coverage — shows you understand the codebase.
4. Performance improvements with benchmarks — shows senior-level thinking.
5. Triaging issues, reproducing bugs — builds relationships with maintainers.

**What NOT to do:**

- Don't submit trivial PRs (typo fixes in comments, reformatting) to pad your
  contribution graph. Maintainers notice and it hurts your reputation.
- Don't submit a PR without reading the CONTRIBUTING.md first.
- Don't claim an issue and then ghost. If you can't finish, comment and unassign
  yourself.
- Don't argue with maintainers about style. Their project, their rules.

## Exercises

1. Find 5 Rust projects on GitHub that have open "good first issue" labels.
   For each, note: star count, last commit date, contributor count, and whether
   the CONTRIBUTING.md exists. Pick the best candidate for your first PR.

2. Clone your chosen project. Build it. Run the tests. Read the project structure.
   Spend at least 30 minutes reading code before you touch anything.

3. Read 3 merged PRs from other contributors to understand the project's review
   culture: how detailed are reviews? How long do they take? What do maintainers
   care about?

---

## Career Checkpoint

From this point forward, every piece of code you write should be written as if
a hiring manager will see it. Clean commit messages. Clear PR descriptions.
Professional tone in all GitHub interactions. Your open source profile IS your
resume for remote Rust jobs.
