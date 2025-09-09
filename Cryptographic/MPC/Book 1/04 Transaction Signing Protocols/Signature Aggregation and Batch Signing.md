Below is a self-contained “mini-textbook” on the two most common ways to **amortise signature creation in Multi-Party Computation (MPC) wallets**:

1. Signature Aggregation (many signatures → one short aggregate)  
2. Batch Signing (many messages → one signing session)

We will look at **why** they are useful, **how** they work inside an MPC protocol, **what** security properties must be preserved, and **where** the trade-offs are.

---

## 1. Why bother?

| Problem | Aggregation fix | Batch-signing fix |
|---------|-----------------|-------------------|
| Blockchains charge per byte; 100 64-byte ECDSA sigs = 6.4 kB | One 32-48 byte BLS aggregate | n/a |
| 100 separate “MPC sign” rounds = 100·(3–10) s latency | n/a | One 3-round signing produces 100 sigs |
| Verifier does 100 expensive ECDSA pairings | One pairing for BLS aggregate | n/a |

---

## 2. Quick refresher on MPC signing

In **threshold ECDSA/BLS/Ed25519** every player *Pᵢ* holds a secret share  
skᵢ such that the implicit joint public key is

  pk = g^{sk}  with  sk = Σ skᵢ

but **sk is never reconstructed**.  
To sign a single message m the players run an interactive protocol (typically 3–5 rounds) that outputs a standard signature σ under pk.  
Aggregation and batching are **orthogonal tricks** that we plug **around** this protocol.

---

## 3. Signature Aggregation inside MPC

Aggregation means: **n distinct signers** (or one signer n times) produce **n individual signatures** σ₁…σₙ and anybody (no secret keys) compresses them into one short string σₐgg that validates under the n public keys.

### 3.1 Cryptographic tool-kit

| Scheme | Aggregation | MPC friendly? | Notes |
|--------|-------------|---------------|-------|
| BLS (Boneh–Lynn–Shacham) | yes, one group element, *non-interactive* | yes, pairing-based DKG exists | Needs pairing-friendly curve (BLS12-381) |
| Schnorr | yes, linear sig: σₐgg = Σ σᵢ | yes, already used in most MPC-ECDSA | Verification needs n public keys |
| ECDSA (secp256k1) | **no algebraic aggregation** | n/a | Must fall back to batch-verification or proof of knowledge |

### 3.2 How to aggregate **inside** an MPC wallet

Step 0. Distributed key generation (DKG)  
All n devices jointly run an MPC-DKG so that each obtains skᵢ and the **single** aggregate public key pk = g^{Σ skᵢ} is published.

Step 1. Individual signing  
For every message mⱼ the signers run the standard MPC signing protocol → obtain σⱼ (Schnorr or BLS).

Step 2. Public compression  
Anybody (even a light client) computes  
  σₐgg = Σ σⱼ  (Schnorr)  or  σₐgg = ∏ σⱼ  (BLS)  
and broadcasts one signature instead of n.

Step 3. Verification  
Verifier runs one pairing (BLS) or one multi-base exponentiation (Schnorr) with the list of public keys and messages.

> Security caveat: you must prove that the same set of messages/public-keys was used. For Schnorr add the public-key list in the challenge hash; for BLS use the *rogue-key* protection (proof-of-possession or Delrina–Wagner mitigation).

### 3.3 Concrete sizes & speed (BLS12-381, 128-bit security)

| # sigs | individual total | aggregate size | verify time |
|--------|------------------|----------------|-------------|
| 1 | 48 B | 48 B | 1.0 ms |
| 100 | 4.8 kB | 48 B | 1.2 ms |
| 1 000 | 48 kB | 48 B | 1.9 ms |

---

## 4. Batch Signing (one MPC session → many signatures)

Aggregation shrinks **communication after** signing; batch signing shrinks **communication during** signing.

### 4.1 Naïve way = n sequential MPC rounds  
Cost ≈ n·(3–5) rounds · (network latency + crypto)

### 4.2 Better way = “one-many” threshold protocol

High-level idea (works for Schnorr, ECDSA, Ed25519):

1. **Pre-computation (offline)**  
   Players jointly generate one *presignature* Γ = g^{k⁻¹} where k = Σ kᵢ is secret-shared.  
   This costs one heavy round but **Γ can be reused** for many messages.

2. **Online phase (per message mⱼ)**  
   a. Each player locally hashes eⱼ = H(pk, mⱼ, Γ)  
   b. One lightweight round: players convert their secret shares into additive shares of  
       sⱼ = k(eⱼ + sk)  
   c. Publish sⱼ; anybody sums → s = Σ sⱼ  
   d. Signature is σⱼ = (Γ, s) (Schnorr) or (rⱼ, sⱼ) (ECDSA).

3. **Complexity**  
   - Offline: 1 heavy 3-round protocol  
   - Online: 1 broadcast round per message (can be done in parallel)  
   - Total latency ≈ 1 + ε rounds for **unlimited** signatures.

### 4.3 Security sketch

The presignature Γ must be **bound** to each message via the hash; otherwise Wagner’s attack (generic birthday) breaks existential unforgeability.  
The online phase reveals **no information** about sk because each sⱼ is masked by the fresh random k.

---

## 5. Putting both tricks together

| Wallet workflow | Latency | Bandwidth | On-chain cost |
|-----------------|---------|-----------|---------------|
| 100 individual MPC signs | 100·3 rounds | 100·σ | 100·σ |
| Batch-sign 100 + aggregate | 1 + ε rounds | 100·σ | 1·σₐgg |

**Example numbers** (Schnorr, secp256k1, 100 sigs)  
- Naïve: 300 rounds, 6.4 kB broadcast, 6.4 kB chain  
- Batch+aggregate: 2 rounds, 6.4 kB broadcast, 64 B chain

---

## 6. Implementation tips & open-source code

- **BLS**: `blst` (C/Go/Rust) + `threshold-bls` (DFINITY)  
- **Schnorr batch**: `secp256k1-zkp` (Elements) has `musig_batch` module  
- **ECDSA batch**: `tss-lib` (Binance) & `mpclib` (ZenGo) expose *presign* API  
- **Full wallet**: Fireblocks, Curv, Coinbase Wallet-as-a-Service, KZen all run batch + aggregate under the hood.

---

## 7. Recap cheat-sheet

|  | Signature Aggregation | Batch Signing |
|---|-----------------------|---------------|
| **Goal** | shrink *signature* size | shrink *signing* rounds |
| **When** | after signatures exist | during signing |
| **Crypto required** | homomorphic sig (BLS/Schnorr) | presign techniques |
| **MPC change** | none (public operation) | add presign phase |
| **Verifier cost** | one pairing / multi-exp | same as normal |
| **Trust model** | transparent (anyone aggregates) | still threshold t-of-n |

Master these two techniques and you can build an MPC wallet that is **as cheap to verify as a single signature** and **as fast to sign as one online round**, no matter how many transactions your user fires.