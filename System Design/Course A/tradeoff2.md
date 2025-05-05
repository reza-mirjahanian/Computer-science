### Data Management Trade-offs in System Design

#### **SQL vs NoSQL Databases**
- **SQL Databases**
  - *Strengths:*
    - Strong consistency
    - Structured schemas
    - Powerful query capabilities
    - Ensures data integrity
  - *Challenges:*
    - Difficult to horizontally scale
    - Schema modifications can be cumbersome

- **NoSQL Databases**
  - *Strengths:*
    - Horizontal scalability
    - Flexible schema design
  - *Trade-offs:*
    - Often sacrifice some level of consistency
    - Limited query capabilities compared to SQL

- **Key Trade-off:**  
  _SQL trades scalability for consistency and structure._  
  _NoSQL trades consistency and query power for scalability and flexibility._

---

#### **Normalization vs Denormalization**

- **Normalization**
  - *Definition:* Organizing data into separate tables to minimize redundancy.
  - *Benefits:*
    - Maintains data integrity
    - Efficient storage usage
  - *Drawbacks:*
    - Performance degradation due to joins at scale

- **Denormalization**
  - *Definition:* Intentional duplication of data across tables.
  - *Benefits:*
    - Eliminates expensive joins
    - Improves read performance
  - *Drawbacks:*
    - Increases complexity in write operations
    - Potential for data inconsistencies

- **Evolution of Design:**  
  Most systems start with normalized designs and adopt strategic denormalization as performance demands increase.

---

#### **CAP Theorem in Distributed Systems**

- **Consistency**  
  Users always receive the most recent data.

- **Availability**  
  System remains operational despite failures.

- **Partition Tolerance**  
  System continues to function even if network partitions occur.

- **Trade-off:**  
  Only two out of the three guarantees can be achieved simultaneously.

- **Use Case Examples:**
  - *Banking systems:* Prioritize consistency for accurate account balances.
  - *Social media platforms:* Favor availability to ensure uninterrupted user access.

---

#### **Consistency Spectrum**

- **Strong Consistency**
  - Updates are immediately visible across all nodes.
  - Requires synchronization, which can impact performance.

- **Eventual Consistency**
  - Updates propagate gradually; temporary stale data may be seen.
  - Offers better speed and scalability.

- **Decision Factor:**  
  Business requirements determine where on the spectrum a system should operate.

---

#### **Batch vs Stream Processing**

- **Batch Processing**
  - *Approach:* Accumulates data and processes it at scheduled intervals.
  - *Benefits:*
    - Computationally efficient
    - Easier error handling
  - *Limitations:*
    - High latency (hours before insights are available)

- **Stream Processing**
  - *Approach:* Processes data in real time as it arrives.
  - *Benefits:*
    - Immediate results
    - Enables instant response to events
  - *Challenges:*
    - Complex state management
    - Handling out-of-order data
    - Variable latencies affecting correctness

- **Core Trade-off:**  
  _Batch processing favors efficiency and simplicity._  
  _Stream processing favors immediacy and responsiveness._

- **Modern Trend:**  
  Hybrid architectures combining stream processing for real-time needs and batch for comprehensive analysis.