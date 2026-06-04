# Raft Consensus

**Phase 5 — Distributed Systems**

This project uses the Distributed Systems in Rust course located at
`courses/dss/raft/`. You will implement the Raft consensus algorithm in full:
leader election, log replication, persistence, and a linearizable key-value
store built on top of the replicated log.

Raft is the most important distributed systems algorithm for a backend
engineer to understand. It powers etcd, CockroachDB, TiKV, and many other
production systems. After completing this project, you will understand how
distributed consensus works at the implementation level — not just the theory.

## Prerequisites

- Completion of all prior phases
- Strong understanding of async Rust (tokio, futures, channels)
- Building block bb-dist-1 (Raft Consensus)
- **Read the Raft paper** before writing any code: "In Search of an
  Understandable Consensus Algorithm" (extended version). Focus on Figure 2 —
  it's your implementation specification.

## Overview

The Raft consensus algorithm allows a cluster of servers to agree on a
replicated log of commands even when some servers fail. Your implementation
will be tested using the **labrpc** simulation framework, which models an
unreliable network: messages can be delayed, reordered, duplicated, or dropped
entirely.

### The labrpc Framework

`labrpc` (found in `courses/dss/labrpc/`) provides:

- **Simulated RPC** — function-call-based message passing between nodes with
  configurable reliability.
- **Network partitions** — programmatic isolation of nodes to test split-brain
  and healing scenarios.
- **Deterministic testing** — reproducible failure injection.

You do not write real networking code; all communication goes through the
simulated RPC layer.

## Lab Sequence

### Lab 2A — Leader Election (1-2 days)

Implement the election mechanism so the cluster can elect a leader.

**What to implement:**
1. `RequestVote` RPC — candidates ask peers for votes
2. Election timeout — if no heartbeat received, start election
3. Term management — each election increments the term
4. Vote granting — a node votes for at most one candidate per term
5. Leader heartbeats — the leader sends periodic `AppendEntries` with no entries

**Key invariants:**
- At most one leader per term
- A leader's log is always at least as up-to-date as any voter's
- Election timeouts must be randomized (150-300ms) to avoid split votes

**Common mistakes:**
- Not resetting the election timer when granting a vote
- Not checking log up-to-dateness in `RequestVote`
- Using fixed instead of randomized election timeouts

**Tests:** `test_initial_election_2a`, `test_reelection_2a`,
`test_many_elections_2a`

### Lab 2B — Log Replication (2-4 days)

Implement log replication so the leader can append entries and commit them.

**What to implement:**
1. `AppendEntries` RPC — leader sends log entries to followers
2. Log matching — followers check `prevLogIndex` and `prevLogTerm`
3. Commit advancement — leader commits when a majority acknowledges
4. `nextIndex` and `matchIndex` tracking per follower
5. Conflict resolution — when a follower's log diverges, backtrack

**Key invariants:**
- If two entries have the same index and term, they're identical
- If two logs are identical up to an entry, all preceding entries are identical
- A leader never overwrites its own log entries

**Common mistakes:**
- Off-by-one errors in log indexing (Raft uses 1-based indexing)
- Not decrementing `nextIndex` correctly on `AppendEntries` rejection
- Committing entries from previous terms (violates safety — see Figure 8)

**Tests:** `test_basic_agree_2b`, `test_fail_agree_2b`,
`test_concurrent_starts_2b`, `test_rejoin_2b`

### Lab 2C — Persistence (1-2 days)

Persist Raft state so servers survive crashes.

**What to persist** (on every change):
- `currentTerm`
- `votedFor`
- `log[]`

**What NOT to persist** (reconstructed after restart):
- `commitIndex`
- `lastApplied`
- `nextIndex[]` / `matchIndex[]`

Use the `Persister` trait provided by the framework. Serialize with `serde`
and `bincode`.

**Common mistakes:**
- Not persisting before responding to RPCs
- Persisting too much (leader state is volatile)
- Not handling the case where persisted state is corrupted/empty

**Tests:** `test_persist1_2c`, `test_persist2_2c`, `test_persist3_2c`,
`test_figure8_2c`, `test_unreliable_agree_2c`

### Lab 3A — Key/Value Service Without Log Compaction (2-3 days)

Build a linearizable key-value store on top of Raft.

**What to implement:**
1. `Get(key)` — returns the value (linearizable read)
2. `Put(key, value)` — sets the value
3. `Append(key, value)` — appends to existing value
4. Client-side retry logic with duplicate detection
5. Each operation goes through the Raft log

**Key challenge:** Duplicate detection. If a client retries a `Put` because it
didn't get a response (but the first `Put` was committed), the second `Put`
must be detected and ignored. Use client IDs and sequence numbers.

**Tests:** `test_basic_3a`, `test_concurrent_3a`, `test_unreliable_3a`

### Lab 3B — Log Compaction with Snapshots (2-3 days)

Add snapshotting so the Raft log doesn't grow without bound.

**What to implement:**
1. `InstallSnapshot` RPC — leader sends a snapshot to slow followers
2. Snapshot creation — when the log exceeds a threshold, snapshot the state
3. Log trimming — discard entries before the snapshot index
4. Startup from snapshot — restore state machine from snapshot on restart

**Key invariants:**
- The snapshot replaces all log entries up to and including the snapshot index
- `InstallSnapshot` must be handled correctly even if the follower already has
  a more recent snapshot

**Tests:** `test_snapshot_basic_3b`, `test_snapshot_install_3b`,
`test_snapshot_recover_3b`

## Architecture Guidance

```
┌─────────┐     ┌─────────┐     ┌─────────┐
│ Client   │────▶│ KV Store │────▶│  Raft    │
│          │◀────│ (state   │◀────│ (log,    │
│          │     │  machine)│     │  election│
└─────────┘     └─────────┘     │  commit) │
                                 └────┬─────┘
                                      │ RPC
                              ┌───────┼───────┐
                              ▼       ▼       ▼
                           Peer 1  Peer 2  Peer 3
```

**Concurrency model:**
- The Raft module runs in its own task
- Client requests are submitted via a channel and applied when committed
- Use `tokio::sync::oneshot` for client-to-Raft notification when committed

## Debugging Tips

- **Add tracing at every state transition:** leader election, vote grant, log
  append, commit advance. You will need these logs.
- **Test one lab at a time.** Don't move to 2B until 2A passes reliably.
- **Run tests multiple times.** Raft bugs are often non-deterministic:
  `for i in $(seq 1 50); do cargo test test_2a; done`
- **Read the students' guide:** The TAs wrote a guide for common pitfalls.
  It's linked in bb-dist-1.

## Estimated Time

2 to 4 weeks, depending on your comfort with async Rust and distributed
algorithms. This is the hardest project in the course.

## Getting Started

```bash
cd courses/dss/raft/
cargo test
```

Work through the labs in order. Each lab has its own set of tests that must
pass before moving on.
