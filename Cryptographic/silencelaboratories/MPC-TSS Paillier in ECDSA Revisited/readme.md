# 🔐 A Compute Perspective of MPC-TSS: Paillier in ECDSA Revisited

## 📜 Background: Why MPC Matters

Multi-Party Computation (MPC) is gaining traction due to:
- Increasing attacks on centralized key management systems
- Failures of centralized entities (e.g., FTX)
- The push for decentralized financial products

> **Threshold Signature Schemes (TSS)** are a practical application of MPC, enabling decentralized key management for cryptographic signatures like ECDSA.

---

## 🧠 What Is a Threshold Signature?

**Threshold Signatures** allow multiple parties to:
- Jointly compute a digital signature
- Use secret shares of a private key
- Operate without trusting each other

### 🔄 Three Phases of TSS
1. **Distributed Key Generation (DKG)**
2. **Distributed Signing**
3. **Pro-activization** (Key rotation/refresh)

> Devices involved can include smartphones, servers, and edge devices.

---

## 📉 Complexity of ECDSA in MPC

ECDSA is:
- Widely used (Bitcoin, Ethereum)
- Compact and fast
- Based on elliptic curves

However, its **non-linear structure** makes it harder to decentralize compared to:
- **BLS** (non-interactive)
- **EdDSA/Schnorr** (simple three-round protocols)

---

## 🔢 Secure Multiplication (MUL) in ECDSA

MPC protocols modularize tasks. A key subroutine is:

```plaintext
Secure Multiplication (MUL)
```

### 🔍 MUL Subroutine
- Two parties (e.g., Alice and Bob)
- Inputs: `a` and `b` (private integers)
- Outputs: `u` and `v` such that `u + v = a * b`

---

## 🔐 Paillier Encryption in Threshold ECDSA

Paillier-based MUL is used in protocols like Lindell 17, GG18, GG20.

### ⚠️ Challenges with Paillier
- Based on **integer factoring**, unlike ECDSA’s elliptic curves
- Requires **larger parameters** due to known factoring attacks
- Operates on **thousands of bits**, increasing computational load

### 📊 Benchmark Insights

| Device Type     | Paillier Key Gen Time |
|-----------------|------------------------|
| Smartphones     | 100 µs – 1 second      |
| Smartwatches    | ~2 seconds             |

> Paillier-based MUL can consume **>50%** of total protocol time.

---

## 🔄 Alternative: OT-MUL via DKLs Protocols

Instead of Paillier, DKLs18 and DKLs19 use:

```plaintext
Oblivious Transfer-based Multiplication (OT-MUL)
```

### ⚙️ How OT-MUL Works
- Based on **Gilboa’s OT protocol**
- Uses **OT Extension** for efficiency
- Requires only a few thousand hash computations

### ✅ Benefits of OT-MUL
- No need for zero-knowledge proofs
- Built from same primitives as ECDSA (elliptic curves, hash functions)
- Smaller integers → lower computational cost

### ⚡ Tradeoff
- **Higher bandwidth usage**
- But latency remains **imperceptible** on modern devices

---

## 📱 Real-World Deployment

Silence Laboratories tested:
- MPC-based wallets using OT-MUL
- Devices: low-budget smartphones and smartwatches

> Signing latency was minimal, making OT-MUL ideal for user-facing applications.

---

## 🧩 Summary Table: Paillier vs OT-MUL

| Feature                     | Paillier-Based MUL       | OT-MUL (DKLs Protocols)     |
|----------------------------|---------------------------|-----------------------------|
| Cryptographic Basis        | Integer Factoring         | Elliptic Curves + Hashes    |
| Computational Load         | High                      | Low                         |
| Zero-Knowledge Proofs      | Required                  | Not Required                |
| Bandwidth Usage            | Low                       | Higher                      |
| Device Compatibility       | Limited (due to load)     | Broad (phones, watches)     |
| Efficiency in Real Devices | Slower                    | Fast and responsive         |

---

## 🛠️ Key Takeaway

> OT-MUL via DKLs protocols offers a **lighter, faster**, and **more compatible** alternative to Paillier-based MUL for threshold ECDSA — especially in real-world, user-facing applications.