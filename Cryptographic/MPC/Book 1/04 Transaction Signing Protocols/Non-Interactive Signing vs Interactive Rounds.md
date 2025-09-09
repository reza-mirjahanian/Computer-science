# Non-Interactive Signing vs Interactive Rounds in MPC

When multiple parties jointly compute a signature without revealing their private shares, MPC-based signing protocols can either require live, back‐and‐forth communication (“interactive rounds”) or enable signing with minimal or no online interaction (“non‐interactive signing”). Understanding the trade‐offs helps choose the right design for latency, security, and deployment constraints.

---

## What Are Interactive Rounds?

Interactive signing protocols unfold in a sequence of message exchanges between all participants. Each “round” consists of every party sending one message, typically:
1. **Commitment Exchange**  
   Parties share commitments to randomness or partial computations.
2. **Challenge/Response**  
   Based on received commitments, they exchange challenges and auxiliary data.
3. **Aggregation**  
   Final partial signatures are combined into the complete signature.

The total round count equals the number of sequential exchanges needed to preserve security properties. For example, classic threshold ECDSA protocols often require 6–9 online rounds to generate a signature, leading to higher latency if parties are geographically dispersed or operating behind network constraints.

### Pros and Cons of Interactive Rounds

- Pros  
  - Strong, composable security in the Universal Composability (UC) framework  
  - Can adaptively resist malicious behavior with on‐the‐fly checks  

- Cons  
  - High latency when parties have high network latency  
  - Complex coordination required for every signing operation  
  - Harder to support air‐gapped or cold vault setups  

---

## What Is Non‐Interactive Signing?

Non‐interactive signing separates most cryptographic work into an offline (preprocessing) phase. During this phase, parties:
1. Generate and share random “signing tokens” or multiplication triples  
2. Store these tokens locally until a message needs signing  

When the message arrives, participants use precomputed tokens to produce their partial signatures in a single burst of computation—and often a single communication step—rather than multiple interactive rounds. In some designs, all necessary messages are embedded into zero‐knowledge proofs or non‐interactive proofs of correctness, eliminating live interaction altogether.

### Example: 4 Rounds with 3 Offline

Canetti, Makriyannis, and Peled’s threshold ECDSA protocol reduces the online signing stage to just **1** interactive round by pushing three of its four total rounds into a preprocessing stage. This effectively yields a non‐interactive signing experience when the message is known, because no extra exchanges are required during the live phase.

### Ultra‐Low Latency via 1 Online Round

Fireblocks’ MPC‐CMP protocol pushes this further: it requires exactly **1** round for the entire signing operation, delivering up to **8×** faster transaction signing compared to standard 9‐round protocols. By supporting a cold‐storage share and a single interactive step, it bridges the gap between high security and enterprise speed demands.

---

## Side‐by‐Side Comparison

| Aspect                       | Interactive Rounds                         | Non‐Interactive Signing                                      |
|------------------------------|--------------------------------------------|--------------------------------------------------------------|
| Online Latency               | Proportional to number of rounds           | Minimal (often 1 round or zero if proofs are pre‐packaged)   |
| Preprocessing Requirement    | Typically none                             | Significant token or triple generation upfront               |
| Cold/Hot Wallet Compatibility| Harder (requires live links)               | Easier (offline share can remain air‐gapped)                 |
| Security Model               | UC‐secure with adaptive checks             | UC‐secure but relies on precomputed tokens’ integrity        |
| Implementation Complexity    | Moderate to high                           | Higher (managing offline tokens, zero‐knowledge proofs)      |

---

## When to Choose Which?

- **Interactive Rounds** are preferable if you prioritize:
  - On‐the‐fly security checks against malicious behavior  
  - Simpler key management (no large preprocessing storage)  
  - Environments where network latency is low and parties stay online  

- **Non‐Interactive Signing** excels when you need:
  - Ultra‐low signing latency or single‐step signing SLAs  
  - Cold wallet or air‐gapped setups with minimal online exposure  
  - Bulk signing where preprocessing amortizes over many signatures  

---

## Looking Ahead

Beyond ECDSA, these principles apply to any threshold signature or MPC‐based cryptosystem (e.g., Schnorr, BLS). As zero‐knowledge proof tech matures, we’ll likely see more protocols embedding entire interactive phases into succinct proofs, pushing toward truly zero‐round signing experiences in permissioned and permissionless settings alike.

**Further reading:**
- Explore the UC framework’s treatment of online vs. preprocessing rounds in threshold signatures.  
- Compare MPC‐CMP against other one‐round MPC proposals in DAC conferences.  
- Investigate emerging Schnorr‐based MPC schemes with post‐quantum resilience.


-----------------------------------




In Multi-Party Computation (MPC), **Non-Interactive Signing** and **Interactive Rounds** refer to two approaches for generating digital signatures collaboratively without exposing private keys. Below is a structured comparison to clarify their differences, use cases, and trade-offs.

---

### **1. Interactive Signing**
**Definition**:  
Parties must communicate in multiple rounds during the signing process. Each round involves exchanging messages (e.g., partial signatures, zero-knowledge proofs) to collectively compute the final signature.

#### **Key Characteristics**:
- **Real-Time Coordination**:  
  All parties (or a threshold subset) must be online and synchronize messages during signing.
- **Multiple Communication Rounds**:  
  Requires 2+ rounds of interaction (e.g., 2–4 rounds in protocols like GG18/GG20 for ECDSA).
- **Dynamic Adaptability**:  
  Can handle ad-hoc signing requests without prior setup beyond key generation.
- **Complexity**:  
  Higher communication overhead due to round trips. Vulnerable to network latency or party dropouts.

#### **How It Works**:
1. **Key Generation** (Interactive):  
   Parties generate shares of a private key collaboratively (e.g., via Pedersen DKG).
2. **Signing** (Interactive):  
   - **Round 1**: Parties compute partial signatures using their key shares and exchange commitments.  
   - **Round 2**: Parties exchange partial signatures and verify consistency.  
   - **Round 3+**: Additional rounds for zero-knowledge proofs (e.g., to ensure security against malicious actors).  
   - **Final Step**: A combiner aggregates partial signatures into a valid signature.

#### **Examples**:
- **Threshold ECDSA**: GG18, GG20, CMP protocols.  
- **Schnorr Signatures**: Some MPC implementations require 1–2 rounds.

#### **Pros & Cons**:
| **Pros**                          | **Cons**                          |
|-----------------------------------|-----------------------------------|
| ✅ No precomputation needed       | ❌ High latency (unsuitable for high-throughput apps) |
| ✅ Flexible for ad-hoc signing    | ❌ Vulnerable to network failures |
| ✅ Strong security (malicious-secure variants exist) | ❌ Scalability issues with many parties |

---

### **2. Non-Interactive Signing**
**Definition**:  
Signing requires **no real-time communication**. Parties precompute values during an initial setup phase. Later, each party generates a partial signature locally, which anyone can combine into a full signature without further interaction.

#### **Key Characteristics**:
- **Precomputation Phase**:  
  An interactive setup generates "one-time" nonces or randomness (e.g., via MPC).  
- **Single-Shot Signing**:  
  During signing, parties work offline. No communication is needed.  
- **Amortized Overhead**:  
  Setup costs are spread over many signatures. Ideal for high-frequency signing.  
- **Simplicity**:  
  Low latency during signing; combiner only needs partial signatures.

#### **How It Works**:
1. **Setup Phase** (Interactive, one-time):  
   Parties jointly generate:  
   - Long-term key shares (as in interactive MPC).  
   - **Precomputed nonces** (e.g., using Beaver triples or similar).  
2. **Signing Phase** (Non-Interactive):  
   - Each party uses its key share and precomputed nonce to generate a **partial signature** locally.  
   - A combiner (e.g., a server or any party) aggregates partial signatures into a valid signature.  
   - **No communication** between parties during signing.

#### **Examples**:
- **BLS Signatures**: Inherently non-interactive in MPC setups.  
- **Schnorr with Precomputation**: Parties precompute nonces; signing is a local computation.  
- **ECDSA with Precomputed Nonces**: Some protocols (e.g., Lindell17) reduce signing to 1 round via precomputation.

#### **Pros & Cons**:
| **Pros**                          | **Cons**                          |
|-----------------------------------|-----------------------------------|
| ✅ Minimal signing latency        | ❌ Setup phase is complex and interactive |
| ✅ High throughput (ideal for blockchains) | ❌ Precomputed nonces are single-use; must refresh periodically |
| ✅ Resilient to network issues     | ❌ Storage overhead for precomputed data |

---

### **3. Key Differences Summary**
| **Aspect**               | **Interactive Signing**                          | **Non-Interactive Signing**               |
|--------------------------|--------------------------------------------------|-------------------------------------------|
| **Communication Rounds** | 2+ rounds during signing                         | 0 rounds during signing                   |
| **Setup Phase**          | Key generation only                              | Key generation + precomputation           |
| **Signing Speed**        | Slow (ms–s due to latency)                       | Fast (µs–ms, local computation)           |
| **Use Cases**            | Ad-hoc signing, low-frequency transactions       | High-frequency signing (e.g., blockchains) |
| **Security Model**       | Malicious-secure variants exist (e.g., GG20)     | Often semi-honest; malicious variants complex |
| **Scalability**          | Poor for large parties/high latency              | Excellent (scales with precomputation)    |
| **Example Protocols**    | GG18, GG20 (ECDSA), Frost (Schnorr)              | BLS, Schnorr with precomputation, Lindell17 |

---

### **4. When to Use Which?**
- **Choose Interactive Signing If**:  
  - Signing events are infrequent or unpredictable.  
  - Parties can coordinate in real-time (e.g., enterprise MPC).  
  - Security against malicious adversaries is critical.  

- **Choose Non-Interactive Signing If**:  
  - High throughput is needed (e.g., blockchain validators).  
  - Parties are geographically distributed with high latency.  
  - Precomputation overhead is acceptable (e.g., long-running systems).

---

### **5. Security Considerations**
- **Interactive Protocols**:  
  Use zero-knowledge proofs (ZKPs) to ensure malicious parties don’t sabotage signing.  
- **Non-Interactive Protocols**:  
  Precomputation must be secure (e.g., ZKPs to verify nonce generation).  
  Refresh precomputed data periodically to prevent replay attacks.

---

### **Conclusion**
- **Interactive Signing** prioritizes flexibility and security at the cost of real-time communication.  
- **Non-Interactive Signing** optimizes for speed and scalability by shifting complexity to a setup phase.  

Modern MPC systems often blend both: using non-interactive signing for efficiency while retaining interactive key generation for security. The choice hinges on application-specific constraints like latency, throughput, and adversary models.