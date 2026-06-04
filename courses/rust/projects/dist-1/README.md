# Raft Consensus (DSS Lab)

This project uses the existing Distributed Systems in Rust course located at
`courses/dss/raft/`. Rather than duplicating the material, this document serves
as a guide for integrating the Raft labs into the Talent Plan learning path.

## Prerequisites

- Completion of all prior phases (Foundations, Networking/Web, Production, and
  Senior-level projects).
- Strong understanding of async Rust (`tokio`, `futures`, pinning, task
  coordination).
- Familiarity with basic distributed systems concepts (replication, consensus,
  fault tolerance).

## Building Block

Read [bb-dist-1.md](../../../building-blocks/bb-dist-1.md) before starting.

## Overview

The Raft consensus algorithm allows a cluster of servers to agree on a
replicated log of commands even when some servers fail. Your implementation
will be tested using the **labrpc** simulation framework, which models an
unreliable network: messages can be delayed, reordered, duplicated, or dropped
entirely. This lets you exercise failure scenarios without needing real network
partitions.

### The labrpc Framework

`labrpc` (found in `courses/dss/labrpc/`) provides:

- **Simulated RPC** — function-call-based message passing between nodes with
  configurable reliability.
- **Network partitions** — programmatic isolation of nodes to test split-brain
  and healing scenarios.
- **Deterministic testing** — reproducible failure injection so tests are not
  flaky.

You do not need to write any real networking code; all communication goes
through the simulated RPC layer.

## Lab Sequence

1. **Read the Raft paper.** Lamport's "In Search of an Understandable Consensus
   Algorithm" (the extended version) is the primary reference. Pay close
   attention to Figure 2.

2. **Lab 2A — Leader Election.** Implement `RequestVote` RPC and the election
   timeout mechanism. Your cluster should reliably elect a single leader and
   re-elect after the leader crashes.

3. **Lab 2B — Log Replication.** Implement `AppendEntries` RPC. The leader
   replicates log entries to followers and commits them when a majority
   acknowledges.

4. **Lab 2C — Persistence.** Persist Raft state (current term, voted-for, log)
   so that servers survive restarts without violating safety.

5. **Lab 3A — Key/Value Service without Log Compaction.** Build a linearizable
   key/value store on top of Raft. Clients send `Get`, `Put`, and `Append`
   operations; the service applies them via the replicated log.

6. **Lab 3B — Key/Value Service with Log Compaction.** Add snapshotting so that
   the log does not grow without bound.

## Estimated Time

2 to 4 weeks, depending on your comfort with async Rust and distributed
algorithms.

## Getting Started

```bash
cd courses/dss/raft/
cargo test
```

Work through the labs in order. Each lab has its own set of tests that must pass
before moving on.
