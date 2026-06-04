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

## Exercises

1. **Trace through a leader election by hand.** Three servers: S1, S2, S3. S1
   is the leader in term 1. S1 crashes. Walk through every RPC message until a
   new leader is elected. Note: what term is the new leader in? What happens
   when S1 comes back?

2. **Trace through log replication.** Leader has log entries [A, B, C]. Follower
   has [A, B]. Leader sends AppendEntries. Walk through the message exchange
   until the follower's log matches the leader's and the entries are committed.

3. **Trace through a partition scenario.** Five servers: S1 (leader), S2, S3,
   S4, S5. Network partitions into {S1, S2} and {S3, S4, S5}. A client sends
   a write to S1. Another client sends a write to S3 (which will elect a new
   leader). What happens when the partition heals? Walk through every step.

4. **Identify the bug.** A Raft implementation allows a leader in term 3 to
   commit an entry from term 2 by counting replicas. Why is this unsafe? (Hint:
   read the Raft paper Figure 8 and the paragraph about it.)

## You're ready when...

- [ ] You can explain Raft's leader election algorithm from memory
- [ ] You understand log replication, commitment, and the role of `matchIndex`
- [ ] You know what happens during a network partition and how safety is maintained
- [ ] You can trace through Figure 8 in the paper and explain why it matters
- [ ] You've read the Raft paper at least once (twice is better)

Next: [Raft Consensus](../projects/dist-1/README.md)
