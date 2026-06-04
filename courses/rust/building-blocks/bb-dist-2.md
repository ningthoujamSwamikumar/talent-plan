# Building Block Dist-2: Distributed Transactions

**Prerequisites**: [Raft Lab](../../dss/raft/README.md) complete.

Before starting the [Percolator Lab](../../dss/percolator/README.md), complete the
readings below.

## What to read

- [Large-scale Incremental Processing Using Distributed Transactions and Notifications (Percolator Paper)](https://storage.googleapis.com/pub-tools-public-publication-data/pdf/36726.pdf).
  Google's protocol for snapshot-isolation transactions over Bigtable.

- [Two-Phase Commit (2PC)](https://en.wikipedia.org/wiki/Two-phase_commit_protocol).
  The fundamental protocol for distributed atomic commits.

- [Snapshot Isolation](https://en.wikipedia.org/wiki/Snapshot_isolation).
  A concurrency control mechanism that provides a consistent snapshot of the
  database at the start of each transaction.

## Key concepts

### Percolator Architecture
- **Timestamp Oracle (TSO)**: Provides globally unique, monotonically increasing timestamps
- **Data Column**: Stores actual values with timestamp versions
- **Lock Column**: Tracks in-progress transactions
- **Write Column**: Records committed version pointers

### Two-Phase Commit in Percolator
1. **Prewrite**: Lock all keys, write tentative values
2. **Commit**: Write commit records, remove locks

### Why This Matters
Understanding distributed transactions is essential for senior engineers working
with microservices or distributed databases.

## Exercises

1. **Trace a transaction by hand.** Transaction T1 (start_ts=5) writes key A=1
   and key B=2. Primary is A. Walk through every step: prewrite A, prewrite B,
   get commit_ts=10, commit A, commit B. Show the state of the data, lock, and
   write columns after each step.

2. **Trace a conflict.** Transaction T1 (start_ts=5) writes A=1. Transaction T2
   (start_ts=7) writes A=2. T1 prewrites first and succeeds. T2 prewrites and
   finds T1's lock. What happens? Now reverse: T2 prewrites first. What happens
   to T1?

3. **Trace crash recovery.** Transaction T1 (start_ts=5, primary=A) has
   completed prewrite on both A and B, and committed A (write column updated,
   lock removed). Then the coordinator crashes before committing B. Transaction
   T2 encounters B's stale lock. Walk through T2's resolution process: how does
   it detect that T1 committed? How does it resolve B's lock?

4. **Compare isolation levels.** In your own words, explain the difference
   between: Read Committed, Repeatable Read, Snapshot Isolation, and
   Serializable. Which does Percolator provide? Give a concrete example of an
   anomaly that Snapshot Isolation allows but Serializable does not (write skew).

## You're ready when...

- [ ] You can explain snapshot isolation and how it differs from serializability
- [ ] You understand 2PC and why the primary key is the atomicity boundary
- [ ] You've read the Percolator paper and can describe the three columns
- [ ] You know how the TSO provides ordering guarantees
- [ ] You can trace through crash recovery for a partially committed transaction

Next: [Percolator Transactions](../projects/dist-2/README.md)
