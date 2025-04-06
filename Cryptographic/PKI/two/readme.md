### **PKI Concepts**

---

#### **Overview of PKI**
- **Definition**: PKI (Public Key Infrastructure) is a two-key asymmetric cryptosystem.
- **Purpose**:
  - Ensures **information confidentiality** through **strong encryption**.
  - Provides **authentication** via **digital signatures** and **digital certificates**.

---

#### **Key Pair System**
- **Structure**:
  - **Public Key**: Can be shared openly; used for encryption.
  - **Private Key**: Kept secret; used for decryption.
- **Relationship**:
  - Keys are **statistically unique** and **mathematically related** but **distinct**.

---

#### **Example Scenario: Bob and Mary**
1. **Step 1**: 
   - Mary has a key pair: **public key** and **private key**.
2. **Step 2**: 
   - Bob requests Mary’s **public key**.
   - Since the **public key** is not a secret, Mary sends it to Bob.
3. **Step 3**: 
   - Bob uses Mary’s **public key** to encrypt the message.
   - Bob sends the encrypted message to Mary.
4. **Step 4**: 
   - Mary decrypts the message using her **private key**.
5. **Result**:
   - Confidentiality is ensured as only Mary’s **private key** can decrypt the message.

---

#### **Potential Issue: Authenticity of Public Key**
- **Problem**: How does Bob know the received **public key** truly belongs to Mary?
- **Solution**: Use **digital certificates**.

---

#### **Digital Certificates**
- **Definition**: Data packages that associate a **public key** with its respective entity (e.g., Mary).
- **Issued By**: 
  - A trusted entity called a **Certificate Authority (CA)**.
  - CA confirms the identity of the entity before issuing the certificate.
- **Protection**: Digital certificates are secured using **asymmetric cryptography**.

---

#### **Role of Certificate Authority (CA)**
- **Functions**:
  - Issues, revokes, and distributes **digital certificates**.
- **Trust**:
  - Often a **third-party organization** trusted by all participants.
  - Can be an **in-house CA** for organizations (e.g., Secure Metrics PKI appliance).

---

#### **Enhanced Bob and Mary Scenario with PKI**
1. **Step 1**: 
   - Mary obtains a **digital certificate** from a trusted CA.
2. **Step 2**: 
   - Bob retrieves Mary’s **digital certificate** from:
     - The CA.
     - Mary herself.
     - Any other entity holding the certificate.
3. **Step 3**: 
   - Bob validates Mary’s **digital certificate** to ensure the **public key** belongs to her.
4. **Step 4**: 
   - Bob encrypts the message using Mary’s **public key**, and Mary decrypts it using her **private key**.

---

#### **Contents of a Digital Certificate**
- **Information Included**:
  - Mary’s **public key**.
  - Mary’s **particulars**.
  - The CA’s **particulars**.
  - Cryptographic data for identity verification.

---

#### **Legislation Supporting PKI**
- **Examples**:
  - Digital Signatures Act.
  - Electronic Transactions Act.
  - Electronic Commerce Act.
- **Country-Specific**: PKI legislation varies by region.

---

#### **Validation of Digital Certificates**
- **Process**: Ensures the authenticity of the **public key**.
- **Details**: Reserved for further explanation.

---

#### **Benefits of PKI Appliance**
- **Example**: Secure Metrics PKI in a box.
- **Advantages**:
  - Simplifies CA management for organizations.
  - Reduces costs.
  - Enhances ease of implementation.

