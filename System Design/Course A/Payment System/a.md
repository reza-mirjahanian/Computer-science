### **E-commerce Payment System Overview**  
- **Definition**: Trading goods/services via the internet, facilitated by backend payment systems.  
- **Key Challenges**:  
  - **Reliability & correctness**: Critical to avoid revenue loss.  
  - **Scalability**: Handle high payment request volumes.  
  - **Availability**: Minimize downtime.  

---

### **Core Components**  
1. **Payment Gateway**:  
   - Validates financial credentials.  
   - Transfers funds to the merchant’s bank.  
   - Manages compliance (e.g., **PCI DSS**, **GDPR**).  
   - Integrates fraud/risk prevention services.  

2. **Payment Service Provider (PSP)**:  
   - Facilitates payments securely (e.g., Stripe, PayPal).  
   - Offers risk management, reconciliation, and order management tools.  
   - Avoids storing sensitive card data in-house.  

3. **Acquiring Bank**:  
   - Processes card payments for merchants.  
   - Routes transactions to card networks (Visa, Mastercard).  

4. **Issuing Bank**:  
   - Approves/declines transactions based on:  
     - Card validity.  
     - Sufficient balance.  
     - Account standing.  

---

### **System Requirements**  
- **Functional**:  
  - Move money from account A to B.  
  - Handle payment events (e.g., order placement → PSP interaction → balance updates).  
  - Maintain a **ledger** for auditing/revenue analysis.  
- **Non-Functional**:  
  - **High availability**: Tolerate component failures.  
  - **Consistency**: Avoid double payments or data mismatches.  
  - **Scalability**: Manage traffic spikes (e.g., via queues like **Kafka**).  

---

### **Communication Patterns**  
1. **Synchronous**:  
   - **Pros**: Real-time feedback (e.g., in-store payments).  
   - **Cons**: Tight coupling, cascading failures, latency sensitivity.  

2. **Asynchronous**:  
   - **Pros**:  
     - Loose coupling, fault tolerance.  
     - Buffers traffic (e.g., Kafka queues).  
   - **Use Cases**: Online payments, fraud detection, analytics.  

---

### **Challenges & Solutions**  
1. **Failures**:  
   - **Types**: Network/server errors, poison pills, functional bugs.  
   - **Tools**:  
     - **Retries**: Exponential backoff with jitter to avoid overloading.  
     - **Dead Letter Queues**: Isolate poison pills for later analysis.  

2. **Idempotency**:  
   - **Goal**: Prevent duplicate charges.  
   - **Implementation**:  
     - Unique **idempotency key** (e.g., UUID) in request headers.  
     - Database constraints to block duplicate key insertion.  

3. **Timeouts**:  
   - **Risk**: Ambiguous transaction status (e.g., charged but marked as failed).  
   - **Mitigation**: Backend retries + idempotency checks.  

4. **Fallbacks**:  
   - **Example**: Bypass fraud checks for small amounts if the service is down.  

---

### **Distributed Systems**  
- **Benefits**:  
  - Redundancy via replication.  
  - Workload distribution.  
  - Failure tolerance.  
- **Challenges**:  
  - Network partitions.  
  - Replication lag → consistency trade-offs (e.g., read/write consistency levels).  

---

### **Security Measures**  
1. **Encryption**:  
   - **At rest**: Disk/database encryption.  
   - **In transit**: VPNs, TLS, HTTPS.  

2. **Access Control**:  
   - Two-factor authentication.  
   - Regular software updates/patches.  

3. **Data Integrity**:  
   - Cryptographic checksums to detect unauthorized changes.  
   - Prioritize monitoring sensitive data (e.g., credentials, encryption keys).  

4. **Password Policies**:  
   - Enforce complex, unique passwords to thwart rainbow table attacks.  

---

### **Monitoring & Compliance**  
- **Compliance**: Adhere to regional regulations (varies by country).  
- **Backups**: Mitigate ransomware risks.  
- **Alerts**: Trigger on data integrity breaches or suspicious activity.