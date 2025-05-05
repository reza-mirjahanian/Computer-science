# SQL Injection Attack  
**Definition**: Exploiting input fields on a web page to inject malicious SQL code into a database due to improper input sanitization.  
- **Attack Methods**:  
  - Alter user privileges.  
  - Read/drop databases.  
  - Execute unauthorized queries (e.g., `SELECT * FROM users;`).  
- **Example**:  
  - A query like `SELECT * FROM users WHERE username = '[input]'` becomes vulnerable if an attacker inputs `'; DROP TABLE users;--`.  
- **Prevention**:  
  - **Sanitize inputs**.  
  - Use **SQL parameters** (e.g., `$1` placeholders) to separate executable code from data.  

---

# HTTP Caching  
**Purpose**: Reduce server load and improve page load speed by reusing cached resources.  
- **Mechanism**:  
  - **Cache-Control Headers** (e.g., `public, max-age=31536000`) specify caching rules.  
  - **ETag**: Unique hash for resource validation.  
- **Process**:  
  1. Client requests a resource (e.g., CSS file).  
  2. Server responds with headers (`Cache-Control`, `ETag`).  
  3. On subsequent requests:  
     - If `max-age` expires, client sends `ETag` to check freshness.  
     - Server responds with `304 Not Modified` (unchanged) or new resource.  
- **Advantages**:  
  - Reduced bandwidth usage.  
  - Faster client-side rendering.  
- **Tools**:  
  - CDNs (e.g., AWS CloudFront) handle caching automatically.  

---

# Scaling a SQL Database  
**Methods**:  
1. **Read-Only Replicas**:  
   - **Use Case**: Read-heavy applications (e.g., social media).  
   - **Setup**:  
     - Write to a **primary database**.  
     - Replicate data to **read replicas**.  
   - **Advantages**:  
     - Offloads read operations.  
     - Easy to implement (e.g., AWS RDS).  

2. **Sharding**:  
   - **Use Case**: High write volume.  
   - **Setup**:  
     - Split data across databases using a **hash function** (e.g., user ID).  
   - **Challenges**:  
     - Complex joins across shards.  
     - Requires **consistent hashing** for data distribution.  

3. **Vertical vs. Horizontal Scaling**:  
   - **Vertical**: Upgrade server hardware (limited by cost and downtime).  
   - **Horizontal**: Add more servers (improves scalability and availability).  

---

# Deployment Types  
1. **In-Place Deployment**:  
   - **Process**: Replace code on a live server and restart.  
   - **Pros**: Simple setup.  
   - **Cons**: Downtime, difficult rollback.  

2. **Rolling Deployment**:  
   - **Process**: Gradually update instances in a fleet.  
   - **Pros**: Minimal downtime.  
   - **Cons**: Mixed versions during rollout.  

3. **Blue-Green Deployment**:  
   - **Process**: Duplicate infrastructure, switch traffic post-testing.  
   - **Pros**: Zero downtime, easy rollback.  
   - **Cons**: High infrastructure cost.  

4. **Canary Deployment**:  
   - **Process**: Redirect a subset of traffic (e.g., 2%) to new version.  
   - **Pros**: Test changes safely.  
   - **Cons**: Requires advanced load balancing.  

---

# How the Internet Works  
**Step-by-Step**:  
1. **DNS Lookup**:  
   - Browser queries DNS servers to resolve domain (e.g., `theseniordev.com`) to an IP address.  

2. **TCP/IP Connection**:  
   - Establishes a TCP connection with the server via IP.  

3. **HTTP Request**:  
   - Browser sends `GET` request with headers (e.g., `Accept: text/html`).  

4. **Server Response**:  
   - Returns `200 OK` with HTML content and headers (e.g., `Content-Type: text/html`).  

5. **Rendering**:  
   - Browser parses HTML/CSS/JS, builds DOM/CSSOM, and renders the page.