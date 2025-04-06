Understanding fundamental PKI concepts makes the technology simple.

**Key Concepts Covered:**

1.  What is a CA (Certificate Authority)?
2.  How Certificate Trust Works
3.  PKI Hierarchy
4.  Important Parts of a Certificate
5.  Certificate Revocation
6.  Common Mistakes
7.  Best Practices

---

#### 1. What is a Certificate Authority (CA)?

*   **Analogy:** The DMV (Department of Motor Vehicles).
*   **Function:** The central component of PKI that issues credentials (certificates).
*   **Types:**
    *   *Public CAs:* Issue publicly trusted identities (like a driver's license).
    *   *Private CAs:* Issue internal identities for specific organizations (like a school ID or work badge for humans or computers). Needed because public identities don't contain private/internal information (e.g., department, job role).
*   **Revocation:** CAs must support certificate revocation (e.g., if an employee leaves or loses a credential).
*   **Security Recommendation:** Use a **Hardware Security Module (HSM)** to protect the CA's private keys.
    *   *HSM:* A tamper-proof device; prevents key theft even if the CA server is compromised.

---

#### 2. How Certificate Trust Works

*   Relies on cryptographic "magic" .
*   **Mechanism:**
    *   A CA issues (signs) a certificate asserting an identity (e.g., an IP address belongs to a specific entity).
    *   Systems trust the certificate because they trust the issuing CA.
*   **Analogy:** A police officer trusts your driver's license because they trust the issuing authority (DMV), not because they know you personally.
*   **Hierarchies:** Trust can cascade. If you trust the top-level CA (Root CA), you implicitly trust certificates issued by CAs under it.

---

#### 3. PKI Hierarchy

*   **One-Tier:**
    *   A single CA issues all certificates.
    *   *Not recommended* except for testing.
*   **Two-Tier (Most Common & Recommended):**
    *   **Structure:**
        1.  *Offline Root CA:* The ultimate trust anchor, kept secure and offline. Pushed to all devices via MDM/GPO.
        2.  *Issuing CA(s):* Online CAs that issue end-entity certificates. Trust is inherited from the Root CA.
    *   **Benefits:**
        *   *Segregation:* Different Issuing CAs can be used for different purposes (e.g., VPN/Wi-Fi certs vs. Smart Card certs), preventing cross-contamination if one is compromised or misconfigured.
        *   *Simplified Trust:* Only the Root CA needs to be distributed and trusted.
    *   *Example:* Used at Microsoft.
*   **Three-Tier:**
    *   **Structure:**
        1.  *Offline Root CA*
        2.  *Policy CA(s):* Intermediate CAs defining rules and scope (e.g., Department of Energy CA).
        3.  *Issuing CA(s):* Managed by specific departments/teams under the Policy CAs.
    *   **Use Case:** Very large organizations (e.g., governments) needing strict separation of duties and policies between major divisions.
    *   **Downside:** Adds complexity; usually unnecessary.

---

#### 4. Important Parts of a Certificate (X.509 v3)

*   **Subject Name:**    *   Legacy field identifying the certificate holder.
    *   *Less used* for identity now.
    *   *Tip:* Use it to describe the certificate's *purpose* (e.g., `CN=AALF - Entra ID`) for easier selection in prompts.
*   **Subject Alternative Name (SAN):**
    *   An *extension* where modern identity information resides.    *   *Required* by most specifications.
    *   Can contain multiple identity types: Email, DNS Name, URL, IP Address, User Principal Name (UPN).
*   **Certificate Revocation List (CRL) Distribution Points:**
    *   URLs where the list of revoked certificates (CRL) can be downloaded.
*   **Authority Information Access (AIA):**
    *   URLs to:
        1.  Download the issuing CA's certificate (helps build the trust chain).
        2.  Access the OCSP responder (if used).
*   **Key Usages:**
    *   Defines *broad* allowed cryptographic operations for the key.
    *   *Examples:* Digital Signature, Key Encipherment, Certificate Signing, CRL Signing.
*   **Enhanced Key Usages (EKU):**
    *   Defines *specific* allowed application purposes for the certificate.
    *   *Examples:* Server Authentication, Client Authentication, Code Signing, Secure Email.
    *   Can include *custom* OIDs (Object Identifiers) for specific uses (e.g., VPN authentication) allowing servers to enforce EKU checks.
*   **Serial Number:** A unique number assigned by the CA to this specific certificate.
*   **Thumbprint:** A unique hash (digest) of the entire certificate.
*   **Validity Period:** `Valid From` and `Valid To` dates defining the certificate's lifespan.
*   **Extensions (Critical vs. Non-Critical):**
    *   Extensions add extra information/constraints.
    *   ***Critical*** *(often shown with a yellow exclamation mark - this is normal):* The application processing the certificate *must* understand and process this extension. If it doesn't, it *must* reject the certificate. (e.g., Key Usages).
    *   ***Non-Critical:*** The application can ignore the extension if it doesn't understand it.

---

#### 5. Certificate Revocation

*   Necessary when a certificate needs to be invalidated *before* its expiry date.
*   **Methods:**
    1.  **Certificate Revocation List (CRL):**
        *   A *blacklist* containing the serial numbers of revoked certificates.
        *   Signed and published periodically by the CA (e.g., *weekly* is common).
        *   Clients download the CRL and check if the certificate's serial number is present.
        *   *Delta CRLs:* Smaller, incremental updates issued between full CRL publications.
        *   *Challenge:* CRLs can become very large for busy CAs. Clients *must* check revocation, but sometimes don't (security risk).
    2.  **Online Certificate Status Protocol (OCSP):**
        *   A real-time *API query* protocol.
        *   Client asks an OCSP responder (server): "Is certificate [serial number] valid?". Responder answers "Good", "Revoked", or "Unknown".
        *   *Benefit:* Avoids large CRL downloads, better for low-resource devices.
        *   *Challenge:* Not universally supported by all services (e.g., *Azure AD / Entra ID Certificate-Based Authentication does not support OCSP*, relies on CRLs).
        *   ***OCSP Stapling:*** A performance optimization where the web server proactively fetches the signed OCSP response and sends ("staples") it along with its certificate during the TLS handshake, saving the client a separate lookup.

---

#### 6. Common Mistakes

*(Addressed implicitly via Best Practices)*

---

#### 7. Best Practices

*   **Keep the Root CA Offline:** Protect the most critical key. Store securely, have offsite backups.*   **Use Hardware Security Modules (HSMs):** Protect private keys for both Root and Issuing CAs. Offloads cryptographic operations.
*   **Keep Secure Offsite Backups:** Essential for disaster recovery. *Practice restoring* from backups regularly.
*   **Use Quorum for Key Access (HSM):** Require multiple individuals/smartcards (e.g., 3 out of 8) to authorize sensitive operations. Store quorum parts securely and geographically separated.
*   **Ensure Redundancy / High Availability:**
    *   *Crucial* for CRL Distribution Points and OCSP Responders. If these are down, certificate validation fails, leading to authentication failures.
    *   Consider using robust storage (e.g., Azure Storage) or managed cloud PKI solutions.
    *   CA redundancy (for issuance) is less critical than revocation endpoint availability unless issuance volume is very high (e.g., IoT).

---

#### PKI Glossary / Cheat Sheet Terms Mentioned:

*   CRL (Certificate Revocation List)
*   OCSP (Online Certificate Status Protocol)
*   HSM (Hardware Security Module)
*   RA (Registration Authority - *mentioned in glossary slide but not discussed*)
*   CSR (Certificate Signing Request - *mentioned in glossary slide but not discussed*)
*   AIA (Authority Information Access)
*   CA (Certificate Authority)
*   PKI (Public Key Infrastructure)
*   EKU (Enhanced Key Usage)*   SAN (Subject Alternative Name)