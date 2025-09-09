# 🔐 Commitment Schemes  
*“The digital safe that never rusts.”*

---

## 🎯 Core Purpose  
- **One-way lock** – hide data today, reveal it later  
- **Two-party distrust** – neither side can cheat after the lock clicks  
- **Two-phase ritual**  
  1. **Commit** – sender publishes *c* = Commit(*m*, *r*)  
  2. **Reveal** – sender opens (*m*, *r*); verifier checks *c*

---

## 🛡️ Required Security Properties  

| Property | Intuition | Failure Horror Story |
|----------|-----------|----------------------|
| **Correctness** | Honest commit → honest verify always passes | Receiver rejects valid coin-flip → protocol dead |
| **Hiding** | *c* leaks **zero** bits of *m* before reveal | Sherlock sees “odd/even” → game ruined |
| **Binding** | Sender **cannot** find (*m′*, *r′*) ≠ (*m*, *r*) with same *c* | Watson flips coin after Sherlock’s guess → casino bankrupt |

*Impossibility tidbit* 🚫  
> **Perfect hiding + perfect binding** ⇒ contradiction (unbounded brute-force breaks hiding).  
Pick your poison: *perfect* on one side, *computational* on the other.

---

## 🔧 Construction 1 – Hash-Based Commitment  
### Recipe (Pedagogical)  
1. Sender chooses **r** ← {0,1}^256 (true random)  
2. Compute *c* = **H(r ‖ m)**  
3. Publish *c* only  

### Security Ledger  
| Axis | Guarantee | Assumption |
|------|-----------|------------|
| **Hiding** | *Computational* | **Pre-image resistance** of *H* |
| **Binding** | *Computational* | **Collision resistance** of *H* |

### ⚠️ Weakness Spotlight  
*Short message space* (e.g., “yes”/“no”) → **enumerative attack** even with strong *H*.  
**Fix**: pad *m* to 256-bit domain separator or use **HMAC(k, m)** with secret *k*.

---

## 🔧 Construction 2 – Pedersen Commitment  
### Mathematical Blueprint  
Public group **G** = ⟨g⟩, order *q* prime.  
Additional generator **h** = g^x with unknown *x* (trusted setup).  

**Commit**  
*c* = g^m · h^r  (*m* ∈ ℤ_q, *r* ← ℤ_q)  

**Reveal**  
Send (*m*, *r*); verifier checks *c* ≟ g^m · h^r  

### Security Ledger  
| Axis | Guarantee | Reduction |
|------|-----------|-----------|
| **Hiding** | **Perfect** | For every *m′* ∃ unique *r′* s.t. *c* = g^m′ · h^r′ |
| **Binding** | *Computational* | Breaking ⇒ solving **Discrete Log** in *G* |

### Schnorr-like Bonus 🎁  
Pedersen is **additive homomorphic**:  
*c₁* · *c₂* = g^{m₁+m₂} · h^{r₁+r₂} ← commitment to sum without reveal.

---

## 🔍 Comparative Matrix  

| Feature | Hash-&-Random | Pedersen |
|---------|---------------|----------|
| Setup | None (standard hash) | **Trusted** unknown-log setup |
| Hiding | Computational | **Perfect** |
| Binding | Computational | Computational |
| Post-Quantum | **Shaky** (hash-only) | **Broken** (Shor on DLP) |
| Homomorphic | ❌ | ✅ |
| Speed | One hash call | One scalar mult + one group op |

---

## 🧪 Advanced Variations  
- **Polynomial-commit** (KZG) – constant-size commitment to gigabytes  
- **Inner-product commitments** – Bulletproofs’ heart, no trusted setup  
- **Timelock commitments** – reveal only after *T* blocks (hash-chain + PoW)  
- **Verifiable delay commitments** – force **sequential** work to open

---

## 🔐 Real-World Use-Cases  
🎲 **Coin-flip online** – each party commits, then reveals  
🗳️ **Electronic voting** – voter commits to ballot, later proves inclusion  
💰 **Atomic swaps** – hash-lock = commitment; preimage = reveal  
🎮 **Mental poker** – card shuffling without trusted dealer  
📈 **Zero-knowledge contingent payments** – pay when seller opens valid solution commitment

---

## 🚦 Implementation Checklist  
- [ ] Use **256-bit** (or domain-separated) randomness  
- [ ] Constant-time hash/compare to avoid side-channels  
- [ ] Never reuse *r* across Pedersen commits (breaks binding proof)  
- [ ] Serialize group points in **compressed** form (33 B for Ed25519)  
- [ ] Audit trusted-setup ceremony transcripts (Pedersen/KZG)  
- [ ] Add **range proof** if committing to numeric amounts (Bulletproof)

---

## 🧠 Mini-Quiz (Self-Check)  
1. Can a perfectly hiding scheme be **everlastingly** binding?  
2. Why is *H(m)* alone **never** a secure commitment?  
3. Name one advantage Pedersen has over hash-based when building **confidential transactions**.

*(Answers hide between the lines above.)*