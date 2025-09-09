
# 🔐 MPC Protocol Comparison

| **Protocol**                                                                  | **Year / Origin** | **Security Model**                               | **Adversary Type**                             | **Efficiency**                                          | **Key Features**                                                                                                                                                                            | **Limitations**                                                                                                                                  |
| ----------------------------------------------------------------------------- | ----------------- | ------------------------------------------------ | ---------------------------------------------- | ------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| **GMW (Goldreich–Micali–Wigderson)**                                          | 1987              | Semi-honest (original), extensions for malicious | Semi-honest (active secure via cut-and-choose) | Moderate, circuit-based (Boolean circuits)              | - First general MPC protocol<br>- Works over **Boolean circuits**<br>- Simple concept: secret sharing each wire                                                                             | - High overhead for malicious security<br>- Bit-level granularity → inefficient for arithmetic-heavy tasks                                       |
| **BGW (Ben-Or–Goldwasser–Wigderson)**                                         | 1988              | Information-theoretic (perfect) security         | Active / malicious (up to ⌊n/3⌋ corruption)    | Polynomial-time, over **arithmetic circuits**           | - Uses **Shamir Secret Sharing**<br>- Tolerates malicious adversaries<br>- Perfect security in honest majority setting                                                                      | - Requires **n ≥ 3t+1** for malicious<br>- Expensive in practice due to polynomial interpolation<br>- Communication-heavy                        |
| **SPDZ (a.k.a. SPDZ / MASCOT family)**                                        | 2012              | Computational security (based on FHE + MACs)     | Malicious                                      | Very efficient online phase (after heavy preprocessing) | - Separates **offline (preprocessing)** and **online** phase<br>- Based on **MAC-checks** for correctness<br>- Scales well for arithmetic circuits<br>- Practical for large MPC deployments | - Expensive preprocessing (requires somewhat homomorphic encryption)<br>- Strong cryptographic assumptions (e.g., DDH, LWE depending on variant) |
| **MASCOT (Multiparty Arithmetic Secure Computation with Oblivious Transfer)** | 2016              | Computational security (OT-based)                | Malicious                                      | Faster than classical SPDZ preprocessing                | - Generates multiplication triples efficiently using **Oblivious Transfer**<br>- Avoids reliance on fully homomorphic encryption<br>- Compatible with SPDZ online phase                     | - Still needs large OT extension<br>- Preprocessing still heavy (but better than SPDZ)<br>- Security depends on OT assumptions                   |

---

# ⚡ Key Insights

* **GMW** → Boolean circuit focus, historic foundation, simple but inefficient for large arithmetic tasks.
* **BGW** → First robust *information-theoretic secure* protocol; good for theory, but expensive in practice.
* **SPDZ** → Introduced **offline/online split**; practical large-scale MPC protocol, heavy cryptography in preprocessing.
* **MASCOT** → Optimized SPDZ-style preprocessing with OT, makes **SPDZ practical at scale**.

---

👉 Think of it this way:

* **GMW / BGW** are **classics** — foundational but impractical today except in research/teaching.
* **SPDZ / MASCOT** are **modern workhorses** — powering real-world MPC deployments in finance, blockchain, and privacy-preserving ML.

---

