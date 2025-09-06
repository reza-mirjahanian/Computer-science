# Distributed Key Generation (DKG) and a New Perspective

## Traditional Approach: Threshold Secret Sharing

A classic cryptographic primitive is **threshold secret sharing**, parameterized by:
- **n**: Total number of shares
- **t + 1**: Minimum number of shares needed to reconstruct the secret

> 🔒 Example: In **Shamir’s Secret Sharing**, a secret is embedded as the constant term of a polynomial of degree *t*. Any *t+1* points on the polynomial can reconstruct it; fewer reveal nothing.

---

## The Need for Distributed Key Generation (DKG)

In decentralized settings (e.g., consensus protocols, threshold wallets), we often want:
- No single party to know the entire secret
- A way to *generate* secret shares among mutually distrusting parties

**DKG** is an MPC protocol where:
- Each party ends up with a share *Zᵢ* of a secret *Z*
- Public keys are derived deterministically from these shares
- The secret remains hidden from adversaries

---

## Asynchronous Networks & High-Threshold DKG

- We operate in **asynchronous networks** where message delays can be arbitrary.
- **High-threshold DKG** means the polynomial degree *L* is greater than the number of faulty nodes *t*.

### Why High Thresholds?
- Better fault tolerance
- Improved privacy in threshold encryption
- Enhanced efficiency in consensus protocols

---

## Classic DKG Template (Inefficient for High Thresholds)

1. Each party shares a secret using **Verifiable Secret Sharing (VSS)**
2. Parties run a **consensus protocol** to decide which VSS instances were correct
3. The final secret is the **sum** of the accepted secrets
4. Each party sums its shares to get its final share

⚠️ **Problem**: Using high-degree polynomials in VSS is computationally expensive and bandwidth-heavy.

---

## A New Perspective: Distributed Polynomial Sampling

> 💡 Instead of building DKG from VSS and summing, think of it as:  
> **“Sampling a random polynomial and giving each party one point on it.”**

### Ideal Functionality for DKG:
- Samples a random polynomial *G(X)* of degree *L*
- Secret *Z* is the constant term *G(0)*
- Each party *i* receives *G(i)* as its share

---

## Efficient High-Threshold DKG via Randomness Extraction

### Step 1: Sample Random Coefficients
- A degree-*L* polynomial requires *L+1* random coefficients
- Each party shares **two secrets** via VSS (instead of one)
- Use a **randomness extractor** to derive *t+1* random values from the VSS outputs

### Step 2: Construct the Polynomial
- Define *Z(X)* using the extracted coefficients
- Each party now holds **secret shares of the coefficients**

### Step 3: Compute Evaluation Points
- Each party **locally computes** its share of *Z(i)* for all *i*
- Parties exchange these shares via **one round of communication**
- Each party reconstructs its final share *Z(i)* from received shares

---

## Advantages of This Approach

- ✅ **Efficiency**: Performance comparable to low-threshold DKG
- ✅ **Flexibility**: Polynomial degree *L* is independent of the number of parties
- ✅ **Non-interactive handoff**: Enables proactive secret sharing and DKG-as-a-service

---

## Performance

- ~30% overhead in computation and bandwidth compared to standard DKG
- Supports arbitrary polynomial degrees without significant efficiency loss

---

## Applications

- Threshold signatures
- Threshold encryption
- Randomness beacons
- Secure multi-party computation (MPC)
- Proactive secret sharing
- Distributed key generation as a service