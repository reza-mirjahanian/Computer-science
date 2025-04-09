### **Trust Models in Cybersecurity**  

#### **1. Introduction to Trust Models**  
- **Purpose**: Establish secure relationships between entities (users, devices, services).  
- **Challenge**: Balancing security with connectivity needs.  
- **Core Principle**: "Never trust, always verify" (Zero Trust).  

#### **2. Types of Trust Models**  

##### **A. Direct Trust**  
- **Definition**: One-to-one trust between two entities (e.g., user ↔ website).  
- **How It Works**:  
  - User manually installs a **digital certificate** (e.g., for `technosurge.com`).  
  - Communication is verified using **pre-shared public keys**.  
- **Pros**:  
  - High security (no intermediaries).  
- **Cons**:  
  - **Not scalable** (impractical for every website).  
  - Requires manual certificate management.  

##### **B. Third-Party Trust**  
- **Definition**: Trust delegated to a **Certificate Authority (CA)**.  
- **How It Works**:  
  - CA verifies entities (e.g., websites) and issues **signed certificates**.  
  - Users trust the CA, so they inherently trust its certificates.  
- **Example**: HTTPS (browsers trust CAs like DigiCert, Let’s Encrypt).  
- **Pros**:  
  - Scalable (global trust system).  
- **Cons**:  
  - Reliance on CA security (compromised CA = broken trust).  

##### **C. Hierarchical Trust**  
- **Definition**: Top-down trust chain (root CA → intermediate CAs → end entities).  
- **How It Works**:  
  - **Root CA** signs intermediate CAs, which issue end-user certificates.  
  - Example: Corporate PKI (internal root CA signs employee certificates).  
- **Pros**:  
  - Structured and manageable.  
- **Cons**:  
  - Single point of failure (root CA compromise).  

##### **D. Web of Trust**  
- **Definition**: Decentralized trust via **peer endorsements** (common in PGP/GPG).  
- **How It Works**:  
  - Users **cross-sign** each other’s keys (e.g., Alice trusts Bob, Bob trusts Carol → Alice indirectly trusts Carol).  
- **Example**: Open-source software signing.  
- **Pros**:  
  - No central authority.  
- **Cons**:  
  - Complexity in managing trust relationships.  

#### **3. Supporting Concepts**  

##### **Trust Anchor**  
- **Definition**: The **root entity** in a trust hierarchy (e.g., root CA).  
- **Role**:  
  - All trust flows from this anchor (e.g., pre-installed root certificates in browsers).  

##### **Chain of Trust**  
- **Definition**: Linked sequence of trusted entities (root → intermediate → end certificate).  
- **Example**:  
  - `Root CA → Intermediate CA → technosurge.com` certificate.  

#### **4. Zero Trust vs. Traditional Trust Models**  
| **Aspect**       | **Traditional Trust**                     | **Zero Trust**                              |  
|------------------|------------------------------------------|--------------------------------------------|  
| **Default Policy** | "Trust but verify"                       | **"Never trust, always verify"**           |  
| **Scope**        | External entities (e.g., websites)       | **Internal networks & users**              |  
| **Implementation** | PKI, CAs, certificates                   | **Continuous authentication** (MFA, device checks) |  
| **Example**      | HTTPS browsing                           | **Employee access to sensitive databases** |  

#### **5. Zero Trust in Practice**  
- **Core Principles**:  
  1. **Verify explicitly**: Authenticate every access request.  
  2. **Least privilege**: Grant minimal necessary permissions.  
  3. **Assume breach**: Monitor/log all activity.  
- **Tools Used**:  
  - Internal PKI (for device certificates).  
  - Multi-factor authentication (MFA).  
  - Micro-segmentation (isolate network resources).  

#### **6. Key Takeaways**  
- **Direct Trust**: Secure but impractical at scale.  
- **Third-Party/Hierarchical Trust**: Powers PKI (HTTPS, VPNs).  
- **Web of Trust**: Decentralized (used in PGP).  
- **Zero Trust**: Compliments traditional models by enforcing **continuous verification**.  

