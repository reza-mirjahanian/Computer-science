
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