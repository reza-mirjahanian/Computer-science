### **System Design Trade-offs Breakdown**  

#### **1. Scaling: Vertical vs. Horizontal**  
- **Vertical Scaling:**  
  - Add more resources (CPU, RAM) to existing servers.  
  - **Pros:** No code changes, simpler to manage.  
  - **Cons:** Physical limits, expensive upgrades.  

- **Horizontal Scaling:**  
  - Add more servers to distribute load.  
  - **Pros:** Unlimited scaling, better fault tolerance.  
  - **Cons:** Complexity (load balancing, data consistency, distributed systems).  

**Trade-off:** Simplicity (vertical) vs. Scalability (horizontal).  

---  

#### **2. API Design: REST vs. GraphQL**  
- **REST APIs:**  
  - Mature, HTTP-based, resource-oriented.  
  - **Pros:** Simple CRUD, good for public APIs.  
  - **Cons:** Over-fetching, multiple round trips, endpoint sprawl.  

- **GraphQL:**  
  - Clients request only needed data in one query.  
  - **Pros:** Flexible, efficient for complex UIs.  
  - **Cons:** Complex server logic, nested query risks, security concerns.  

**Trade-off:** REST for simplicity vs. GraphQL for flexibility.  

---  

#### **3. Stateful vs. Stateless Design**  
- **Stateless:**  
  - Scales well (e.g., RESTful web services).  
  - No server-side session tracking.  

- **Stateful:**  
  - Required for real-time apps (e.g., game servers, chat, trading platforms).  
  - Tracks live data (player positions, WebSocket connections).  

**Modern Approach:** Hybrid (stateless services + stateful components for real-time features).  

---  

#### **4. Caching Strategies**  
- **Read-Through Cache:**  
  - Fetches from DB if cache miss.  
  - **Pros:** Simple.  
  - **Cons:** Stale data (e.g., User B sees outdated info after User A updates).  

- **Write-Through Cache:**  
  - Updates cache + DB simultaneously.  
  - **Pros:** Fresh data.  
  - **Cons:** Higher write latency.  

**Trade-off:** Data freshness vs. write performance.  

---  

#### **5. Synchronous vs. Asynchronous Processing**  
- **Synchronous:**  
  - Immediate response (e.g., simple CRUD).  
  - **Pros:** Predictable.  
  - **Cons:** Blocks users during long tasks.  

- **Asynchronous:**  
  - Background processing (e.g., report generation, batch emails).  
  - **Pros:** Better UX for slow operations.  
  - **Cons:** Complex (message queues, retries, status tracking).  

**Hybrid Approach:** Quick ops synchronous, heavy tasks asynchronous.  

---  

