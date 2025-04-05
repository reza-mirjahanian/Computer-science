

## **Distributed Consensus**
### **Definition:**
- **Distributed:**  
  - Involves multiple nodes working together.  
- **Consensus:**  
  - Agreement on a single value or system state among nodes.  

### **Applications:**
- **Databases:**  
  - Master-slave replication.  
- **Leader Election:**  
  - Choosing a leader from a group of nodes.  
- **Distributed Locks:**  
  - Sharing a resource among nodes one at a time.  

---

## **Overview of Consensus Protocols**
### **Popular Protocols:**
1. **Paxos**  
   - Introduced by Leslie Lamport in 1989.  
   - **Roles:**
     - **Client:** Requests changes to the system.  
     - **Proposer:** Advocates for the client.  
     - **Acceptors:** Approve changes through quorum/majority.  
     - **Learners:** Learn the new system state.  
     - **Leader:** Finalizes and distributes changes.  
   - **Challenges:**  
     - Complex and opaque.  
2. **Viewstamp Replication:**  
   - Similar to Raft, less widely used.  
3. **ZAB Protocol:**  
   - Used in Zookeeper.

---

## **Introduction to Raft**
### **Creators:**
- **Diego Ongaro:** PhD student at Stanford.  
- **John Ousterhout:** Professor at Stanford.  

### **History:**
- Released in **April/May** of the year discussed.  
- Reference implementation: **Log Cabin**, used in **RamCloud** project.  

### **Popularity:**
- **28 implementations** across various languages, including:
  - **Go (8 implementations)**.  
  - **F#**, developed by Hendrick.  
  - Commercial use by **CoreOS** (a Y Combinator company).  

---

## **Raft Protocol Overview**
### **Government Analogy:**
- Described as a **democratically elected dictatorship**.  

### **Roles in Raft:**
1. **Leader:**  
   - Responsible for maintaining the state and sending commands to followers.  
2. **Followers:**  
   - Accept commands and replicate the leader's state.  
3. **Candidates:**  
   - Followers that attempt to become leaders when no leader is present.  

---

## **Raft Process**
### **Leader Election:**
1. **Initial State:**  
   - All nodes start as **followers**.  
2. **Election Process:**  
   - If no leader is heard, a node switches to **candidate** status.  
   - Candidates request votes from followers.  
   - Majority vote results in a leader being elected.  
3. **Heartbeat Messages:**  
   - Leaders send regular messages to followers to maintain their authority.  
4. **Failure Handling:**  
   - If a leader dies or is unreachable, followers elect a new leader.  

### **Quorum Requirement:**  
- Majority of nodes must be functional for the cluster to operate.  

---

## **Log Replication**
### **Process:**
1. **Leader writes to its log.**  
2. **Replicates entries to followers** via append entries RPC.  
3. **Commitment:**  
   - Once a majority of followers write the entry, the leader commits it.  
   - Followers commit upon receiving confirmation from the leader.  

### **Example:**
- **Entry:** "Sally"  
  - Written to leader log → Replicated to followers → Committed across nodes.  
- **Next Entry:** "Bob"  
  - Process repeats for new entries.  

---

## **Network Partitions**
### **Challenges:**
- **Split Brain Issue:**  
  - Two leaders in separate partitions may attempt to commit conflicting changes.  
- **Resolution:**  
  - The majority in one partition operates as the functional cluster.  
  - Uncommitted entries are rolled back when the partition resolves.  

### **Example:**
1. **Partition Occurs:**  
   - Leader in partition A cannot commit due to lack of quorum.  
   - Partition B elects a new leader and commits changes.  
2. **Recovery:**  
   - Partition resolves, and nodes in partition A update their logs to match the new leader's state.  

---

## **Snapshotting**
### **Purpose:**
- Prevent logs from growing indefinitely and improve recovery time.  
- Avoid replaying all entries during node recovery.  

### **Strategies:**
1. **Leader-Initiated Snapshots:**  
   - Sliced snapshots embedded in logs.  
2. **Library-Initiated Snapshots:**  
   - Full block of snapshots sent out.  
3. **Application-Initiated Snapshots:**  
   - Controlled by the application, followed by log entries.  

---

## **Q&A Highlights**
### **Key Questions:**
1. **How do nodes know about each other?**  
   - Configuration specifies cluster membership.  
2. **What happens during asymmetric network partitions?**  
   - Requires bi-directional communication for leadership elections.  
3. **How does Raft handle adding/removing nodes?**  
   - Membership changes require careful handling to avoid disjoint majorities.  
4. **Can followers read/write directly?**  
   - Writing must go through the leader to ensure consistency.  

---

