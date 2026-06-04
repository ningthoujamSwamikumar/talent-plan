# Building Block Dist-1: Raft Consensus

**Prerequisites**: Phase 4 complete. Strong understanding of networking and concurrency.

Before starting the [Raft Lab](../../dss/raft/README.md), complete the readings below.

## What to read

- [In Search of an Understandable Consensus Algorithm (Raft Paper)](https://raft.github.io/raft.pdf).
  Read the entire paper. This is the most important reading in the distributed
  systems section. Pay special attention to Figures 2 and 3.

- [Raft Visualization](https://raft.github.io/).
  Interactive visualization of leader election and log replication.

- [The labrpc Framework](../../dss/labrpc/).
  Understand the simulated network used by the labs. It allows injecting
  network partitions, message drops, and delays for testing.

- [Students' Guide to Raft](https://thesquareplanet.com/blog/students-guide-to-raft/).
  Practical tips from a TA on common mistakes when implementing Raft.

## Key concepts

### Consensus Problem
Multiple servers must agree on a sequence of commands, even when some
servers crash or messages are lost.

### Raft Roles
- **Leader**: Handles all client requests, replicates log entries
- **Follower**: Responds to leader's AppendEntries RPCs
- **Candidate**: Requests votes during leader election

### Safety Properties
1. Election Safety: at most one leader per term
2. Leader Append-Only: leader never overwrites its log
3. Log Matching: if two logs have same index+term, all preceding entries match
4. Leader Completeness: committed entries appear in all future leaders' logs

## You're ready when...

- [ ] You can explain Raft's leader election algorithm
- [ ] You understand log replication and commitment
- [ ] You know what happens during a network partition
- [ ] You've read the Raft paper at least once

Next: [Raft Lab](../../dss/raft/README.md)
