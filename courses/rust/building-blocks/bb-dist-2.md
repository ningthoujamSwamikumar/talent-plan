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

## You're ready when...

- [ ] You can explain snapshot isolation
- [ ] You understand 2PC (prepare/commit/abort)
- [ ] You've read the Percolator paper
- [ ] You know how the TSO provides ordering guarantees

Next: [Percolator Lab](../../dss/percolator/README.md)
