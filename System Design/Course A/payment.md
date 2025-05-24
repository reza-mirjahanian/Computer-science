# Implementing a Reliable and Scalable Payment System

## E-commerce and Payment Systems
* **E-commerce:** Trading of goods and services on the internet in exchange for monetary payment.
* **Payment System:** Enables every e-commerce transaction, operating in the background.
* **Challenges in Implementation:**
    * *Reliability and correctness* are critical.
    * Successful businesses generate a lot of payment requests as they scale.
    * *Availability* is essential; downtime means lost revenue.

## Global Payment System Process Overview
1.  A customer places an order on a merchant website.
2.  The customer provides payment information.
3.  The merchant sends the customer to a payment form page (usually provided by a **Payment Gateway**) to enter payment details.
    * **Payment Gateway:**
        * Must manage compliance rules (e.g., *PCI DSS*, *GDPR*).
        * Can forward requests to verification services for risk and fraud prevention.
        * Main function: Validate financial credentials and transfer them to a merchant bank account.
4.  Cardholder information is transmitted to the **Acquiring Bank** (the bank that processes card payments for the merchant).
    * **Payment Service Provider (PSP):**
        * A broader term for third-party companies assisting businesses with safe and secure payment facilitation.
        * Offers services like risk management, reconciliation tools, and sometimes order management.
        * PSPs can also be the acquiring bank, but not necessarily.
5.  The Acquiring Bank captures transaction information, performs basic validation, and routes requests via appropriate **card networks** to the cardholder's **Issuing Bank**.
6.  The customer's Issuing Bank receives the transaction information and responds by approving or declining. It checks:
    * Transaction information validity.
    * Sufficient cardholder balance.
    * Account is in good standing.
7.  The transaction status follows the same route back to the merchant and is also displayed to the client.

## Building a Payment System: Approaches
* **Initial Steps:**
    * Establish system requirements.
    * Ask refining questions to clarify functional needs, scope, and non-functional requirements.
* **Two Main Ways to Build:**
    1.  **Using a PSP (e.g., Stripe, PayPal):**
        * Most common for online stores or platforms requiring payments.
        * PSP moves money from the buyer's account to the merchant's account.
        * *Advantages:*
            * PSP stores and processes card data, reducing security implementation burden.
            * Simplifies compliance.
        * *Considerations:* Still need to manage the logic to process transactions.
    2.  **Direct Connection to Banks or Card Schemes (e.g., Visa, Mastercard):**
        * Uncommon and difficult to establish.
        * Requires compliance with numerous standards and regulations for security and user protection against identity theft.
        * Compliance is complex and varies by country.
        * Most companies, even large ones, use payment gateways.
* **General Needs:**
    * Plan for transaction failures.
    * Perform reconciliation to fix inconsistencies.

## System Requirements and Focus
* **Functional Level:** Move money from account A to account B.
* **Challenges:** Ensuring reliability, especially with unknown situations; a small slip can cause significant revenue loss.
* **Video Focus:** Technical concepts applicable to most systems and handling large throughput of payment requests.

## Core Features for an Online Store Payment System (with PSP)
1.  User clicks "place order" -> payment event is generated and sent to the payment service.
2.  **Payment Service (Coordinator Service):**
    * Stores the payment event in the database.
    * Calls an external PSP to process the card payment (providing monetary amount and currency).
3.  **Payment Page:**
    * User sees this page to input payment details.
    * Two ways to provide this page:
        * *PSP-provided form page (common):* Recommended as it avoids self-managing sensitive payment information and complex compliance.
        * *In-house built form page (uncommon):* High effort, requires storing sensitive data and going through compliance; usually not justified unless for very large companies.
4.  **PSP Function:** Sends card details to banks or card schemes.
5.  Post-successful PSP processing, the coordinator service updates the merchant's **Wallet** (to track account balance).
6.  Updated wallet balance information is stored in the database.
7.  Payment service updates the **Ledger** (logs all financial transactions record by record).
    * Used for post-payment analysis (e.g., calculating total revenue, auditing).
8.  Ledger service appends new information to a database.

## Communication Patterns: Synchronous vs. Asynchronous
* **Synchronous Communication:** One service sends a request and waits for the response before proceeding.
* **Asynchronous Communication:** A service sends a request and continues its execution, polling for the response or being notified.
* **Considerations for Choice:**
    * Any involved service can fail.
    * Communication is mostly over the network, which can be slow (congestion) or unreliable (requests lost).
* **Synchronous Drawbacks:**
    * Not tolerant to failure or high latencies.
    * Doesn't isolate failure, reducing overall system availability (risk of cascading failure).
    * If a service is slow, the caller service is blocked.
    * Results from tight coupling between services.
    * *Use Case:* Only if immediate response is essential (e.g., physical store payments).
* **Asynchronous Advantages:**
    * Preferred in most cases due to loose coupling.
    * Better for large-scale payment systems with complex logic and many third-party dependencies.
    * Easier to handle uneven traffic and spikes using **persistent queues (e.g., Kafka)** as buffers.
        * Requests stored and processed at a constant pace.
        * Allows time to scale resources (e.g., spin up new servers).
    * Kafka is used by 7 out of 10 banks and financial companies.
    * Suitable for online payments, fraud detection, and analytics because it's tolerant to failure and high latencies.

## Common Issues in Payment Systems
* **System Failures:** Network and server failures.
* **Poison Pill Errors:** An inbound message cannot be processed or consumed.
* **Functional Bugs:** No technical errors, but the results are invalid.

## Tools for Building Reliable Payment Systems

### Messaging Queues for Guaranteed Delivery (e.g., Apache Kafka)
* **Purpose:** Ensure transaction completion and prevent message loss.
* **Mechanism:**
    * Create an event in Kafka for any order placed or paid.
    * Kafka persists communication messages, ensuring they aren't lost.
    * Payment operation completes successfully only when the event is safely stored in the message queue.
    * Kafka's simplicity in storing messages contributes to its high availability.
    * Messages are consumed individually by interested services.
    * Consumer marks message as "seen" or "consumed" *only after* successful processing and storage in its database.
    * Ensures messages are stored on disk (likely replicated).

### Retry Mechanisms
* **Purpose:** Handle temporary failures (e.g., unstable network connection).
* **Considerations:**
    * Number of retries.
    * Appropriate time intervals between retries.
* **Time Interval Strategies:**
    * *Immediate Retry:* Unlikely to succeed, can waste resources and overload the system.
    * *Fixed Intervals:* Retries at set periods.
    * *Incremental Intervals:* Increases wait time with each retry.
    * ***Exponential Backoff (Recommended):*** Doubles the waiting time between retries after each attempt. This gives the system a break to recover.
    * ***Jitter:*** Adding randomness to each client's wait time when multiple clients depend on one service. This spaces out retry requests, giving the troubled server breathing room.

### Timeouts
* **Goal:** Avoid unbounded waiting times for a response.
* **Mechanism:** If time to respond is too high, the operation is aborted and treated as failed.
* **Challenges:**
    * Determining the request's actual status on timeout (e.g., was payment successful but response lost? Is request still in progress?).
    * If marked as failed, customer might retry, potentially leading to double charges (addressed by *idempotency*).
* **Setting Timeout Value:**
    * High enough to allow slower (valid) responses.
    * Low enough to stop waiting for a response that will never arrive.
    * Depends on the specific endpoint.

### Fallbacks
* **Purpose:** Allow a service to continue execution even if requests to a dependent service fail.
* **Example:** If a fraud check service returns an error during payment:
    * Instead of aborting, use a fallback value or a simple business rule (e.g., if the amount is small, let the transaction proceed).
* **Trade-off:** Between risk and keeping customers happy.

### Handling Persistent Errors and Poison Pills
* For failures persisting for minutes or hours:
    * **Non-retrievable errors (Poison Pills):** Incompatible messages that will always fail.
        * Isolate by saving them to a **Dead Letter Queue (DLQ)** for later debugging.
    * **Retrievable errors (service down temporarily):**
        * Store failed transactions in a persistent queue.
        * Process them later when the failed service recovers.

### Idempotency for Safe Retries
* **Goal:** Allow safe retrying of operations without charging the customer twice or causing other unintended side effects.
* **Idempotent Operation:** An operation that has no additional effect if called more than once with the same input parameters.
* **Scenario for Use:** Customer makes a payment, PSP processes it, but the success response fails to reach the payment system. The user might retry.
* **Mechanism:**
    * Use an **idempotency key**: A unique value (e.g., UUID, often the payment order ID) generated by the client, usually expiring after a certain period.
    * The key is added to the HTTP header. (Stripe and PayPal use this).
* **Server-Side Implementation:**
    * When the server receives a payment request with an idempotency key:
        * It tries to insert a row with this key into a database table (using a unique key constraint).
        * *Successful insertion:* New request, process it.
        * *Failed insertion (duplicate key):* It's a retry. Return the latest status of the previous request with that key.
    * If multiple concurrent requests with the same idempotency key are detected, only one is processed; others receive a `429 Too Many Requests` status.
* **Benefit:** Ensures "exactly-once" processing guarantee.

## Distributed Systems for Scalability and Reliability
* **When to Use:** When a single machine limits storage or processing capacity.
* **Benefits:**
    * **Redundancy:** Multiple copies of data and processes (via replication) improve reliability by providing backups.
    * **Workload Distribution:** Spreading tasks across multiple machines reduces the risk of overwhelming a single component.
    * **Fault Tolerance:** System can continue functioning even if some components fail.
    * **Scalability:** Easily scale up or down by adding or removing components to handle varying workloads.
* **Challenges:**
    * Communication failures between nodes can cause *data inconsistency*.
    * *Replication lag* can lead to inconsistent data between primary and replica databases.
    * Requires awareness of the *consistency level* used for data reads and writes.

## Data Protection Strategies
* **Encryption:**
    * **Data at Rest:** Convert data into a secure format unreadable without a key (using software for disk or database encryption).
    * **Data in Transit:**
        * *VPN:* Secures and encrypts connections between a device and a network.
        * *TLS (Transport Layer Security):* Provides confidentiality, data integrity, and authentication for data transmitted between parties (client/server). *SSL is deprecated*.
        * *HTTPS:* Uses HTTP over TLS/SSL.
* **Access Control:** Restrict data access only to authorized users (e.g., using two-factor authentication).
* **Regular Updates:** Keep software, libraries, and operating systems up-to-date with the latest security patches.
* **Data Backup:** Ensure data can be recovered in case of loss, damage, or ransomware attacks.
* **Strong Passwords:**
    * Users should use long, complex, and uncommon passwords.
    * This helps prevent attackers from guessing passwords, even if they have an encrypted version, by using tools like *rainbow tables* (pre-computed tables of reversed password hashes).

## Monitoring Data Integrity
* **Purpose:** Secure business data against known and unknown threats by checking for unauthorized changes to vulnerable data.
* **Process:**
    1.  Assess files of databases and file systems.
    2.  Generate a cryptographic checksum as a baseline.
    3.  Regularly recalculate the checksum of the same resources.
    4.  Compare with the baseline; if changes are detected, generate a security alert.
* **Benefits:** Can detect malware within the OS or applications.
* **Considerations:**
    * Can be resource-intensive, especially with large data amounts.
    * Crucial to focus monitoring on data and files most vulnerable to cyberattacks:
        * User credentials, privileges, and identities.
        * Encryption key stores.
        * Operating system files, configuration files, and application files.

## Recap: Key Tools for Reliability and Fault Tolerance
* **Redundancy:** Enables resilience during internal system failures.
* **Patterns for Payment Guarantee:** Using Kafka capabilities to persist messages so they are not lost.
* **Strategies for Retry, Timeouts, and Fallbacks:** Make the system robust and predictable.
* **Messaging Queues:** Avoid overloading the system.
* **Idempotent Message Handling:** Allows clients to retry requests safely, preventing inconsistent data states or double payments.