# Distributed Transactions — Percolator (DSS Lab)

This project uses the existing Distributed Systems in Rust course located at
`courses/dss/percolator/`. It covers the Percolator distributed transaction
protocol, originally described in Google's paper on incremental processing for
large datasets.

## Prerequisites

- Completion of the Raft lab (dist-1) — you must be comfortable with replicated
  state machines and distributed consensus before tackling transactions.
- Understanding of MVCC (Multi-Version Concurrency Control) concepts.

## Building Block

Read [bb-dist-2.md](../../../building-blocks/bb-dist-2.md) before starting.

## Overview

Percolator provides snapshot-isolation transactions on top of a distributed
key/value store. The protocol uses a two-phase commit scheme with a clever
use of timestamps and lock columns to detect conflicts. Your implementation
will handle:

- **Prewrite** — acquire locks and write tentative values.
- **Commit** — atomically make all writes visible by replacing locks with
  commit timestamps.
- **Conflict detection** — abort transactions that conflict with concurrent
  writes.
- **Rollback / cleanup** — handle crashed coordinators and stale locks.

## Estimated Time

1 to 2 weeks.

## Getting Started

```bash
cd courses/dss/percolator/
cargo test
```

Refer to the Percolator paper and the test suite for expected behavior.
