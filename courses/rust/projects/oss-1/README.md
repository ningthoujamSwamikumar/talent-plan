# Project: First Open Source Contribution

**Phase 8 — Open Source & Community**

Your goal in this project is to make a real contribution to a real Rust open
source project. Not a course exercise — an actual PR to a codebase used by
other developers in production.

This is the single most important thing you can do for your employability beyond
the technical coursework. A merged PR to a known project is proof that you can
read unfamiliar code, work with a team, follow conventions, and ship.

## Deliverables

By the end of this project you will have:

- [ ] Identified a target open source Rust project
- [ ] Set up the project locally (clone, build, run tests)
- [ ] Read and understood the contribution guidelines
- [ ] Found an issue to work on (or identified a fix/improvement yourself)
- [ ] Submitted a pull request with tests and documentation
- [ ] Responded to review feedback and iterated

The PR does not need to be merged to complete this project — the submission and
review process is the learning. But aim for a mergeable PR.

## Part 1: Choose Your Target Project

**Criteria for a good first target:**

| Factor | Good Sign | Bad Sign |
|--------|-----------|----------|
| Activity | Commits in the last month | No commits in 3+ months |
| Issue response time | Maintainers reply within a week | Issues pile up with no response |
| Contributor friendliness | "good first issue" labels exist | No contribution labels |
| Documentation | CONTRIBUTING.md exists | No contributing guide |
| CI | GitHub Actions or similar CI runs on PRs | No CI |
| Size | 100-10,000 stars | <10 stars (may be abandoned) or >50K (complex process) |

**Recommended project categories based on what you know from this course:**

- **Web frameworks**: axum ecosystem, tower middleware, axum-extra
- **Async runtime**: tokio ecosystem, tokio-util, tokio-console
- **Serialization**: serde ecosystem, serde_json, serde_yaml
- **Database**: sqlx, sea-orm, diesel
- **Testing**: proptest, insta, cargo-nextest
- **CLI tools**: clap, bat, ripgrep, fd
- **Observability**: tracing ecosystem, metrics-rs
- **Other**: any crate you used and liked in this course

**What to search for:**

```sh
# Search GitHub for Rust projects with good first issues
# https://github.com/issues?q=is%3Aopen+is%3Aissue+language%3Arust+label%3A%22good+first+issue%22+sort%3Aupdated-desc

# Or use the GitHub CLI:
gh search issues --language rust --label "good first issue" --state open --sort updated
```

## Part 2: Set Up and Explore

Once you've chosen a project:

1. **Fork and clone** the repository.

2. **Build it.** Follow the README or CONTRIBUTING.md instructions exactly. If the
   build fails, that's a valid issue to file or fix.

   ```sh
   cargo build
   cargo test
   cargo clippy
   ```

3. **Read the project structure.** Spend at least 1 hour reading code before
   changing anything. Understand:
   - How is the crate organized? (`lib.rs` → modules)
   - What are the core types and traits?
   - Where are the tests?
   - How does CI work?

4. **Read 3 recently merged PRs.** Note:
   - How detailed are PR descriptions?
   - How long did review take?
   - What kinds of things do maintainers comment on?
   - What's the test expectation?

## Part 3: Find Your Issue

**Option A: Pick an existing "good first issue"**

- Comment on the issue: "I'd like to work on this. I've read the contributing
  guide and have the project building locally. Is this still available?"
- Wait for a maintainer response before starting work. Don't start on an issue
  that's already assigned.

**Option B: Identify something yourself**

These are always welcome:
- A function that's missing a doc comment or has an incorrect example
- A test case that's missing (you found an edge case)
- A performance improvement you can demonstrate with a benchmark
- A compiler warning or clippy lint that should be fixed
- An error message that's confusing and could be improved

## Part 4: Do the Work

1. **Create a branch** with a descriptive name: `fix-timeout-panic-on-empty-queue`
   not `my-fix`.

2. **Write the smallest possible change that fixes the issue.** Don't refactor
   surrounding code. Don't add features. Fix the one thing.

3. **Add or update tests.** If you're fixing a bug, write a test that fails
   without your fix and passes with it.

4. **Run the full test suite** before submitting:
   ```sh
   cargo test
   cargo clippy -- -D warnings
   cargo fmt --check
   ```

5. **Write a clear commit message:**
   ```
   Fix timeout panic when queue is empty

   The `poll_next` method panicked with "index out of bounds" when called
   on an empty queue with a timeout of 0. This was because the timeout
   branch didn't check the queue length before indexing.

   Added a length check before the index operation and a regression test.

   Fixes #1234
   ```

## Part 5: Submit and Iterate

1. **Write a thorough PR description:**
   - What does this PR do?
   - Why? (link to the issue)
   - How did you test it?
   - Any concerns or questions for reviewers?

2. **Be patient.** Maintainers are volunteers. A week with no response is normal.
   Two weeks is worth a polite ping.

3. **Respond to all review feedback.** Even if you disagree, be respectful.
   "Thanks for the suggestion — I considered that approach but went with X
   because Y. Happy to change if you prefer." is always the right tone.

4. **Iterate until merged or abandoned.** If the maintainer asks for changes,
   make them promptly. If the PR is rejected, that's okay — you still learned.

## Part 6: Reflect

Write a brief document (for yourself, not to submit anywhere) answering:

1. What was the hardest part of contributing to an unfamiliar codebase?
2. What surprised you about the review process?
3. What would you do differently next time?
4. How does this experience compare to working on course projects alone?

Keep this reflection — you'll use it in your behavioral interview preparation.

## Success Criteria

- You submitted a PR to a real Rust open source project
- The PR includes tests (if applicable to the change type)
- You responded professionally to any review feedback
- You can explain what you changed and why to someone unfamiliar with the project

## Going Further

Don't stop at one contribution. Aim for:
- **3 merged PRs** across 1-2 projects before you start job searching
- At least **1 non-trivial contribution** (not just docs/typos)
- **1 project where a maintainer recognizes you** — this often leads to referrals
