
In the world of distributed systems, ensuring that nodes can agree on a single, consistent state is crucial. Raft is a consensus algorithm designed to achieve this by providing fault tolerance and ensuring that a single leader is elected to coordinate operations.

It’s designed to be easier to understand than previous algorithms like Paxos while providing strong fault tolerance and leader election capabilities.


Core Concepts:
==============

1.  **Nodes (Servers):** In Raft, the network is composed of several nodes or servers.
2.  **Leader:** One of the nodes in the Raft cluster is elected as the leader. The leader is responsible for managing the replication of logs across the cluster.
3.  **Follower:** All other nodes in the cluster are followers. They respond to requests from the leader and forward client requests to the leader.
4.  **Candidate:** When a leader fails, a new leader needs to be elected. Nodes transition to the candidate state and initiate an election.
5.  **Term:** Raft operates in terms, where each term begins with an election and ends with a new leader being elected or re-elected.
6.  **Log Replication:** Raft ensures that all logs across the cluster are replicated and maintained in the same order.

-----------

Consensus and Leader Election:
==============================

The primary goal of Raft is to achieve consensus among nodes in the cluster regarding the state of the system. Here's how it works:

1.  **Leader Election:**

-   At the beginning of each term, nodes start as followers.
-   If a follower doesn't hear from the leader for a certain period (election timeout), it transitions to the candidate state.
-   The candidate requests votes from other nodes. If it receives votes from the majority, it becomes the leader.
-   If no node receives a majority, a new election is started in the next term.


**Log Replication and Consistency:**

-   The leader accepts client requests and appends them to its log.
-   It then sends the log entry to followers, which replicate the log entry.
-   Once a majority of followers acknowledge the entry, it's committed to the log and applied to the state machine.


---------------------

1.  **Log Replication:**

-   When a client initiates an operation, such as setting a key-value pair, the leader node receives the request.
-   The leader appends the operation to its log and broadcasts this log entry to all other nodes in the cluster, including peer nodes.
-   Each node in the cluster appends the log entry to its log.

2. **Majority Agreement:**

-   Raft operates on the principle of majority agreement. Before committing an operation to its state machine, the leader node waits for acknowledgments from most nodes.
-   If most of the nodes(`Say N/2 + 1`) acknowledge the operation by replicating it in their logs, the leader commits the operation to its state machine.
-   This ensures that the operation is officially part of the system's state and will be applied consistently across all nodes.

Additionally, the leader regularly sends updates to the other servers to keep them in sync. This ensures that even if a server falls behind or crashes, it can quickly catch up with the latest state of the key-value store.