### **Hashing, Salting, Peppering, and Key Stretching**  

#### **Scenario Setup**  
- A database contains a table with **usernames and passwords**.  
- Passwords are stored as **hashes** (not plaintext) using a hashing algorithm.  
- **Problem:** Hackers can attempt to crack these hashes.  

#### **Hacker Techniques**  
1. **Brute Force Attacks**  
   - Software tries **thousands/millions of password combinations**.  
   - Tests:  
     - Dictionary words  
     - Common substitutions  
     - Known password patterns  

2. **Rainbow Tables**  
   - Precomputed tables mapping **common passwords to their hashes**.  
   - If a user’s password is weak (e.g., "Ghouse"), the hash can be **looked up instantly**.  

---  

### **Defense Mechanisms**  

#### **1. Salting**  
- **What it does:**  
  - Adds a **random value (salt)** to the password before hashing.  
  - Ensures the same password produces **different hashes** for different users.  

- **Benefits:**  
  - **Mitigates rainbow table attacks** (precomputed tables become ineffective).  
  - Requires hackers to **recompute hashes** for each salt.  

- **Limitations:**  
  - Salts must be **stored** (usually in the same database).  
  - If the database is stolen, salts are exposed.  
  - **Does not prevent brute force attacks** (hackers can still guess passwords + salt).  

---  

#### **2. Peppering**  
- **What it does:**  
  - Adds a **secret value (pepper)** to the password before hashing.  
  - Unlike salt, pepper is **not stored in the database** (e.g., kept in code or hardware).  

- **Benefits:**  
  - **Extra security layer**—even if the database is stolen, pepper remains hidden.  
  - Makes brute force attacks **harder** (hacker must guess both password + pepper).  

- **Limitations:**  
  - **Resource-intensive** if using unique peppers per user.  
  - **Single pepper risk:** If compromised, all passwords are easier to crack.  
  - **Does not protect against internal threats** (if attacker has access to both DB and pepper).  

---  

#### **3. Key Stretching**  
- **What it does:**  
  - **Slows down hash computation** (e.g., by hashing repeatedly).  
  - Examples:  
    - Hashing **100 or 1000 times** instead of once.  
    - Using algorithms like **PBKDF2, bcrypt, or Argon2**.  

- **Benefits:**  
  - **Increases time** needed for brute force attacks.  
  - Makes **massive password guessing impractical**.  

- **Considerations:**  
  - Must balance **security vs. performance** (too many iterations slow down legitimate logins).  

---  

### **Summary of Protections**  
| **Technique**      | **Protects Against**          | **Limitations**                          |  
|---------------------|-------------------------------|------------------------------------------|  
| **Salting**         | Rainbow tables                | Salt must be stored; brute force still possible |  
| **Peppering**       | Brute force (if hidden)       | Single pepper risk; resource-heavy       |  
| **Key Stretching**  | Slows brute force attacks     | Must balance computation time            |  

Using **all three methods** provides **stronger security** against different attack vectors.