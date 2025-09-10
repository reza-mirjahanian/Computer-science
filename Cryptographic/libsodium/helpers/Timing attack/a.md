# Timing attacks in cryptography — complete reference (deep, practical, source-backed)

---

> **Short definition.** A *timing attack* is a side-channel attack that extracts secret information by measuring how long cryptographic or related code takes to run. Any data-dependent variation in execution time — at the algorithm, implementation, compiler, OS, or microarchitecture level — can leak secrets. ([Wikipedia][1])

---

# 1 — Taxonomy (how attackers measure and what they exploit)

**By attacker locality**

* **Local / same host** — attacker runs code on same machine (co-tenant VM, container, local process) and measures fine-grained timing (cache, shared core). Very powerful. ([CS-People][2], [IACR][3])
* **Remote / network** — attacker measures network round-trip times of requests (web servers, TLS handshakes). Noisy but feasible with many samples and clever stats (e.g., Lucky13, Bleichenbacher-style oracles). ([Veracode Docs][4], [tlsfuzzer.readthedocs.io][5])

**By leakage mechanism**

* **Algorithmic / control-flow timing** — branches/loops that depend on secret values (e.g., if(secret) then …).
* **Memory-access / cache timing** — secret-dependent array/table lookups change cache hits/misses (AES T-table attacks, flush+reload, prime+probe). ([IACR][3], [CS-People][2])
* **Speculative/execution timing** — speculative leaks (Spectre-style) expose secret-dependent microarchitectural state. ([Wikipedia][1])
* **Library/protocol oracles** — implementations that respond differently (different error message timing, padding checks) enable remote attacks (Bleichenbacher, Lucky13). ([tlsfuzzer.readthedocs.io][5], [Medium][6])

---

# 2 — Classic real-world examples & research papers (must-read)

* **Cache attacks on AES T-tables** — practical attacks on table-driven AES showing cache collisions leak key bytes. (Bonneau & Mironov / CHES). ([IACR][3])
* **Osvik, Shamir & Tromer — cache timing attacks** — demonstration of cross-process leakage via caches. ([CS-People][2])
* **Lucky13 (TLS MAC timing)** — differential timing in MAC/CBC processing can be exploited in TLS. Implementations mitigated it differently; remote network noise makes exploitation nontrivial. ([Veracode Docs][4], [Medium][6])
* **Bleichenbacher / ROBOT** — RSA PKCS#1 v1.5 padding oracles and timing side channels enabling RSA decryption when padding validity leaks. ([tlsfuzzer.readthedocs.io][5], [Medium][7])

---

# 3 — Core mitigation approaches (what to use and why)

### 3.1 Constant-time programming (primary line of defense)

* **Goal:** ensure execution time is *independent* of secret values.
* **Techniques:** avoid secret-dependent branches; avoid secret-dependent memory indices; use bitwise logic, arithmetic that is time-stable; use table-free or bit-sliced implementations; use hardware instructions that are constant time (e.g., AES-NI for AES). ([bearssl.org][8], [chosenplaintext.ca][9])

**Pros:** When correctly implemented, removes a large class of timing leakage.
**Cons:** Hard to get correct (compilers, microarchitecture), can be slower than naive code.

**Common pitfalls**

* Compiler optimizations can reintroduce branches or transform code into variable-time patterns unless you write very carefully or use compiler barriers. ([LWN.net][10])
* Some CPU instructions are variable time across inputs (division, some CPU string ops). Avoid secrets in those. ([chosenplaintext.ca][9])

### 3.2 Algorithmic/higher-level mitigations

* **Blinding / randomization** — randomize inputs (e.g., RSA blinding) so timing observations do not map to a fixed secret. Good for modular exponentiation.
* **Masking** — split secret into shares processed separately so individual operations leak nothing useful (common in hardware implementations).
* **Constant-time libraries** — use vetted implementations and helpers (libsodium, BearSSL, Intel libraries). For example, libsodium documents `sodium_memcmp()` and uses constant-time compares. ([libsodium][11], [libsodium.gitbook.io][12])

**Pros:** Easier to apply in some cases; blinding is robust for public-key ops.
**Cons:** May add complexity, performance overhead, or not cover all channels (e.g., cache).

### 3.3 System/hardware mitigations

* **Use AES-NI / dedicated crypto hardware** — avoids table lookup implementations and many cache attacks.
* **Microcode / CPU features** — future CPU modes or mitigations that reduce timing variability (some OS/CPU patches). ([LWN.net][10])

**Pros:** Often best performance and security combination.
**Cons:** Requires specific hardware; speculative exec attacks may still leak.

### 3.4 Protocol fixes / error-hiding

* For protocols where different errors leak timing (padding vs MAC), standardize response times and messages (or perform all checks and always take the same time path). Example mitigations after Lucky13. ([Veracode Docs][4], [Amazon Web Services, Inc.][13])

---

# 4 — How to write constant-time code (practical checklist + gotchas)

**Checklist**

1. **Identify secrets** — explicitly document which variables are secret.
2. **Avoid secret-dependent branches** — `if (secret & mask) { … }` is a red flag. Use arithmetic/bit masking to select values.
3. **Avoid secret-dependent indices** — no `array[secret]` lookups. If table lookups are necessary, ensure table access *pattern* is independent (e.g., always read full table into registers or use bit-slicing). ([chosenplaintext.ca][9], [bearssl.org][8])
4. **Use constant-time primitives** — `sodium_memcmp`, `crypto_verify_*` etc. Prefer vetted libraries (libsodium, BearSSL). ([libsodium][11], [libsodium.gitbook.io][12])
5. **Beware compiler optimizations** — use `volatile` carefully, compiler intrinsics, or compiler-provided constant-time helpers; add unit tests asserting timing invariance. ([LWN.net][10])
6. **Prefer hardware crypto** where available (AES-NI, ARM Crypto Extensions).
7. **Test on target platform** — timing depends on CPU, OS, microcode; test on the actual environment.

**Common code patterns (C pseudo)**

* Constant-time compare (simplified):

```c
int ct_compare(const unsigned char *a, const unsigned char *b, size_t n){
    unsigned char r = 0;
    for (size_t i = 0; i < n; i++) r |= a[i] ^ b[i];
    return (int)(1 & ((r - 1) >> 8)); // returns 0 if equal, nonzero otherwise
}
```

But prefer library `sodium_memcmp()` or `crypto_verify_*` instead of homebrew. ([libsodium][11], [libsodium.gitbook.io][12])

**Gotchas**

* Using `memcmp()` for secrets is unsafe — it may early-exit. Use dedicated constant-time compares. ([libsodium][11])
* `volatile` does not guarantee constant-time semantics nor prevent compiler reordering in all cases.
* Floating-point, division, modulo, or variable-latency CPU instructions can reintroduce timing variability. Avoid secrets in such operations. ([chosenplaintext.ca][9])

---

# 5 — Detection & testing (how to find timing leaks)

**Local/precise measurements**

* Use cycle counters (RDTSC on x86; `clock_gettime` with high resolution) to measure microsecond/nanosecond differences.
* Tools: `perf`, `Cachegrind`, `valgrind` (for memory access behavior), hardware PMCs. ([CS-People][2])

**Remote / network timing testing**

* Use many repeated measurements with jitter/statistics; measure distributions (median, variance). Tools like `tlsfuzzer` provide scripts for timing analysis of TLS oracles. ([tlsfuzzer.readthedocs.io][5])

**Automated analysis & formal tools**

* **FaCT** — DSL/compiler to help write constant-time cryptographic code and check it. ([GitHub][14])
* **ct-wasm** — constant-time WebAssembly to enforce non-interference at the type level. ([GitHub][15])

**Practical methodology**

1. Make a test harness that runs operation thousands-to-millions of times with controlled inputs.
2. Use statistical tests to detect differences between secret classes (t-test, Mann–Whitney).
3. For cache attacks, mount prime+probe or flush+reload experiments to observe secret-dependent cache hits. ([CS-People][2], [IACR][3])

---

# 6 — Examples of vulnerable patterns and safer replacements

| Vulnerable pattern                    |                                    Why it's bad | Safer alternative                                                            |
| ------------------------------------- | ----------------------------------------------: | ---------------------------------------------------------------------------- |
| `if (secret & 1) do_x();`             |                 Branch timing depends on secret | Use bitmask to compute both results and select without branch                |
| `memcmp(secret, guess)`               | early return leaks position of first difference | Use `sodium_memcmp()` or constant-time loop                                  |
| Table lookups `S[secret_byte]` in AES |               cache access pattern leaks secret | Use AES-NI or bit-sliced AES; or ensure uniform memory accesses              |
| Different error returns & messages    |                  remote oracles reveal validity | Normalize responses and timings; perform all checks and return generic error |

(Sources: common best practices, libsodium docs, BearSSL guidelines). ([libsodium][11], [bearssl.org][8])

---

# 7 — Pros/cons table of main mitigation strategies

| Strategy                 | Strengths                                         | Weaknesses                                                       |
| ------------------------ | ------------------------------------------------- | ---------------------------------------------------------------- |
| Constant-time code       | Eliminates many timing leak classes at source     | Hard to implement/verify; compiler/CPU may undermine it          |
| Hardware crypto (AES-NI) | Fast, avoids table accesses                       | Requires CPU support; not a silver bullet vs speculative attacks |
| Blinding                 | Very effective for public-key ops (RSA)           | Extra randomness & computation; must be implemented carefully    |
| Masking (hardware)       | Strong against microarchitectural leakage         | Complex; often used in hardware or specialised libs              |
| Protocol-level fixes     | Removes oracles (uniform errors, constant delays) | May add latency; not applicable to all contexts                  |
| Randomized delays        | Can obscure timing for remote attacks             | Difficult to get right; may be bypassed with many samples        |

(Sources: literature, libsodium/crypto libs, practical experience). ([libsodium][11], [IACR][3])

---

# 8 — Implementation resources, libraries and projects (where to copy from)

**Vetted libraries**

* **libsodium** — easy-to-use, documents constant-time helpers like `sodium_memcmp()` and secure memory APIs. Use it for new code. ([libsodium][11], [libsodium.gitbook.io][16])
* **BearSSL** — focuses on small, auditable, constant-time TLS primitives (read its constant-time notes). ([bearssl.org][8])
* **OpenSSL/s2n** — major TLS implementations with many timing mitigations; research their patches (e.g., Lucky13 mitigations in s2n). ([Amazon Web Services, Inc.][13])
* **Intel Crypto Primitives** (GitHub) — CPU-optimized libraries that prefer hardware instructions where possible. ([GitHub][17])

**Research & verification projects**

* **FaCT** — language/compiler for constant-time code. ([GitHub][14])
* **CT-Wasm** — WebAssembly extension for constant-time enforcement. ([GitHub][15])

**Testing tools**

* `tlsfuzzer` timing scripts, `perf`, `Cachegrind`, hardware counters. ([tlsfuzzer.readthedocs.io][5], [CS-People][2])

---

# 9 — Tricky, subtle pitfalls (things that bite you in practice)

* **Network noise hides local timing but not always** — remote attacks require many samples, but high-precision timing information can still be obtained from compromised clients or co-tenants. Lucky13 showed remote feasibility in the lab. ([Medium][6], [Veracode Docs][4])
* **Compiler reordering / intrinsics** — your “branchless” code may be optimized into branches; test the compiled assembly on target CPU. Use compiler flags or intrinsics where appropriate. ([LWN.net][10])
* **Microarchitectural state beyond caches** — TLB, branch predictor, speculative execution, DRAM row buffer timing — all can leak. Fixing cache patterns alone may not suffice. ([Wikipedia][1])
* **Third-party libraries** — dependency might introduce timing oracles; treat any external return codes or error differences as potential leaks. ([tlsfuzzer.readthedocs.io][5])
* **Testing on developer machine ≠ production** — different CPUs, patches, containerization change timing behavior. Always test on production-like hardware. ([LWN.net][10])

---

# 10 — Real-world usage & projects where timing matters

* **TLS implementations** — historically targeted (Lucky13, Padding oracle variants). Use constant-time MAC/padding operations and normalize error handling. ([Veracode Docs][4], [tlsfuzzer.readthedocs.io][5])
* **Password verification** — always use constant-time compare for hashes (libsodium helpers). Timing leak can reveal password prefixes. ([libsodium][11])
* **Authentication tokens / HMAC** — compare signatures in constant time.
* **Secure enclaves & multi-tenant cloud** — co-tenant cache attacks on clouds demonstrated; careful crypto implementation is crucial. ([CS-People][2])

---

# 11 — How to audit existing code for timing issues (practical steps)

1. **Inventory secrets** (keys, passwords, nonces, tokens).
2. **Static scan** for uses of `memcmp`, secret-dependent indexing, `switch`/`if` using secrets.
3. **Compile to assembly** and inspect for branches or variable-latency instructions influenced by secrets.
4. **Microbench**: instrument the code with cycle counters and test many input classes, plot distributions.
5. **Cache analysis**: run prime+probe / flush+reload to see if table index leaks.
6. **Fuzz remote oracles** (if web-facing) using tools like `tlsfuzzer` to detect different server behaviors and timings. ([tlsfuzzer.readthedocs.io][5], [CS-People][2])

---

# 12 — Useful quick references (table)

| Topic                         | Resource                                                                                      |
| ----------------------------- | --------------------------------------------------------------------------------------------- |
| Overview of timing attacks    | Wikipedia (Timing attack). ([Wikipedia][1])                                                   |
| Cache attacks & AES           | Osvik/Tromer paper; Bonneau & Mironov CHES. ([CS-People][2], [IACR][3])                       |
| Constant-time coding guidance | BearSSL constant-time doc; chosenplaintext guide. ([bearssl.org][8], [chosenplaintext.ca][9]) |
| Libraries with CT helpers     | libsodium docs (sodium\_memcmp, secure memory). ([libsodium][11], [libsodium.gitbook.io][16]) |
| Testing TLS timing oracles    | tlsfuzzer timing analysis. ([tlsfuzzer.readthedocs.io][5])                                    |
| Formal/DSL projects           | FaCT, CT-Wasm. ([GitHub][14])                                                                 |

---

# 13 — Quick pragmatic recommendations (what to do today)

* **Use vetted libraries** (libsodium, BearSSL) for crypto primitives. Don’t roll your own. ([libsodium][11], [bearssl.org][8])
* **Use hardware instructions** when available (AES-NI). ([GitHub][17])
* **Replace `memcmp` with `sodium_memcmp()` or crypto\_verify functions** for secret comparisons. ([libsodium][11])
* **Normalize errors and response times** for networked services that validate secrets. ([tlsfuzzer.readthedocs.io][5])
* **Test**: build an automated timing test suite for your critical crypto paths on production-like hardware. ([CS-People][2], [tlsfuzzer.readthedocs.io][5])

---

# 14 — If you must implement constant-time primitives — short style guide

* Keep functions small and auditable.
* Use bitwise operations and arithmetic for selection: `mask = (0 - cond); res = (res & ~mask) | (val & mask)`.
* Avoid secret as array index. If unavoidable, access every slot and conditionally accumulate.
* Document target CPU/ABI and test compiled assembly.
* Run static and dynamic analyses; include timing tests in CI on the target hardware.

---

# 15 — What testing cannot fully guarantee

Even with careful measures, **complete** elimination of timing leakage is extremely hard because timing can leak at many layers (compiler, OS scheduler, microarchitectural features, power, EM). The goal is **risk reduction** — make exploitation infeasible in your threat model (cost, access, precision). ([Wikipedia][1])

---

# 16 — Further reading & papers (selected)

* Osvik, Shamir, Tromer — *Cache attacks and countermeasures: the case of AES.* ([CS-People][2])
* Bonneau & Mironov — *Cache-Collision Timing Attacks Against AES.* ([IACR][3])
* BearSSL constant-time notes. ([bearssl.org][8])
* Libsodium docs for secure helpers. ([libsodium][11], [libsodium.gitbook.io][12])

---

# Next step (advanced topic)

**Formal verification of constant-time properties** — study and apply tools like **FaCT** (constant-time DSL + compiler), **CT-Wasm**, and formal verification approaches that prove non-interference / constant-time properties at the language or IR level. This is the natural next step if you need provable guarantees beyond testing. ([GitHub][14])

---


[1]: https://en.wikipedia.org/wiki/Timing_attack?utm_source=chatgpt.com "Timing attack"
[2]: https://cs-people.bu.edu/tromer/papers/cache.pdf?utm_source=chatgpt.com "Cache Attacks and Countermeasures: the Case of AES"
[3]: https://www.iacr.org/archive/ches2006/16/16.pdf?utm_source=chatgpt.com "Cache-Collision Timing Attacks Against AES"
[4]: https://docs.veracode.com/r/prevent-ssl-lucky13?utm_source=chatgpt.com "Prevent SSL LUCKY13 attacks"
[5]: https://tlsfuzzer.readthedocs.io/en/latest/timing-analysis.html?utm_source=chatgpt.com "Timing analysis — tlsfuzzer 0.0.0 documentation"
[6]: https://medium.com/%40c0D3M/lucky-13-attack-explained-dd9a9fd42fa6?utm_source=chatgpt.com "Lucky 13 Attack Explained"
[7]: https://medium.com/%40c0D3M/bleichenbacher-attack-explained-bc630f88ff25?utm_source=chatgpt.com "Bleichenbacher Attack Explained"
[8]: https://bearssl.org/constanttime.html?utm_source=chatgpt.com "Constant-Time Crypto"
[9]: https://www.chosenplaintext.ca/articles/beginners-guide-constant-time-cryptography.html?utm_source=chatgpt.com "A beginner's guide to constant-time cryptography"
[10]: https://lwn.net/Articles/921511/?utm_source=chatgpt.com "Constant-time instructions and processor optimizations"
[11]: https://doc.libsodium.org/helpers?utm_source=chatgpt.com "Helpers"
[12]: https://libsodium.gitbook.io/doc/internals?utm_source=chatgpt.com "Internals - Libsodium documentation - GitBook"
[13]: https://aws.amazon.com/blogs/security/s2n-and-lucky-13/?utm_source=chatgpt.com "s2n and Lucky 13 | AWS Security Blog"
[14]: https://github.com/PLSysSec/FaCT?utm_source=chatgpt.com "PLSysSec/FaCT: Flexible and Constant Time Programming ..."
[15]: https://github.com/PLSysSec/ct-wasm?utm_source=chatgpt.com "PLSysSec/ct-wasm: Constant-Time WebAssembly"
[16]: https://libsodium.gitbook.io/doc/memory_management?utm_source=chatgpt.com "Secure memory | Libsodium documentation - GitBook"
[17]: https://github.com/intel/cryptography-primitives?utm_source=chatgpt.com "Intel® Cryptography Primitives Library is a secure, fast and ..."
