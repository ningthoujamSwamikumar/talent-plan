# Building Block Sr-3: System Design and Interview Preparation

**Prerequisites**: [Project: Security Hardening](../projects/senior-2/README.md).

Before starting [Project: System Design & Interview](../projects/senior-3/README.md),
complete the readings below.

## What to read

- [Designing Data-Intensive Applications (DDIA)](https://dataintensive.net/)
  by Martin Kleppmann. Chapters 1-3, 5-6, and 9 are essential. This is the
  single most important book for backend engineering interviews.

- [System Design Interview Framework](https://blog.bytebytego.com/p/a-framework-for-system-design-interviews).
  A structured approach: requirements → estimation → high-level design →
  deep dive → wrap up.

- [Google's Code Review Guidelines](https://google.github.io/eng-practices/review/).
  How to review code effectively: what to look for, how to give feedback.

- [Effective Code Review Practices](https://github.com/google/eng-practices/blob/master/review/reviewer/looking-for.md).
  Checklist: correctness, design, complexity, tests, naming, comments.

## Key concepts

### System Design Framework

1. **Clarify Requirements** (2-3 min): Functional + non-functional (scale, latency, consistency)
2. **Capacity Estimation** (3-5 min): QPS, storage, bandwidth
3. **High-Level Design** (10-15 min): Components, data flow, API design
4. **Deep Dive** (10-15 min): Pick 1-2 components to detail
5. **Wrap Up** (3-5 min): Tradeoffs, failure modes, monitoring

### Back-of-Envelope Estimation

| Resource | Latency/Size |
|----------|-------------|
| L1 cache | 0.5 ns |
| L2 cache | 7 ns |
| Main memory | 100 ns |
| SSD read | 150 μs |
| HDD read | 10 ms |
| Network round trip (same DC) | 0.5 ms |
| Network round trip (cross-continent) | 150 ms |

### Code Review Checklist
1. Does the code do what it claims?
2. Are there edge cases not handled?
3. Is error handling correct and complete?
4. Are there performance concerns at scale?
5. Is the code testable?
6. Is the API ergonomic and well-documented?

## Exercises

**Exercise 1**: Design a URL shortener on paper. Estimate storage for 100M URLs.
Choose a hash/encoding strategy. Design the API. Discuss read/write ratio.

**Exercise 2**: Review the provided buggy code and write review comments. Identify
correctness bugs, performance issues, and style problems.

**Exercise 3**: Practice explaining a technical decision you made in this course
(e.g., why axum over actix, why sqlx over diesel). Structure: context, options,
decision, tradeoffs.

## You're ready when...

- [ ] You can structure a system design answer in 35 minutes
- [ ] You can estimate capacity (QPS, storage, bandwidth)
- [ ] You can give constructive code review feedback
- [ ] You can articulate technical tradeoffs clearly

Next: [Project: System Design & Interview](../projects/senior-3/README.md)
