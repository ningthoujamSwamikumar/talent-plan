# Building Block: RFCs, Design Docs, and Remote Communication

Senior engineers spend as much time writing as coding. In remote teams, this is
even more critical — you can't walk over to someone's desk to explain your
approach. Your ability to communicate technical decisions in writing directly
determines your career progression.

## Readings

- [Design Docs at Google](https://www.industrialempathy.com/posts/design-docs-at-google/)
  — how one of the largest engineering orgs uses design docs.
- [The RFC process in the Rust project](https://github.com/rust-lang/rfcs)
  — read 2-3 accepted RFCs to see the format and level of detail. Start with
  short ones like [RFC 2126](https://rust-lang.github.io/rfcs/2126-path-clarity.html).
- [How to write good async communication](https://nohello.net/en/) and
  [Don't ask to ask](https://dontasktoask.com/) — remote communication basics.
- [The Staff Engineer's Path](https://www.oreilly.com/library/view/the-staff-engineers/9781098118723/)
  — read Chapter 7 on "Writing" if you can access it. Otherwise, read
  [StaffEng.com](https://staffeng.com/) stories for how senior engineers communicate.

## Key Concepts

**Design doc structure (use this template):**

```markdown
# Title: [Feature/Change Name]
Author: [Name] | Date: [Date] | Status: [Draft/Review/Approved]

## Context
What problem are we solving? Why now? What's the current state?

## Goals and Non-Goals
- Goal: What this design achieves
- Non-Goal: What this design explicitly does NOT address

## Proposed Solution
Technical approach with enough detail that someone could implement it.
Include: data models, API contracts, sequence diagrams, error handling.

## Alternatives Considered
Other approaches and why they were rejected. This section is often the
most valuable — it shows your judgment.

## Risks and Mitigations
What could go wrong? How do we detect and recover?

## Rollout Plan
How do we deploy this safely? Feature flags? Gradual rollout?
```

**Code review as communication:**

Giving good code reviews is a senior skill. The principles:

1. **Be specific**: "This could race if two requests hit simultaneously" not
   "this seems wrong."
2. **Suggest, don't demand**: "Consider using `BTreeMap` here for ordered
   iteration" not "Use `BTreeMap`."
3. **Separate blocking from non-blocking**: Mark comments as "nit" (optional)
   vs. "blocking" (must fix).
4. **Review the design, not just the code**: Does this approach make sense? Is
   there a simpler way?

**Remote communication principles:**

- **Write things down**: If it's not written, it didn't happen. Meeting notes,
  decisions, context — all in writing.
- **Be async-first**: Don't default to meetings. A well-written message or
  document is more inclusive across time zones.
- **Over-communicate status**: "I'm blocked on X, switching to Y, will update
  by EOD Friday." No one should have to ask what you're working on.
- **Provide context**: "The auth middleware is returning 403 for valid tokens
  when the clock skew exceeds 30s" not "auth is broken."

## Exercises

1. Write a design doc for a feature you implemented in this course (pick one
   from Phase 4 or 5). Use the template above. Focus on the "Alternatives
   Considered" section — listing what you didn't do and why is the hardest and
   most valuable part.

2. Go to any open Rust project on GitHub and read 5 pull request reviews. Note
   examples of good and bad review comments. What patterns do the best reviewers
   use?

3. Practice writing a status update as if you're on a remote team. Describe what
   you accomplished this week on the course, what's blocked, and what you'll do
   next. Keep it under 10 lines.
