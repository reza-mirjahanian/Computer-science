# What is MPC? History and Evolution

Multi-Party Computation (MPC) is a cryptographic paradigm that allows several parties to jointly compute a public function over their private inputs without ever revealing those inputs to one another. In other words, MPC guarantees that each participant learns only the final result—nothing more, nothing less. This article traces the journey of MPC from a purely theoretical curiosity in the early 1980s to a practical tool now safeguarding billions of dollars in digital assets.

---

## 1. The Genesis: From “The Millionaire’s Problem” to General Feasibility

### 1.1 1982 – Yao’s Millionaire’s Problem  
Secure two-party computation was born when Andrew Yao asked a deceptively simple question: “Can two millionaires find out who is richer without disclosing their actual wealth?” Yao’s solution introduced garbled circuits, a technique that still underpins many modern protocols.

### 1.2 1986 – General Two-Party Feasibility  
Yao extended his idea to show that *any* polynomial-time two-party function can be computed securely, provided that at least one party is semi-honest (follows the protocol but tries to learn extra information).

### 1.3 1987 – The GMW Breakthrough  
Goldreich, Micali, and Wigderson (GMW) generalized the result to an arbitrary number of parties and malicious adversaries. Their famous completeness theorem states: “If one-way functions exist, then every multi-party functionality can be securely computed against a static, malicious adversary corrupting strictly less than half of the parties.” This transformed MPC from a collection of clever tricks into a full-fledged branch of cryptography.

---

## 2. The 1990s: Refinements and Foundational Questions

With feasibility established, researchers turned to efficiency, composability, and stronger security notions.

- **Universal Composability (UC)** – Introduced by Ran Canetti, UC framework allowed MPC protocols to be securely plugged together like Lego bricks.  
- **Zero-Knowledge Proofs** – Became a standard building block for enforcing honest behavior without revealing data.  
- **Mobile & Reactive Security** – Explored how MPC could survive dynamic adversaries that corrupted parties over time.

Despite these advances, protocols remained orders of magnitude too slow for real-world deployment; MPC was still conference-room cryptography.

---

## 3. 2000–2010: First Practical Deployments

### 3.1 2008 – Danish Sugar Beet Auction  
A consortium of Danish farmers used MPC to run a secure double auction on 25,000 tons of production contracts. For the first time, live MPC computation ran on commodity servers connected over the public Internet. The auction finished in minutes, proving that theory could meet practice.

### 3.2 Parallel Efforts  
- **SIMAP & VIFF** – Open-source frameworks written in Python made MPC accessible to non-cryptographers.  
- **Financial Benchmarks** – Banks experimented with MPC to compute LIBOR-style benchmarks without exposing individual submissions.

---

## 4. 2010–2020: Specialization and Standardization

### 4.1 Homomorphic Encryption & SPDZ  
The SPDZ family of protocols (pronounced “Speedz”) combined somewhat homomorphic encryption with message authentication to achieve malicious security and *pre-processing*: most heavy cryptography moved to an offline phase, leaving the online phase extremely fast.

### 4.2 Industry Consortiums  
2020 saw the birth of the **MPC Alliance**, a trade body of over 50 companies aiming to standardize APIs, audit methodologies, and interoperability tests.

### 4.3 Regulatory Tailwinds  
GDPR, HIPAA, and cross-border data-transfer rules turned MPC from a “nice-to-have” into a compliance tool: organizations could now derive insights from pooled datasets without ever centralizing raw personal data.

---

## 5. MPC in Digital-Asset Security: The Killer App

### 5.1 Key Sharding vs. Traditional Storage  
Legacy wallets store a single private key in an HSM, hardware wallet, or exchange database—each a single point of compromise. An MPC wallet splits the key into *n* cryptographically linked shares, distributed across laptops, servers, or mobile devices. No single machine ever reconstructs the key; instead, machines jointly sign transactions using additive share sequences.

### 5.2 Automatic Refresh & 1-Round Signing  
Fireblocks’ MPC-CMP protocol (2019) introduced **1-round, automatic key refresh**: shares are re-randomized after every signature, so yesterday’s breach cannot sign today’s transaction—even if the adversary learns *t* of *n* shares.

### 5.3 Institutional Adoption  
By 2024, custodians, neobanks, and fintechs collectively safeguarded over **$60 billion** in cryptocurrencies using MPC-based custody. The technology’s ability to eliminate single points of failure while retaining hot-wallet speed has made it the de-facto standard for enterprise-grade digital-asset infrastructure.

---

## 6. Core Technical Concepts Demystified

| Term | One-Sentence Intuition |
|------|------------------------|
| **Secret Sharing** | Split a secret *s* into *n* pieces so that any ≤ *t* pieces reveal nothing, but *t*+1 reconstruct *s*. |
| **Garbled Circuit** | One party encrypts a Boolean circuit; the other obliviously evaluates it gate-by-gate without learning intermediate wire values. |
| **Oblivious Transfer** | Sender has *m* messages; receiver chooses *i* without revealing *i*; sender does not learn which message was picked. |
| **Broadcast Channel** | All messages are published identically to every party; prevents selective message delivery. |
| **Universal Composability** | A protocol is “secure” if it behaves like an ideal functionality *F* even when arbitrarily composed with other protocols. |

---

## 7. Frontier Research & Open Challenges

1. **Post-Quantum MPC**  
   Current protocols rely on factoring/discrete-log assumptions. NIST-compatible lattice-based primitives are being woven into SPDZ-style frameworks.

2. **Scalability to Thousands of Parties**  
   Most protocols scale quadratically or worse. Recent works use “MPC-in-the-head” and committee election to achieve quasi-linear communication.

3. **Threshold Signature Schemes (TSS)**  
   Blockchain communities favor TSS because it produces ordinary ECDSA/Schnorr signatures verifiable by existing chains—no smart-contract changes needed.

4. **Verifiable Delay Functions inside MPC**  
   Combining VDFs with MPC could enable trustless lotteries and leader-election in proof-of-stake systems without leaking entropy.

5. **Usability & Formal Verification**  
   High-level languages (e.g., MP-SPDZ, SCALE-MAMBA) now compile Python-like code into bytecode executed by secret-shared VMs. Formal verification of such compilers is an active research goal.

---

## 8. Looking Ahead: MPC as Invisible Infrastructure

Much like SSL became an unnoticed background process that secures everyday web traffic, MPC is poised to vanish into the stack. Tomorrow’s credit-score checks, medical diagnostics, collaborative AI training, and even privacy-preserving genome matching will simply *use* MPC, the same way mobile apps today *use* AES or TLS.

From millionaires comparing salaries to global custodians moving billion-dollar transactions, MPC has traveled from elegant chalkboard proofs to production-grade, regulator-audited systems in just four decades. Its evolution illustrates a broader trend in cryptography: once a breakthrough is reduced to a library call, innovation shifts to making that call faster, cheaper, and invisible to the end user. The next time you sign a blockchain transaction in milliseconds or help compute a pooled data insight without exposing raw records, remember that you are witnessing the ongoing history of Multi-Party Computation—one secret-shared piece at a time.

---

### References  
: Fireblocks – *What is MPC (Multi-Party Computation)?*  
: Chainlink – *Secure Multi-Party Computation*  
: SCAND – *MPC Wallet Development: A Comprehensive Guide*