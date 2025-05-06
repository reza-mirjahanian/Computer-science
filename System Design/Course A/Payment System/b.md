# **Designing a Global Payment System**

## **Key Components Explained**

* **Payment Gateway:**
    * Acts as a **bridge** between the user's device and the payment processor.
    * *Securely transmits* payment data.
    * Handles **encryption** and **authorization** of the transaction.
    * *Example:* Encrypts card details entered on a website and sends them for approval.
* **Payment Processor:**
    * The **engine** behind the scenes.
    * *Communicates* with the user's bank or card network (e.g., Visa, Mastercard).
    * **Validates** payment information, confirms funds, and completes the transaction.
    * Ensures money flows from the user's bank to the merchant's account.

## **System Requirements**

### **Functional Requirements:**

* **Global Accessibility:** Users worldwide.
* **User Types:** Merchants and Consumers.
* **Payment Types:**
    * One-time payments
    * Recurring payments
    * Refunds
    * Dispute resolutions
* **Currency Support:** Multiple currencies with real-time exchange rates.
* **Scalability:** Handle thousands of transactions per second (similar to PayPal).
* **Compliance:** Meet international and regional regulations (PCI DSS, KYC).

### **Non-Functional Requirements:**

* **High Reliability**
* **Scalability**
* **Low Latency**
* **Top-notch Security**

## **System Goals**

* **High Availability:** Aim for *99.99%* uptime.
* **Scalability:** Efficiently handle peak loads and grow with user demand.
* **Security:** Implement robust measures to protect data and transactions.
* **Performance:** Ensure quick and seamless transaction processing.
* **Extensibility:** Easily add new features, currencies, or payment methods.

## **System Architecture Layers**

1.  **Client Layer:** User interaction point (Web apps, Mobile apps, APIs). Sends requests to the backend.
2.  **Service Layer:** The core system with microservices handling business logic (verification, processing, external interactions). Uses message brokers (e.g., Kafka) for asynchronous communication.
3.  **Data Layer:** Stores all data.
    * *Relational Databases:* Transactional data.
    * *NoSQL Systems:* Session data, logs.
    * *Caches (e.g., Redis):* Frequently accessed data for speed.

## **Client Layer Details**

* **Web Applications/Portals:** User login, account management, payments.
* **Mobile Apps (iOS/Android):** Customer payment initiation, balance checks.
* **API Consumers:** Third-party systems (e.g., merchants integrating payments).

## **Service Layer Details**

* **Core:** Contains business logic and microservices.
* **Payment Gateway Role:**
    * Implemented using components like **Amazon API Gateway**, **Payment Service**, and parts of **Fraud Detection Service**.
    * Handles secure data transmission, interaction with external processors/banks.
    * Manages rate limiting, request routing, user authentication.
* **Payment Processor Role:**
    * Implemented by **Payment Service**, **Account Service**, and **Transaction Service**.
    * Verifies balances, initiates transactions, interacts with banks/external gateways.
    * Handles currency conversions, records transactions, updates balances.
    * Ensures *ACID compliance* and manages distributed transactions.
* **Microservices Architecture:**
    1.  **User Service:** Authentication, profile management.
    2.  **Payment Service:** Transaction processing, external communications (gateways, banks).
    3.  **Account Service:** Tracks user balances (multi-currency), ensures transaction integrity.
    4.  **Fraud Detection Service:** Real-time monitoring, uses ML models to flag fraud.
    5.  **Notification Service:** Sends transaction updates to users.
* **Communication:** Asynchronous via message queues (e.g., Kafka, RabbitMQ) for reliability and decoupling.

## **Data Layer Details**

* **Storage Types:**
    * *Relational (e.g., PostgreSQL):* Transactional data (requires ACID compliance).
    * *NoSQL (e.g., Cassandra, MongoDB):* Non-critical data (logs, session info).
* **Database per Service Pattern:** Each microservice owns its database for loose coupling.
    * **User Service DB:** User profiles, auth details, KYC status.
    * **Account Service DB:** User balances, currencies (ACID compliant).
    * **Payment Service DB:** Payment request metadata (initiator, merchant, currency, status), Currency Exchange Rates Table.
    * **Transaction Service DB:** All detailed transaction records (durability, traceability).
    * **Fraud Detection Service DB:** Historical fraud data, scores, user behavior.
    * **Notification Service DB:** Notification logs (status of sent messages).
* **Data Management Techniques:**
    * **Sharding:** Horizontal partitioning (by user ID, region).
    * **Read Replicas:** Improve read performance, provide redundancy.
    * **Indexing:** Speed up queries on frequently accessed fields.
* **NoSQL Data Usage:**
    * **Session Data:** User login tokens, timestamps, expiry.
    * **Event Logs:** User/system actions (logins, payments) requiring high write throughput.

## **Asynchronous Communication: Kafka Topics**

* **Balance Check Queue:** Payment Service -> Account Service (verify balance).
* **Balance Response Queue:** Account Service -> Payment Service (verification result).
* **Transaction Recording Queue:** Payment Service -> Transaction Service (persist transaction).
* **Account Update Queue:** Payment Service -> Account Service (update balance).
* **Notification Queue:** Payment Service -> Notification Service (send confirmation).
* **Fraud Detection Queue:** Payment Service -> Fraud Detection Service (send data for analysis).
* **Payment Gateway Queue:** Payment Service -> External Gateways (async communication).
* **Gateway Response Queue:** External Gateways -> Payment Service (response).

## **Database Ownership**

* **User Service:** Owns User DB.
* **Account Service:** Owns Account DB.
* **Payment Service:** Owns Payment Metadata DB.
* **Transaction Service:** Owns Transaction DB.
* **Fraud Detection Service:** Owns Fraud Detection DB.
* **Notification Service:** Owns Notification Log DB.

## **Transaction Flow Examples**

### **1. User Login Process**

1.  User sends credentials via Client Layer -> API Gateway.
2.  Load Balancer distributes request to an API Gateway instance.
3.  API Gateway forwards request to User Service.
4.  User Service checks **Redis cache** for user profile.
5.  If not cached, fetches from User Service DB.
6.  Validates credentials.
7.  If valid:
    * Generates **JWT token**.
    * Stores session data in **Redis**.
    * Sends JWT token back to Client via API Gateway.

### **2. Payment Initiation Process**

1.  Authenticated user sends payment details via Client -> API Gateway.
2.  API Gateway verifies JWT token, forwards request to **Payment Service**.
3.  Payment Service checks **Redis cache** for user balance.
4.  If balance insufficient/not cached: publishes message to **Balance Check Queue (Kafka)** for Account Service.
5.  **Account Service** listens, checks balance (Redis cache or PostgreSQL DB).
6.  If sufficient funds: locks funds (PostgreSQL transaction), updates cache.
7.  Account Service publishes result to **Balance Response Queue (Kafka)**.
8.  Payment Service listens:
    * *If insufficient:* Returns error to user.
    * *If sufficient:* Locks amount, proceeds.
9.  **(External Interaction)** Payment Service publishes request to **Payment Gateway Queue (Kafka)** for external processing (e.g., Visa, Mastercard).
    * *Retries:* Uses exponential backoff if external gateway fails.
    * *Circuit Breaker:* Prevents overloading failing external service.
10. External gateway sends response via **Gateway Response Queue (Kafka)**.
11. Payment Service consumes response (Approved/Declined).
12. **(Fraud Check)** Simultaneously, Payment Service sends data to **Fraud Detection Queue (Kafka)**.
13. Fraud Detection Service analyzes risk. If high, publishes message to potentially halt/flag transaction.
14. **(Recording)** If approved & not flagged: Payment Service publishes message to **Transaction Recording Queue (Kafka)**.
    * Sends **Idempotency Key** to prevent duplicate transactions on retry.
15. **Transaction Service** consumes message, checks idempotency key, persists transaction (ACID compliant).
16. **(Balance Update)** Payment Service publishes message to **Account Update Queue (Kafka)**.
17. **Account Service** consumes message, deducts amount from balance (PostgreSQL), updates Redis cache.
18. **(Notification)** Payment Service publishes message to **Notification Queue (Kafka)**.
19. **Notification Service** consumes message, sends confirmation (Email, SMS, Push) asynchronously.
20. Payment Service sends final response (success/failure) with details (ID, amount, timestamp) back to the user via API Gateway.
    * *Asynchronous nature:* Core transaction response is fast; notifications, fraud flagging occur in the background.

## **Resilience and Performance Patterns**

* **Circuit Breaker:** For external calls (Payment Gateways, Banks). Prevents cascading failures.
* **Retry Pattern:** For idempotent operations (external interactions) with exponential backoff.
* **Idempotency Key:** Prevents duplicate operations (e.g., charges) during retries.
* **Saga Pattern:** Manages distributed transactions across microservices (Payment, Account, Fraud) ensuring eventual consistency via compensating actions (e.g., refunds).

## **Security Considerations**

* **Encryption:**
    * *Data in Transit:* TLS encryption for all communication.
    * *Data at Rest:* Encrypt sensitive data in databases.
* **Authentication & Authorization:**
    * *Multi-Factor Authentication (MFA):* For users.
    * *Role-Based Access Control (RBAC):* Limit internal system access.
* **Fraud Detection:** Real-time monitoring and ML models.

## **Scaling Strategies (Handling High Volume)**

* **Microservices Architecture:** Scale individual services horizontally based on load.
* **Load Balancing:** Distribute traffic across instances (including Global Load Balancers for geographic routing).
* **Caching (e.g., Redis):** Reduce database load for frequently accessed data.
* **Asynchronous Processing (Message Queues - Kafka):** Offload non-critical tasks (notifications, some checks) to prevent blocking user requests.
* **Database Optimization:**
    * *Read Replicas:* Handle read queries, reduce load on primary DB.
    * *Partitioning/Sharding:* Manage large datasets effectively.