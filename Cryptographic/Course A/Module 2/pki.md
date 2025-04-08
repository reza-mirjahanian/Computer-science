### **Public Key Infrastructure (PKI) **  

#### **1. Introduction to PKI**  
- **Definition**: PKI stands for **Public Key Infrastructure**.  
- **Purpose**:  
  - Ensures **secure communication** over untrusted networks (e.g., the internet).  
  - Provides **authentication, encryption, and data integrity**.  
  - Critical for **HTTPS, SSL/TLS, VPNs, and digital signatures**.  

#### **2. Why PKI Was Introduced**  
- **Problem Before PKI (Man-in-the-Middle Attack)**:  
  - **Scenario**: Two systems (A and B) communicate using **asymmetric encryption**.  
    - A sends encrypted data to B.  
    - B sends its **public key** to A for encrypting the session key.  
  - **Attack**:  
    - A hacker intercepts and **replaces B’s public key** with their own.  
    - A encrypts the session key with the **hacker’s public key**.  
    - Hacker decrypts the session key using their **private key** and accesses the data.  
  - **Limitation**: No **trusted verification** of public keys.  

#### **3. PKI Solution: Certificate Authority (CA)**  
- **Role of CA**:  
  - Acts as a **trusted third party**.  
  - Issues **digital certificates** that **bind public keys to identities** (e.g., websites, users).  
  - Signs certificates using its **private key** to ensure authenticity.  

- **How PKI Works**:  
  1. **Certificate Request**:  
     - B sends its **public key** to the **CA**.  
  2. **Certificate Generation**:  
     - CA **creates a certificate** containing:  
       - B’s public key.  
       - **Hash** (for integrity).  
       - **Digital signature** (encrypted with CA’s private key).  
  3. **Certificate Verification**:  
     - A receives B’s certificate.  
     - Uses **CA’s public key** to verify the signature.  
     - Extracts B’s **authentic public key** from the certificate.  
  4. **Secure Communication**:  
     - A encrypts a **session key** with B’s verified public key.  
     - B decrypts it with its **private key**, enabling encrypted data transfer.  

#### **4. PKI Components**  
1. **Certificate Authority (CA)**  
   - **Primary Role**: Issues, revokes, and manages certificates.  
   - **Trust Anchor**: Clients must **trust the CA’s public key** (pre-installed in browsers/OS).  
2. **Registration Authority (RA)**  
   - **Role**: Acts as an intermediary between users and CA.  
   - **Functions**:  
     - Validates identity before forwarding certificate requests to CA.  
     - **Example**: Like Pearson VUE (for CompTIA exams) verifies candidates before certification.  

#### **5. Certificate Revocation**  
- **Why Revoke?**  
  - **Compromised private key**.  
  - **Incorrect issuance** (e.g., wrong entity).  
  - **Change in subject** (e.g., domain name change).  
  - **Security policy updates**.  

- **Revocation Methods**:  
  | Method | How It Works | Pros & Cons |  
  |--------|-------------|-------------|  
  | **CRL (Certificate Revocation List)** | Offline list of revoked certificates (by serial number). | **✅ Reliable** but **❌ Slow** (requires periodic updates). |  
  | **OCSP (Online Certificate Status Protocol)** | Real-time validation by querying CA. | **✅ Faster** but **❌ Requires internet**. |  

#### **6. Digital Certificate Standards**  
- **X.509**: Most common standard (used in SSL/TLS).  
- **PKCS (Public Key Cryptography Standards)**: Used in labs (e.g., RSA).  
- **PIV (Personal Identity Verification)**: U.S. government smart cards.  
- **CVC (Card Verifiable Certificates)**: EU e-passports.  

#### **7. PKI Use Cases**  
- **HTTPS (SSL/TLS)**: Secure web browsing (e.g., Gmail).  
- **Email Encryption (S/MIME)**: Digitally signed/encrypted emails.  
- **VPN Authentication**: Certificates replace passwords.  
- **Code Signing**: Ensures software integrity.  

#### **8. Example: SSL/TLS Handshake with PKI**  
1. **Client Request**: `https://gmail.com`.  
2. **Server Response**: Sends its **certificate** (signed by CA).  
3. **Client Verification**:  
   - Checks **CA signature** (using pre-installed CA public key).  
   - Validates **revocation status** (via CRL/OCSP).  
4. **Session Key Exchange**:  
   - Client generates a **session key**, encrypts it with server’s public key.  
   - Server decrypts it with its **private key**.  
5. **Secure Communication**: All data encrypted with the session key.  

#### **9. Common PKI Questions**  
**Q1: What is the primary objective of PKI?**  
- **Answer**: Facilitate secure communication & authentication.  

**Q2: What is OCSP used for?**  
- **Answer**: Real-time certificate revocation checks.  

**Q3: What does a CRL contain?**  
- **Answer**: List of revoked certificates (before expiration).  

**Scenario**: An employee receives an email with a **revoked certificate**.  
- **Action**: Report to IT security (revoked certs indicate potential compromise).  

