### **Public Key Infrastructure (PKI) **  

#### **1. Introduction to PKI**  
- **Purpose**:  
  - Ensures **secure communication** (e.g., banking, web transactions).  
  - Verifies **authenticity** (prevents fake websites stealing credentials).  
  - Uses **encryption, hashing, and digital certificates** for trust.  

#### **2. Core Components of PKI**  
- **Certificate Authority (CA)**:  
  - Trusted entity that **issues and signs certificates**.  
  - Examples: DigiCert, Let’s Encrypt, GlobalSign.  
- **Registration Authority (RA)**:  
  - Validates identities **before CA issues certificates**.  
  - Acts as an intermediary (e.g., verifies domain ownership).  
- **Digital Certificates**:  
  - Bind **public keys to identities** (e.g., `bankofamerica.com`).  
  - Follow **X.509 standard** (contains issuer, expiry, public key, etc.).  
- **Hashing & Encryption**:  
  - Ensures **data integrity** (e.g., SHA-256 for certificate hashing).  
  - **Asymmetric encryption** (RSA/ECC) for secure key exchange.  

#### **3. How PKI Works**  
1. **User Visits a Website**:  
   - Browser requests `https://bank.com`.  
2. **Server Sends Certificate**:  
   - Contains **public key + CA’s digital signature**.  
3. **Browser Verification**:  
   - Checks if the **CA is trusted** (pre-installed in OS/browser).  
   - Validates **certificate integrity** (no tampering).  
4. **Secure Session Established**:  
   - Browser encrypts data with the server’s **public key**.  
   - Server decrypts with its **private key**.  

#### **4. Trust Models in PKI**  
- **Hierarchical Trust**:  
  - Single **root CA** (e.g., VeriSign) signs intermediate CAs.  
- **Web of Trust**:  
  - Used in PGP/GPG—users **cross-sign certificates**.  
- **Bridge Trust**:  
  - Connects different PKI hierarchies (e.g., govt. agencies).  

#### **5. Certificate Revocation**  
- **Why Revoke?**  
  - Private key **compromised**.  
  - Certificate **issued incorrectly**.  
  - **Domain/entity changes**.  
- **Revocation Methods**:  
  - **CRL (Certificate Revocation List)**:  
    - Offline list of **revoked serial numbers** (updated periodically).  
  - **OCSP (Online Certificate Status Protocol)**:  
    - Real-time **"Is this cert valid?"** queries to CA.  

#### **6. PKI Use Cases**  
- **HTTPS/SSL/TLS**: Secure websites (🔒 padlock in browsers).  
- **Email Encryption (S/MIME)**: Digitally signed emails.  
- **VPNs**: Certificate-based authentication (e.g., OpenVPN).  
- **Code Signing**: Ensures software authenticity (e.g., Microsoft drivers).  

#### **7. Example: Avoiding Phishing with PKI**  
- **Without PKI**:  
  - Fake `bank0famerica.com` steals login credentials.  
- **With PKI**:  
  - Browser **blocks access** (certificate invalid/untrusted).  

#### **8. Key Takeaways**  
- PKI = **Trust + Encryption + Verification**.  
- **CA** = "Digital notary" guaranteeing identities.  
- **Certificates** = Digital passports for websites/users.  
- **Revocation** = Critical for security (CRL/OCSP).  

