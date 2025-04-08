
---

### **Public Key Cryptography Overview**
- **Purpose**: Enables secure communication between two parties without a pre-shared key.
- **Core Concept**: Uses asymmetric keys (public and private keys) for encryption/decryption.
- **Problem Solved**: Secure communication with entities you’ve never interacted with before (e.g., web servers).

---

### **Symmetric vs. Asymmetric Encryption**
| Feature               | Symmetric Encryption                     | Asymmetric Encryption                   |
|-----------------------|------------------------------------------|-----------------------------------------|
| **Key Type**          | Single shared key (pre-shared)           | Public/private key pair                 |
| **Use Case**          | Fast, bulk encryption                    | Key exchange, authentication, signatures|
| **Security**          | Requires secure key distribution         | No need for pre-shared keys             |
| **Performance**       | Efficient (fewer bits needed)            | Slower (more bits for same security)    |

---

### **Public Key Cryptography Use Cases**
1. **Key Exchange** (e.g., Diffie-Hellman)
2. **Encryption/Decryption** (e.g., RSA)
3. **Authentication & Digital Signatures**
4. **Non-Repudiation** (proof of origin)

---

### **Key Algorithms**
#### **1. Diffie-Hellman (DH)**
- **Creators**: Martin Hellman & Whitfield Diffie.
- **Function**: Key exchange (not encryption).
- **Process**:
  - Two parties agree on a public shared value (`s`).
  - Each generates a private key (`y` for user, `w` for server).
  - Public keys are derived (`ys` and `ws`) and exchanged.
  - Both compute the same shared secret (`wsy` = `yws` mathematically).
- **Limitation**: Only facilitates key exchange; encryption requires additional steps.

#### **2. RSA**
- **Creators**: Rivest, Shamir, Adleman.
- **Function**: Encryption, decryption, and authentication.
- **Process**:
  - Server has a public/private key pair.
  - Public key encrypts data; private key decrypts (confidentiality).
  - Private key encrypts; public key decrypts (authentication).
- **Advantage**: Supports multiple operations (e.g., SSL/TLS, digital signatures).

---

### **Comparison: Diffie-Hellman vs. RSA**
| Feature               | Diffie-Hellman                          | RSA                                     |
|-----------------------|-----------------------------------------|-----------------------------------------|
| **Primary Use**       | Key exchange                            | Encryption, authentication, signatures  |
| **Keys Involved**     | Two private + two public + shared value | One public + one private key            |
| **Encryption**        | No (only key exchange)                  | Yes                                     |
| **Mathematical Basis**| Discrete logarithms                    | Prime factorization                     |

---

### **Practical Applications**
- **Protocols Using These Algorithms**:
  - **IPsec**: Secure VPNs.
  - **SSL/TLS**: Secure web traffic (HTTPS).
  - **SSH**: Secure remote access.
- **Hybrid Approach**:
  - Asymmetric cryptography (e.g., RSA/DH) establishes a shared key.
  - Symmetric encryption (e.g., AES) handles bulk data transfer for efficiency.

---

### **Key Takeaways**
- **Diffie-Hellman**: Best for secure key exchange but requires additional encryption.
- **RSA**: Versatile (encryption, authentication) but computationally heavier.
- **Combined Use**: Often paired for efficiency (e.g., DH for key exchange + AES for data).