# Distributed Transactions — Percolator

**Phase 5 — Distributed Systems**

This project uses the Distributed Systems in Rust course located at
`courses/dss/percolator/`. You will implement the Percolator distributed
transaction protocol, originally described in Google's 2010 paper on
incremental processing for large datasets.

Percolator is how Google Search's indexing pipeline processes updates
transactionally across thousands of machines. Variations of this protocol are
used in TiDB, CockroachDB, and Spanner. Understanding it teaches you how
production distributed databases achieve snapshot isolation.

## Prerequisites

- Completion of the Raft lab (dist-1) — you must be comfortable with replicated
  state machines and distributed consensus.
- Building block bb-dist-2 (Distributed Transactions)
- **Read the Percolator paper** before writing code: "Large-scale Incremental
  Processing Using Distributed Transactions and Notifications"

## Overview

Percolator provides **snapshot-isolation transactions** on top of a distributed
key/value store. The protocol uses a two-phase commit scheme with a clever use
of timestamps and lock columns to detect conflicts.

### How Percolator Works

Each key in the store has three columns:
- **data** — the actual value, versioned by timestamp
- **lock** — indicates a pending transaction holds this key
- **write** — maps a commit timestamp to the data timestamp, making writes visible

**Transaction flow:**

```
1. Begin: get a start_timestamp from the timestamp oracle (TSO)
2. Read: read data at start_timestamp (snapshot isolation)
3. Buffer writes locally (don't write to store yet)
4. Prewrite (Phase 1):
   a. Pick one key as the "primary"
   b. For each key: check for write-write conflicts, then write data + lock
   c. If any conflict → abort
5. Commit (Phase 2):
   a. Get a commit_timestamp from TSO
   b. Write the "write" column for the primary key, removing its lock
   c. For each secondary key: write "write" column, remove lock
   d. Primary commit is the point of no return
```

### Why This Design

- **Snapshot isolation**: Each transaction sees a consistent snapshot at
  `start_timestamp`. No dirty reads, no non-repeatable reads.
- **Decentralized locking**: Locks are stored in the data store itself, not in
  a lock manager. Any node can check for conflicts.
- **Crash recovery**: If a coordinator crashes mid-commit, other transactions
  can detect stale locks (by checking if the primary lock still exists) and
  roll them back.

## Implementation Guide

### Part 1: Timestamp Oracle (TSO)

Implement a simple timestamp oracle that returns monotonically increasing
timestamps.

```rust
pub struct TimestampOracle {
    next: AtomicU64,
}

impl TimestampOracle {
    pub fn get_timestamp(&self) -> u64 {
        self.next.fetch_add(1, Ordering::SeqCst)
    }
}
```

In production (TiDB), the TSO is a replicated service. For this lab, a single
atomic counter suffices.

### Part 2: Multi-Version Store

Implement the storage layer with three column families:

```rust
pub struct MvccStore {
    data: BTreeMap<(Vec<u8>, u64), Vec<u8>>,    // (key, ts) -> value
    lock: BTreeMap<Vec<u8>, Lock>,               // key -> lock info
    write: BTreeMap<(Vec<u8>, u64), Write>,      // (key, ts) -> write record
}

pub struct Lock {
    primary: Vec<u8>,  // primary key of the transaction
    ts: u64,           // start timestamp
}

pub struct Write {
    start_ts: u64,     // points to the data entry
    kind: WriteKind,   // Put or Delete
}
```

### Part 3: Prewrite

Implement the prewrite phase:

1. **Check for write-write conflicts**: Is there a `write` entry with
   `commit_ts > start_ts`? If so, abort — another transaction committed
   after our snapshot.
2. **Check for lock conflicts**: Is there a `lock` entry from another
   transaction? If so, either wait for it to resolve or abort.
3. **Write data**: Store the value at `(key, start_ts)` in the data column.
4. **Write lock**: Store a lock pointing to the primary key.

### Part 4: Commit

Implement the commit phase:

1. Get `commit_ts` from TSO (must be > `start_ts`).
2. **Commit primary**: Write a `write` record at `(primary_key, commit_ts)`
   pointing to `start_ts`, then remove the primary lock.
3. **Commit secondaries**: Same for each secondary key.
4. The primary commit is atomic — if it succeeds, the transaction is committed
   even if the coordinator crashes before committing secondaries.

### Part 5: Read

Implement snapshot reads:

1. Check for locks at `key` with `ts <= start_ts`. If a lock exists from
   another transaction, it might commit at a timestamp in our snapshot.
   Wait or abort.
2. Find the latest `write` entry at `key` with `commit_ts <= start_ts`.
3. Use the `write.start_ts` to read the actual data from the data column.

### Part 6: Conflict Resolution and Cleanup

Handle the case where a transaction coordinator crashes:

1. If you encounter a stale lock (the locking transaction's primary lock is
   gone), the transaction was committed — resolve by writing the write record.
2. If the primary lock still exists but the transaction has been running too
   long (TTL exceeded), the transaction is presumed dead — roll back by
   removing the lock and data.

## Common Pitfalls

- **Timestamp ordering**: `commit_ts` must always be greater than `start_ts`.
  The TSO guarantees this.
- **Primary first**: Always commit the primary key before secondaries. This is
  the atomicity boundary.
- **Lock cleanup**: A transaction that finds a stale lock MUST check the
  primary before deciding to roll forward or roll back.
- **Read-your-writes**: A transaction should see its own buffered writes, not
  just the snapshot.

## Testing

```bash
cd courses/dss/percolator/
cargo test
```

Tests cover:
- Basic single-key transactions
- Multi-key transactions with commit and abort
- Write-write conflict detection
- Read-after-write consistency
- Crash recovery with stale lock cleanup
- Concurrent transactions with proper isolation

## Estimated Time

1 to 2 weeks.

## What You Will Learn

- Snapshot isolation and MVCC (multi-version concurrency control)
- Two-phase commit protocol
- Distributed conflict detection without a central lock manager
- Crash recovery for distributed transactions
- Timestamp oracle design
- The foundation for understanding TiDB, CockroachDB, and Spanner transactions
