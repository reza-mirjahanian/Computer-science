# Commitment Schemes — From Basics to Expert Mastery

## 1) Core Idea & API

**Goal:** Commit to a value now, reveal it later, with two guarantees:

* **Hiding:** The commitment leaks (essentially) nothing about the value.
* **Binding:** After committing, the committer can’t change the value.

**Algorithms (non-interactive):**

* **Setup(λ)** → `pp` (public params, optional)
* **Commit(pp, m)** → `(C, d)` where `C` is the commitment and `d` is *decommitment* (opening randomness)
* **Verify(pp, m, d, C)** → `true/false`

> Many schemes are *non-interactive* (single message), but some interactive variants exist (e.g., bit-commitment protocols).

---

## 2) Security Notions (precise but compact)

* **Hiding:**

  * *Perfect:* `C` reveals **zero** info about `m` even to unbounded adversaries.
  * *Statistical:* Negligible statistical distance from uniform/indistinguishable.
  * *Computational:* Indistinguishable for probabilistic poly-time adversaries (assumes hardness, e.g., collision resistance).

* **Binding:**

  * *Perfect:* No openings to two different messages exist.
  * *Statistical:* Probability of two valid openings negligible even vs. unbounded adversaries.
  * *Computational:* Infeasible for PPT adversaries.

* **Tight variants & extras:**

  * **Non-malleability:** Cannot transform a seen commitment to a related message commitment.
  * **Extractability (for proofs):** A simulator can extract `m` from `C`.
  * **Equivocability (trapdoor):** With trapdoor, simulator can open `C` to *any* `m`. Useful in simulation-based proofs (UC).

---

## 3) Canonical Constructions (mental map)

| Scheme                   | Commitment `C`          |                   Hiding | Binding |                Homomorphic? | Assumption           | Sizes (typical)            |                               |         |
| ------------------------ | ----------------------- | -----------------------: | ------: | --------------------------: | -------------------- | -------------------------- | ----------------------------- | ------- |
| **Hash-based**           | \`C = H(r               |                          |    m)\` |                       Comp. | Comp.                | No (generally)             | Collision/preimage resistance | 32–64 B |
| **Pedersen (EC group)**  | `C = r·G + m·H`         |              **Perfect** |   Comp. |  **Additively** homomorphic | Discrete log         | 32–48 B point              |                               |         |
| **RSA-group Pedersen**   | `C = g^m h^r mod N`     |              **Perfect** |   Comp. |         Additive (via mul.) | Strong RSA/Factoring | 256–384 B                  |                               |         |
| **Merkle (vector)**      | root of Merkle tree     |                    Comp. |   Comp. |        Positional inclusion | Collision resistance | 32 B root; proofs O(log n) |                               |         |
| **Lattice-based**        | `C = A·r + G·m + e`     | Stat./Comp. (param dep.) |   Comp. |            Often additively | LWE/RLWE             | 100s B–KB                  |                               |         |
| **Polynomial (KZG/IPA)** | Pairing/IPA commitments |                    Comp. |   Comp. | Commit to poly; eval proofs | Pairings; DLOG       | 32–48 B + small proof      |                               |         |

**Trade-off mantra:** *Perfect hiding ⇒ computational binding* (e.g., Pedersen). *Statistical binding ⇒ computational/less-than-perfect hiding* (hash trees, etc.).

---

## 4) Hash-Based Commitments (baseline workhorse)

**Construction:** Let `H` be a collision-resistant hash.
`Commit(m)`:

1. Sample random `r` (≥128 bits).
2. Output `C = H( tag || len(m) || r || m )`, `d = (r, len(m))`.

`Verify(m, d=(r,ℓ), C)`: Recompute; check equality.

**Pros:** Simple, fast, standard-library friendly.
**Cons:** No homomorphism; purely computational security; careful with length/domain separation.

### Go (SHA-256)

```go
package main

import (
	"crypto/rand"
	"crypto/sha256"
	"encoding/binary"
	"fmt"
)

const domain = "COMMIT:v1"

func Commit(m []byte) (C [32]byte, r []byte, err error) {
	r = make([]byte, 32) // 256-bit nonce
	if _, err = rand.Read(r); err != nil {
		return C, nil, err
	}
	// length-prefix m (binding to length)
	lenBuf := make([]byte, 8)
	binary.BigEndian.PutUint64(lenBuf, uint64(len(m)))

	h := sha256.New()
	h.Write([]byte(domain))
	h.Write(lenBuf)
	h.Write(r)
	h.Write(m)
	copy(C[:], h.Sum(nil))
	return C, r, nil
}

func Verify(m, r []byte, C [32]byte) bool {
	lenBuf := make([]byte, 8)
	binary.BigEndian.PutUint64(lenBuf, uint64(len(m)))

	h := sha256.New()
	h.Write([]byte(domain))
	h.Write(lenBuf)
	h.Write(r)
	h.Write(m)
	var c2 [32]byte
	copy(c2[:], h.Sum(nil))
	return c2 == C
}

func main() {
	m := []byte("hello commitments")
	C, r, err := Commit(m)
	if err != nil { panic(err) }
	fmt.Printf("C=%x\n", C)
	fmt.Println("verify:", Verify(m, r, C))
	fmt.Println("bad open:", Verify([]byte("bye"), r, C))
}
```

### Rust (SHA-256)

```rust
use rand::RngCore;
use sha2::{Digest, Sha256};

const DOMAIN: &[u8] = b"COMMIT:v1";

fn commit(m: &[u8]) -> ([u8;32], Vec<u8>) {
    let mut r = vec![0u8; 32];
    rand::thread_rng().fill_bytes(&mut r);

    let mut len_buf = (m.len() as u64).to_be_bytes().to_vec();
    let mut hasher = Sha256::new();
    hasher.update(DOMAIN);
    hasher.update(&len_buf);
    hasher.update(&r);
    hasher.update(m);
    let hash = hasher.finalize();

    let mut C = [0u8;32];
    C.copy_from_slice(&hash);
    (C, r)
}

fn verify(m: &[u8], r: &[u8], C: [u8;32]) -> bool {
    let mut hasher = Sha256::new();
    hasher.update(DOMAIN);
    hasher.update(&(m.len() as u64).to_be_bytes());
    hasher.update(r);
    hasher.update(m);
    hasher.finalize().as_slice() == C
}

fn main() {
    let m = b"hello commitments";
    let (C, r) = commit(m);
    println!("C={:x?}", C);
    assert!(verify(m, &r, C));
    assert!(!verify(b"bye", &r, C));
}
```

### C++ (BLAKE2b via libsodium)

```cpp
#include <sodium.h>
#include <iostream>
#include <vector>
#include <cstring>

static const char* DOMAIN = "COMMIT:v1";

std::vector<unsigned char> commit(const std::vector<unsigned char>& m,
                                  std::vector<unsigned char>& r_out) {
    r_out.resize(32);
    randombytes_buf(r_out.data(), r_out.size());
    unsigned char C[crypto_generichash_BYTES];

    crypto_generichash_state st;
    crypto_generichash_init(&st, nullptr, 0, sizeof(C));
    crypto_generichash_update(&st, (const unsigned char*)DOMAIN, std::strlen(DOMAIN));
    uint64_t len = htobe64(m.size());
    crypto_generichash_update(&st, (unsigned char*)&len, sizeof(len));
    crypto_generichash_update(&st, r_out.data(), r_out.size());
    crypto_generichash_update(&st, m.data(), m.size());
    crypto_generichash_final(&st, C, sizeof(C));

    return std::vector<unsigned char>(C, C + sizeof(C));
}

bool verify(const std::vector<unsigned char>& m, const std::vector<unsigned char>& r,
            const std::vector<unsigned char>& C) {
    std::vector<unsigned char> r2;
    auto C2 = commit(m, r2); // recompute but uses fresh r; NOT OK
    // Fix: we need a "deterministic" recompute using given r
    unsigned char out[crypto_generichash_BYTES];
    crypto_generichash_state st;
    crypto_generichash_init(&st, nullptr, 0, sizeof(out));
    crypto_generichash_update(&st, (const unsigned char*)DOMAIN, std::strlen(DOMAIN));
    uint64_t len = htobe64(m.size());
    crypto_generichash_update(&st, (unsigned char*)&len, sizeof(len));
    crypto_generichash_update(&st, r.data(), r.size());
    crypto_generichash_update(&st, m.data(), m.size());
    crypto_generichash_final(&st, out, sizeof(out));
    return sodium_memcmp(out, C.data(), C.size()) == 0;
}

int main() {
    if (sodium_init() < 0) return 1;
    std::vector<unsigned char> m = {'h','i'};
    std::vector<unsigned char> r;
    auto C = commit(m, r);
    std::cout << "verify: " << verify(m, r, C) << "\n";
    std::vector<unsigned char> m2 = {'b','y','e'};
    std::cout << "bad: " << verify(m2, r, C) << "\n";
}
```

**Edge cases to handle:**

* **Nonce size:** ≥128 bits; never reuse `r` for different messages.
* **Length binding:** Always prefix length (or domain-separated TLV).
* **Domain separation:** `tag` avoids cross-protocol collisions.
* **Canonicalization:** Commit to a canonical serialized form (e.g., JSON Canonical Form, length-prefixed CBOR).

---

## 5) Pedersen Commitments (elliptic curve)

**Setup:** Cyclic group `⟨G⟩` of prime order `q`, with independent generators `G, H` (no known `α` s.t. `H = α·G`).

**Commit:**
`C = r·G + m·H` where `r ← Z_q` random and `m` is encoded in `Z_q` (e.g., reduce hash-to-scalar of data).

**Verify:** Given `(m, r)`, recompute `C'` and check `C' == C`.

**Properties:**

* **Perfect hiding:** For any fixed `C`, `r` hides `m` uniformly.
* **Computational binding:** Changing `m` requires solving discrete log (`H = α·G`) or finding `r'` s.t. `r'·G + m'·H = C`.
* **Additive homomorphism:**
  `Commit(m1,r1) + Commit(m2,r2) = Commit(m1+m2, r1+r2)`.

> **Generator independence is critical.** Derive `H` via hash-to-curve with a distinct domain.

### Rust (Ristretto/curve25519-dalek)

```rust
use curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT as G;
use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::scalar::Scalar;
use rand::rngs::OsRng;
use sha2::{Sha512, Digest};

// Derive independent H using hash-to-curve
fn derive_H() -> RistrettoPoint {
    let mut h = Sha512::new();
    h.update(b"PEDERSEN:H:v1");
    let bytes = h.finalize();
    RistrettoPoint::hash_from_bytes::<Sha512>(&bytes)
}

// Map arbitrary message to scalar
fn msg_to_scalar(m: &[u8]) -> Scalar {
    let mut h = Sha512::new();
    h.update(b"PEDERSEN:MSG");
    h.update(&(m.len() as u64).to_be_bytes());
    h.update(m);
    Scalar::from_hash(h)
}

struct CommitOut {
    C: RistrettoPoint,
    r: Scalar,
}

fn commit(m: &[u8], H: &RistrettoPoint) -> CommitOut {
    let r = Scalar::random(&mut OsRng);
    let ms = msg_to_scalar(m);
    let C = RistrettoPoint::multiscalar_mul(&[r, ms], &[*G, *H]);
    CommitOut { C, r }
}

fn verify(m: &[u8], r: Scalar, C: &RistrettoPoint, H: &RistrettoPoint) -> bool {
    let ms = msg_to_scalar(m);
    let C2 = RistrettoPoint::multiscalar_mul(&[r, ms], &[*G, *H]);
    C2 == *C
}

fn main() {
    let H = derive_H();
    let m = b"confidential amount 42";
    let out = commit(m, &H);
    assert!(verify(m, out.r, &out.C, &H));
}
```

### C++ (libsodium’s Ristretto255)

```cpp
#include <sodium.h>
#include <vector>
#include <iostream>

void hash_to_scalar(const std::vector<unsigned char>& msg, unsigned char s[crypto_core_ristretto255_SCALARBYTES]) {
    unsigned char h[crypto_generichash_BYTES];
    crypto_generichash(h, sizeof(h), msg.data(), msg.size(), (const unsigned char*)"PEDERSEN:MSG", 12);
    crypto_core_ristretto255_scalar_reduce(s, h);
}

void derive_H(unsigned char H[crypto_core_ristretto255_BYTES]) {
    unsigned char h[crypto_generichash_BYTES];
    crypto_generichash(h, sizeof(h), nullptr, 0, (const unsigned char*)"PEDERSEN:H:v1", 13);
    crypto_core_ristretto255_from_hash(H, h);
}

struct ComOut { unsigned char C[crypto_core_ristretto255_BYTES]; unsigned char r[crypto_core_ristretto255_SCALARBYTES]; };

ComOut commit(const std::vector<unsigned char>& m, const unsigned char H[crypto_core_ristretto255_BYTES]) {
    ComOut out;
    randombytes_buf(out.r, sizeof out.r);

    unsigned char msc[crypto_core_ristretto255_SCALARBYTES];
    hash_to_scalar(m, msc);

    unsigned char rG[crypto_core_ristretto255_BYTES], mH[crypto_core_ristretto255_BYTES];
    crypto_scalarmult_ristretto255_base(rG, out.r);
    crypto_scalarmult_ristretto255(mH, msc, H);
    crypto_core_ristretto255_add(out.C, rG, mH);
    return out;
}

bool verify(const std::vector<unsigned char>& m, const unsigned char r[crypto_core_ristretto255_SCALARBYTES], const unsigned char C[crypto_core_ristretto255_BYTES], const unsigned char H[crypto_core_ristretto255_BYTES]) {
    unsigned char msc[crypto_core_ristretto255_SCALARBYTES];
    hash_to_scalar(m, msc);

    unsigned char rG[crypto_core_ristretto255_BYTES], mH[crypto_core_ristretto255_BYTES], C2[crypto_core_ristretto255_BYTES];
    crypto_scalarmult_ristretto255_base(rG, r);
    crypto_scalarmult_ristretto255(mH, msc, H);
    crypto_core_ristretto255_add(C2, rG, mH);
    return sodium_memcmp(C2, C, crypto_core_ristretto255_BYTES) == 0;
}

int main() {
    if (sodium_init() < 0) return 1;
    unsigned char H[crypto_core_ristretto255_BYTES];
    derive_H(H);
    std::vector<unsigned char> m = {'4','2'};
    auto out = commit(m, H);
    std::cout << "ok=" << verify(m, out.r, out.C, H) << "\n";
}
```

**Edge cases & gotchas:**

* Never choose `H = α·G` with known `α`. Use domain-separated **hash-to-curve**.
* Encode `m` → scalar **deterministically** (hash-to-scalar) to avoid ambiguity.
* Group must be prime order; avoid small-subgroup pitfalls (Ristretto/Decaf solve this).

---

## 6) Merkle / Vector Commitments (commit to many items)

**Idea:** Build a Merkle tree over leaves `mi`. The **root** is the commitment. An **opening** for leaf `mi` is its Merkle path.

**Properties:**

* Efficient openings: `O(log n)` sibling hashes.
* **Binding:** Computational (collision-resistance).
* **Hiding:** Not by default (you reveal presence and positions). To hide, either use salted leaves or wrap in an encryption layer.

### Go (Merkle tree with SHA-256)

```go
package main

import (
	"crypto/sha256"
	"fmt"
)

func h(x ...[]byte) []byte {
	H := sha256.New()
	for _, xi := range x { H.Write(xi) }
	return H.Sum(nil)
}

func merkleRoot(leaves [][]byte) [][]byte {
	if len(leaves) == 0 { return [][]byte{make([]byte, 32)} } // all-zero root
	level := make([][]byte, len(leaves))
	for i, m := range leaves {
		level[i] = h([]byte{0x00}, m) // 0x00 domain sep for leaf
	}
	for len(level) > 1 {
		var next [][]byte
		for i := 0; i < len(level); i += 2 {
			if i+1 == len(level) {
				next = append(next, h([]byte{0x01}, level[i], level[i])) // duplicate last
			} else {
				next = append(next, h([]byte{0x01}, level[i], level[i+1]))
			}
		}
		level = next
	}
	return level // single element: root
}

func main() {
	leaves := [][]byte{[]byte("a"), []byte("b"), []byte("c")}
	root := merkleRoot(leaves)[0]
	fmt.Printf("root=%x\n", root)
}
```

**Inclusion proof verification:** Recompute up the tree using provided siblings; check equals root.

---

## 7) Trapdoor / Equivocable Commitments (simulation-friendly)

**Motivation:** In UC/zero-knowledge, simulators need to open commitments to *chosen* values (equivocation), or extract them (extractability).

* **Trapdoor Pedersen (RSA group):** `C = g^m h^r mod N` with `N` composite. With φ(`N`) (trapdoor), simulator can find alternate `(m', r')` with same `C` (equivocation).
* **Chameleon-hash commitments:** Use collision-finding trapdoor to re-open.

> Use these only with rigorous parameter setup (trusted setup or well-formed modulus).

---

## 8) Homomorphic Commitments & Applications

* **Additive homomorphism** (Pedersen): aggregate commitments and later open to sum without opening each.

  * **Range proofs:** Show `m` in `[0, 2^k)` (Bulletproofs) over Pedersen.
  * **Balance proofs** in confidential transactions (CT).

* **Vector commitments:** Commit to a vector; open any index with short proofs (e.g., KZG vector commitments, Merkle, DARK/IPA).

---

## 9) Polynomial Commitments (expert tier snapshot)

* **KZG (pairings):** Commitment to polynomial `f(X)` is a single group element. Open at point `x` with a short proof; verifier uses pairings. Constant-size proofs; trusted setup.
* **IPA-based (no pairings):** Commit using inner-product arguments (logarithmic proof size, no pairings, heavier verify). Used in Bulletproofs-like constructions.

> These are **commit-and-open** over algebraic structures; they underpin many zkSNARKs/Plonkish systems.

---

## 10) Comparisons to Related Concepts

| Concept                           | What it gives                           | How it differs                                                          |
| --------------------------------- | --------------------------------------- | ----------------------------------------------------------------------- |
| **Hash (plain)**                  | Digest of data                          | No *opening randomness*; no hiding (digest reveals hash of message).    |
| **Encryption**                    | Hides message                           | Not binding; ciphertexts can be of many messages (w/o integrity).       |
| **MAC/Signature**                 | Integrity/authenticity                  | Not hiding; also sender-bound.                                          |
| **ZK proof**                      | Prove statement w/o revealing witness   | Commitments often used within ZK as building blocks (commit-and-prove). |
| **Authenticated data structures** | Verifiable queries (e.g., Merkle trees) | Merkle roots are commitments; ADS adds query semantics.                 |

---

## 11) Patterns & Best Practices (production checklists)

* **Serialization:** Commit to canonical byte representation; include *type tags* and *lengths*.
* **Domain separation:** Different protocol steps get different prefixes.
* **Randomness:** Use CSPRNG; avoid nonce reuse; consider `r` size ≥ 128 bits (usually group order size for Pedersen).
* **Side-channels:** Constant-time comparisons, scalar ops; avoid timing leaks in verification.
* **Malleability:** For hash-based, add context to prevent commit reuse across domains.
* **Batching:** Pedersen commitments add homomorphically; verify batch openings efficiently.
* **Multi-message commitments:** Either commit to Merkle root of messages or use multi-base Pedersen `C = r·G + Σ m_i·H_i`.

---

## 12) Commit-and-Prove (CAP) Sketch

* **Goal:** Simultaneously commit to `w` and prove `R(x, w)` without revealing `w`.
* **Construction:** Produce `C = Commit(w; r)` and a ZK proof `π` that “∃(w,r) s.t. C = Commit(w; r) ∧ R(x,w)”.
* **Usage:** Range proofs, set-membership, arithmetic circuits (SNARKs, Bulletproofs).

---

## 13) Edge Cases & Failure Modes (and how to test them)

1. **Reusing randomness (`r`) across messages**
   *Effect:* Hash-based: easier correlation/preimage games; Pedersen: leaks linear relations → may reveal `m`.
   *Test:* Unit test that duplicate `r` across different `m` is flagged in higher layer.

2. **Non-canonical input**
   *Effect:* Two encodings of same logical value open to different results.
   *Fix:* Deterministic canonical encoding (e.g., big-endian fixed width; sorted maps).

3. **Generator correlation in Pedersen**
   *Effect:* If `H = α·G` known, binding breaks.
   *Fix:* Derive `H` via **hash-to-curve** with distinct domain.

4. **Length ambiguity**
   *Effect:* Different tuples `(x,y)` vs `x||y` collisions.
   *Fix:* TLV / length-prefix; structured hashing.

5. **Small-subgroup exposure**
   *Effect:* Invalid-curve attacks.
   *Fix:* Use Ristretto/Decaf or enforce subgroup checks.

6. **Opening proof leakage**
   *Effect:* Reveals more than intended (e.g., position in vector).
   *Fix:* Use hiding wrappers or commit to encryptions.

7. **Committing outside field range**
   *Effect:* For Pedersen, big integers must be reduced mod `q`. Different `m` mod `q` may collide.
   *Fix:* Encode `m` via hash-to-scalar (or split into chunks with multi-base commitments).

---

## 14) Advanced Examples & Edge-Case Tests

### (A) Additive Opening with Pedersen (sum check)

**Goal:** Verify `C_total` opens to `m1+m2` with `r1+r2` without revealing each `mi`.

*Sketch (Rust):*

```rust
// Given (C1, r1, m1), (C2, r2, m2), H
let C_total = C1 + C2;
let r_total = r1 + r2;
let m_total = msg_to_scalar(b"42") + msg_to_scalar(b"58"); // example
assert!(verify(b"42||58-summed", r_total, &C_total, &H)); // practical code: commit to structured sum
```

> In practice you’d commit to structured data and prove constraints in ZK; the homomorphism enables aggregation.

### (B) Multi-Message Pedersen (fixed bases)

Let `H1, H2, …, Hk` be independent generators. Commit to vector `m⃗`:
`C = r·G + Σ m_i·H_i`.
Opens to any subset with partial randomness and reveals only those entries (plus binding to the rest).

### (C) Merkle Inclusion/Exclusion Edge Case (Go)

* Prove a leaf at index `i` with path.
* **Edge case:** odd number of leaves → duplicate last node on level; domain-separate internal vs leaves.

### (D) Hash Commitment With Pre-committed Nonce (C++)

* Sometimes you need deterministic commitments (e.g., recorded `r`) — ensure RNG is injectable and test deterministic vectors.

---

## 15) Quick “Which Scheme Should I Use?” Table

| Need                                                   | Recommended                                                  |
| ------------------------------------------------------ | ------------------------------------------------------------ |
| **Single value, simple API, fast, standard libs**      | Hash-based commitment                                        |
| **Additive homomorphism; used in ZK/range proofs**     | Pedersen over Ristretto/prime-order group                    |
| **Commit to many values with short per-item openings** | Merkle / Vector commitments                                  |
| **Polynomial eval commitments (SNARKs, PLONK)**        | KZG (pairings) or IPA-based (pairing-free)                   |
| **UC/ZK simulation with equivocation**                 | Trapdoor/equivocable commitments (RSA group, chameleon hash) |
| **Post-quantum posture needed today**                  | Lattice-based commitments (LWE/RLWE), Merkle (hash-based)    |

---

## 16) Testing Matrix (what to unit test)

* **Correctness:** `Verify(m, d, Commit(m).C) == true`
* **Soundness (binding):** Fuzz search cannot find `(m≠m', d')` s.t. both verify against same `C`.
* **Hiding sanity:** Empirical indistinguishability: distributions of `C` with fixed `m` vs. random look uniform.
* **API misuse:** Failing tests for reused `r`, non-canonical encodings, missing domain tags.
* **Interoperability vectors:** Fixed test vectors (given `m`, `r`, `C`) for cross-impl checking.

---

## 17) Minimal, End-to-End Examples

### Go — Hash Commit with All Edge Checks

```go
type Opening struct { R []byte; Len uint64 }

func CommitWithOpen(m []byte) (C [32]byte, d Opening, err error) {
    C, r, err := Commit(m)
    if err != nil { return C, Opening{}, err }
    return C, Opening{R: r, Len: uint64(len(m))}, nil
}

func VerifyOpen(m []byte, d Opening, C [32]byte) bool {
    if d.Len != uint64(len(m)) { return false } // length binding
    return Verify(m, d.R, C)
}
```

### Rust — Pedersen with Domain Separation & Hash-to-Scalar

```rust
// already shown above; add explicit domain tags "PEDERSEN:H:v1" and "PEDERSEN:MSG".
```

### C++ — Merkle Proof Verification (outline)

```cpp
// Given leaf m, index i, siblings[], root:
// 1) x = H(0x00 || m)
// 2) For each level j:
//     if (i bit j == 0) x = H(0x01 || x || siblings[j])
//     else               x = H(0x01 || siblings[j] || x)
// 3) return x == root
```

---

## 18) Cheat Sheet (one-page mental recap)

* **API:** `(C, open)` ← Commit; `Verify(m, open, C)`.
* **Basic choice:** Hash-based (simple), Pedersen (homomorphic), Merkle (many items).
* **Hiding/Binding:** Perfect hiding ⇔ computational binding (Pedersen). Hash/Merkle are computational on both.
* **Do not:** reuse randomness, skip domain separation, use non-canonical encodings, or correlated generators.
* **Do:** hash-to-curve for `H`; constant-time ops; TLV/length-prefix; structured hashing.

---

## 19) Micro Benchmarks & Sizes (rules of thumb)

| Scheme                  | Commit time   | Open size           | Verify time                       |
| ----------------------- | ------------- | ------------------- | --------------------------------- |
| Hash (SHA-256)          | \~1–2 µs / KB | 32 B + 32 B nonce   | \~1–2 µs                          |
| Pedersen (ristretto255) | \~60–120 µs   | 32 B point + scalar | \~60–120 µs                       |
| Merkle (n leaves)       | O(n) build    | \~32·log₂n B        | O(log n)                          |
| KZG (degree d)          | ms-level      | \~48 B              | pairings (fast but const factors) |

*(Indicative only; measure in your environment.)*

---

## 20) From Here to Expert Use

* **Commit-and-Prove** with Pedersen + Bulletproofs or Plonkish systems.
* **Batch openings** for vector commitments (accumulate proofs).
* **UC-secure** protocols requiring equivocation/extractability (trapdoor commitments).
* **Post-quantum**: favor hash/Merkle and lattice-based commitments for future-proofing.

> When you wire commitments into larger protocols, always re-check **composition**: domain separation across *every* layer, and match the commitment’s algebra to the proof system you plan to use.
