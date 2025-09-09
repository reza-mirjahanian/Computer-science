| Aspect | ECDSA | EdDSA |
|---|---|---|
| **Full name** | Elliptic-Curve Digital Signature Algorithm | Edwards-curve Digital Signature Algorithm |
| **Underlying curve** | Short Weierstrass form (e.g. secp256k1, P-256) | Twisted Edwards form (e.g. Ed25519, Ed448) |
| **Random nonce?** | YES – a fresh high-entropy *k* is mandatory for every signature | NO – the “nonce” is *r = H(secretKey ‖ message)*, so the same (key, msg) always yields the **same** signature |
| **Signature form** | *(r, s)* pair, both integers mod *n* | *(R, s)* where *R* is a curve point encoded as 32 B |
| **Side-channel resistance** | Needs extra blinding / constant-time code | Designed to be constant-time & fast without counter-measures |
| **Nonce-reuse risk** | Re-using *k* reveals the private key instantly | Impossible by construction |
| **Performance** | One modular inverse at sign time | No inverse, only fast point ops; Ed25519 ≈ 2× faster on many CPUs |
| **Key & sig size** | 32 B priv, 64 B pub (secp256k1), 64 B sig | 32 B priv, 32 B pub (Ed25519), 64 B sig |
| **Aggregation / multi-party** | Not natively supported (needs extra work) | Schnorr heritage ⇒ native support for key & signature aggregation |
| **Standardisation** | NIST FIPS 186-4, widely deployed in TLS, X.509, Bitcoin, Ethereum | RFC 8032, used in TLS 1.3, SSH (Ed25519), Signal, Tor, modern blockchains (Cardano, Stellar) |
| **Security margin** | 128-bit (P-256) or 256-bit (P-521) | 128-bit (Ed25519) or 224-bit (Ed448) |
| **Implementation complexity** | Easy to get wrong (RNG, inversion, DER) | One small self-contained file in most libs |

**Rule of thumb**  
- Need compatibility with legacy / Bitcoin → ECDSA  
- Building something new → choose EdDSA/Ed25519 for speed, safety and simplicity

: “ECDSA signatures change each time … EdDSA signatures do not change … EdDSA uses the Schnorr signature method … naturally supports … aggregation.”  
: “EdDSA … resistant to timing attacks … deterministic … ECDSA … requires random nonce … vulnerable if not done properly.”  
: “EdDSA … faster … no random number generation … smaller key sizes … built-in protection against side-channel attacks.”