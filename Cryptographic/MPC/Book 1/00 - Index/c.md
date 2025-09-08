
A  Architecture & Foundations  
A1  Cryptographic Primitives  
 A1.1  Threshold Cryptography  
  A1.1.1  Shamir Secret Sharing (SSS)  
  A1.1.2  Feldman & Pedersen VSS  
  A1.1.3  Proactive Refresh & Reshare  
 A1.2  Digital Signature Schemes  
  A1.2.1  ECDSA – low-level math, RFC-6979, malleability  
  A1.2.2  EdDSA / Schnorr – linearity, batch verification  
  A1.2.3  BLS – pairing curves, aggregation, rogue-key defense  
 A1.3  Commitment Schemes & ZKPs  
  A1.3.1  Pedersen, ElGamal, KZG  
  A1.3.2  Sigma protocols, Bulletproofs, zk-SNARK/zk-STARK  
A2  Multi-Party Computation Basics  
 A2.1  Adversary Models (semi-honest, malicious, covert, UC)  
 A2.2  Secret Sharing vs. Garbled Circuits vs. Homomorphic  
 A2.3  MPC Communication Patterns (broadcast, point-to-point, mesh)  
 A2.4  Round Complexity & Offline/Online Phases  

B  MPC Wallet Protocol Design  
B1  Key Generation (DKG)  
 B1.1  Interactive DKG (Gennaro-Goldfeder)  
 B1.2  Non-Interactive DKG via VSS & KGC  
 B1.3  Refresh & Rotation Schedules  
B2  Signing Protocols  
 B2.1  Threshold ECDSA (GG20, Lindell-Nof, CGGMP21)  
 B2.2  Threshold EdDSA (FROST, MuSig2, ROAST)  
 B2.3  Threshold BLS (SimpleTSig, BLS-TS)  
B3  Accountability & Identifiable Abort  
 B3.1  Fault Attribution Techniques  
 B3.2  Slashing & Audit Logs  
B4  Batch & Aggregate Operations  
 B4.1  Multi-input & Multi-output Transactions  
 B4.2  Gas Optimization Strategies  

C  Security Analysis & Threat Modeling  
C1  UC-Security & Simulation Paradigm  
C2  Replay & Fork Attacks Across Chains  
C3  Nonce-Misuse & Entropy Failures  
C4  Side-Channel & Timing Leakage  
C5  Quantum Threats & Migration Paths (CRYSTALS-Dilithium)  

D  Chain-Specific Integration  
D1  Bitcoin  
 D1.1  SegWit, Taproot, PSBT v2, Miniscript  
 D1.2  OP_CAT & future covenant discussion  
D2  Ethereum  
 D2.1  EIP-155, 4337 (Account Abstraction), 1271 (isValidSignature)  
 D2.2  MPC inside Smart-Contract Wallets  
D3  Cosmos / Tendermint  
 D3.1  Amino & Protobuf Signing  
 D3.2  Cross-Chain IBC Key Governance  
D4  Solana  
 D4.1  Ed25519 Program & Transaction Format  
D5  Polkadot / Substrate  
 D5.1  SR25519 & Batch Utility Pallet  

E  Networking & Transport Layer  
E1  P2P Overlay Choices (libp2p, Tor, QUIC)  
E2  End-to-End Encryption (Noise Framework, Double-Ratchet)  
E3  NAT Traversal & Hole-Punching  
E4  Reliability & Retry Logic (gRPC, Message-Queues)  

F  Key Management & Storage  
F1  Hardware Security Modules (HSMs) & TEEs  
F2  Mobile Secure Elements (Secure Enclave, StrongBox)  
F3  Cloud KMS Integration with MPC (enclave-signer pattern)  
F4  Social Recovery & Guardian Schemes  
F5  Backup & Disaster Recovery Playbooks  

G  Governance & Compliance  
G1  Multi-Institution Policies (m-of-n Board Sign-off)  
G2  Travel Rule & FATF Guidance  
G3  GDPR & Right-to-be-Forgotten vs. Immutable Keys  
G4  SOC-2, ISO-27001, NIST-800-57 Mapping  

H  Performance Engineering  
H1  Benchmarking Metrics (TPS, latency, CPU/RAM)  
H2  Parallelization & GPU Acceleration  
H3  Caching & Pre-Computation (Beaver triples, presignatures)  
H4  Load-Balancing & Horizontal Scaling Clusters  

I  DevOps & Deployment  
I1  Container Hardening (distroless, gvisor)  
I2  CI/CD & Deterministic Builds (Gitian, Nix, SLSA)  
I3  Secrets Management (Vault, Sealed-Secrets)  
I4  Monitoring, Alerting, SLA/SLO Definitions  

J  SDK & API Design  
J1  Language Bindings (Rust, Kotlin-Swift, TypeScript, Dart)  
J2  gRPC vs. REST vs. GraphQL Trade-offs  
J3  Stateless vs. Stateful Coordinator Modes  
J4  Developer UX (mnemonics, address-book, fee-suggestion)  

K  Testing & Formal Verification  
K1  Unit & Property-Based Testing (proptest, hypothesis)  
K2  MPC Protocol Model-Checking (Tamarin, ProVerif)  
K3  Symbolic Execution & Fuzzing (Echidna, Foundry)  
K4  Pen-Testing & Red-Team Playbooks  

L  Real-World Case Studies  
L1  Enterprise Custody (Fireblocks, Copper, BitGo)  
L2  Retail Self-Custody (ZenGo, Argent, UniPass)  
L3  Cross-Chain Bridges & MPC Validators  
L4  DeFi Treasury Management (m-of-n DAO Signers)  

M  Advanced Topics & Research Frontier  
M1  Post-Quantum Threshold Signatures  
M2  Homomorphic Signatures & Verifiable Delay Functions  
M3  Witness Encryption & One-shot Threshold Schemes  
M4  MPC inside ZK-Rollups (zk-SNARK signing)  
M5  AI-Governed Key Policies & Dynamic Committees  

N  Appendices  
N1  Mathematical Notation & Conventions  
N2  Curve Parameters & Test Vectors  
N3  RFC & BIP Reference Quick-Lookup  
N4  Open-Source Code Links (GitHub repos, crates, npm)