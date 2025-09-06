# 🔐 What is an MPC Wallet?

An ***MPC (Multi-Party Computation) wallet*** is a type of cryptocurrency wallet that uses MPC cryptography to manage your keys. This technology allows two or more parties to jointly compute a function's output without revealing their individual inputs.

# ⚙️ How Does it Work?

MPC creates a secure key management system that avoids a single point of failure, unlike a traditional `private key`. Here’s a breakdown of the process:

* **Secret Shares Creation**
    * Instead of a single private key, MPC wallets create two independently created mathematical *"secret shares"*.
* **Storage of Secret Shares**
    * One share is stored on your mobile device.
    * The other share is stored on the Zengo server.
* **Transaction Signing**
    * When a transaction needs to be signed, the parties involved (your phone and the Zengo server) separately run a computation.

> This process ensures that no single entity ever has full access to the private key, as it is never generated, split, or reconstructed in its entirety.

# ✨ Key Features of MPC Wallets

| Feature | MPC Wallet | Traditional Wallet |
| :--- | :--- | :--- |
| **Key Management** | Uses multi-party computation with "secret shares" | Relies on a single `private key` |
| **Security** | No single point of failure | Vulnerable if the `private key` is compromised |
| **Recovery** | Possible even if one share is lost | Difficult or impossible if the `private key` is lost|
| **User Experience** | Simpler and more user-friendly | Can be complex, requiring users to manage their own keys |

----

---

### What Is an MPC Wallet?

**MPC (Multi-Party Computation) wallets** like **Zengo** replace traditional private keys—often stored as a single seed phrase—with **two separate mathematical “secret shares.”**

* One share resides on your **mobile device**, and the other on **Zengo’s server**—**no single point of failure** ensures only you can access your crypto.

---

### Why Seed Phrases Are Risky

* Seed phrases pose a massive vulnerability: losing or exposing them can compromise your entire account.
* Avoiding them removes a huge barrier to crypto adoption, making it safer and more approachable for new users.

---

### How MPC Works: The Zengo Model

1. **Secret Share Generation**

   * Two independent shares are created:

     * **Personal Share**: stored securely on your device (iPhone's Secure Enclave or Android’s TEE)
     * **Remote Share**: securely held on Zengo’s server

2. **Transaction Signing**

   * Both shares jointly compute the signature needed for blockchain transactions
   * Neither party ever gains access to the entire private key

3. **Threshold Signature Scheme (TSS)**

   * A form of MPC; cryptographic operations occur assuming a threshold of honest participants—here, two parties can jointly sign without exposing full keys

---

### Key Benefits of Zengo’s MPC Wallet

* **Eliminates seed phrase vulnerability**
* **Robust security**: Even if one share is breached, the attacker can’t reconstruct the key
* **Recoverable & user-friendly**: No need for complex backups; users can regain access via 3FA (email, 3D FaceLock, recovery file)
* **Widely supported**: Compatible across multiple blockchains and assets—chain-agnostic design

---

### Zengo Pro: Advanced Security Features

* **Multiple Wallets**

  * Create up to **five isolated wallets** under one account, each with its own security logic

* **Private Transaction Mode**

  * For BTC and DOGE, generates a **new address per transaction** to enhance privacy while consolidating balances in your account

* **Theft Protection**

  * Set thresholds for transactions requiring **3D FaceLock** to approve high-value movements

* **Web3 Firewall & Fraud Detection**

  * Real-time risk alerts to block suspicious Web3 activity

---

### Zengo’s Open-Source, Institutional Security Approach

* **Gotham City Library** (MPC cryptographic toolkit):

  * Accessible, open-source Rust implementation supporting Ethereum and Bitcoin, with features like key derivation and rotation

---

### Institutional Trust & Patents

* Zengo is backed by prominent institutional users and brings MPC to the consumer space safely
* Protected by **multiple US and EU patents**, with a history of zero hacks—despite a \$500K bounty, no wallets have been compromised

---

### Review Highlights: Pros & Cons

| Pros                                      | Cons                                                  |
| ----------------------------------------- | ----------------------------------------------------- |
| MPC-based elimination of seed phrase risk | Some advanced features are behind a premium tier      |
| Strong 3FA account recovery               | Partial reliance on centralized server (remote share) |
| Chain-agnostic with open-source backing   | Fewer supported blockchains vs. some competitors      |
| Never hacked among 1.5M+ users            |                                                       |

---

### Reddit Insight

> “One share (your Personal Share) is created and stored on your mobile device… Every time you initialize a transaction… the two secret shares securely compute to create a signed transaction.”

---

### Summary at a Glance

* **MPC architecture = no seed phrases, far safer**
* **Zengo leverages device + server secret shares** to sign transactions securely
* **Extra layers through Zengo Pro**: privacy, theft protection, multiple wallets
* **Built for transparency**: open-source components like Gotham City
* **Trusted & protected**: patented tech, institutional backing, no hacks

---
