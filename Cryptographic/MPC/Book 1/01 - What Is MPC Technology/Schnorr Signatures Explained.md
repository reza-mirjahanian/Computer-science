# 🔐 Schnorr Signatures Explained

## 🧩 Core Idea

Schnorr signatures are based on **proof of knowledge of a private key** without ever revealing it. They let one party (Alice) prove to another (Bob) that she knows the private key corresponding to a public key.

---

## 👩‍💻 Step 1: Setup

* Alice has:

  * **Private key**: `x`
  * **Public key**: `P = x · G` (where `G` is the generator point on the elliptic curve)
* Bob knows: `P`

Bob wants Alice to prove she really knows `x`.

---

## 🚫 Naive Attempt (Fails)

* Alice sends `x` directly.
* Bob checks if `x · G == P`. ✅
* Problem: Now Bob knows `x` and can impersonate Alice. ❌

---

## 🎭 Blinding with a Nonce

Instead of revealing `x`, Alice blinds it using a random **nonce** `r`:

1. Compute **nonce commitment**:

   ```
   R = r · G
   ```
2. Compute **siglet**:

   ```
   s = r + x   (mod n)
   ```
3. Send `(R, s)` to Bob.

Bob checks if:

```
s · G == R + P
```

---

## ⚠️ Problem: Public Key Cancellation

Alice could cheat by carefully picking `s` and `R` so the public key cancels out.
Solution → Introduce an **unpredictable challenge**.

---

## 🎲 Step 2: Challenge–Response Protocol

1. **Commitment**: Alice sends `R` to Bob.
2. **Challenge**: Bob picks a random number `c`.
3. **Response**:

   ```
   s = r + c·x   (mod n)
   ```
4. **Verification**: Bob checks if

   ```
   s · G == R + c·P
   ```

Since `c` is random and unpredictable, Alice cannot cheat.

✅ Soundness guaranteed.
❌ Still **interactive** (Bob must be present).

---

## 🔄 Step 3: Non-Interactive Schnorr (Fiat–Shamir Transform)

We remove interactivity using a **cryptographic hash function**:

1. Alice computes:

   ```
   c = H(R || message)
   s = r + c·x
   ```
2. Signature = `(R, s)`
3. Verification (anyone can do it):

   ```
   s · G == R + c·P
   ```

   with

   ```
   c = H(R || message)
   ```

---

## 📦 Final Schnorr Signature Scheme

* **Signature size**: 64 bytes → `(R, s)`
* **Properties**:

  * Non-interactive ✅
  * Secure ✅
  * Message-bound ✅ (cannot reuse signature on another message)
  * Publicly verifiable ✅

---

## 📝 Quick Recap

* Blind the private key with a nonce.
* Prevent cancellation with a random challenge.
* Remove interaction using **Fiat–Shamir**.
* Bind challenge to message to prevent replay.

👉 Result: A **secure, efficient, and elegant** digital signature protocol.

---

