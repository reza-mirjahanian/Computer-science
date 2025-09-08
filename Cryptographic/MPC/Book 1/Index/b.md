

# 📑 Index – *Developing MPC Wallets in Depth*

## Part I – Foundations

1. **Introduction to MPC Wallets**

   * What is an MPC Wallet?
   * Evolution of Crypto Wallets
   * Why MPC over Traditional Wallets?

2. **Cryptographic Basics**

   * Modular Arithmetic & Groups
   * Elliptic Curve Cryptography (ECC)
   * Public Key Cryptography Refresher
   * Threshold Cryptography Concepts

3. **Security Foundations**

   * Trust Models in Cryptography
   * Adversarial Models (Honest-but-Curious vs Malicious)
   * Secure Multiparty Computation (MPC) Primer

---

## Part II – MPC in Wallets

4. **MPC Protocols for Wallets**

   * Yao’s Garbled Circuits
   * Secret Sharing (Shamir, additive)
   * Homomorphic Encryption in MPC
   * Oblivious Transfer & Zero-Knowledge Proofs

5. **Threshold Signature Schemes (TSS)**

   * Threshold ECDSA
   * Threshold EdDSA
   * Security Guarantees & Trade-offs

6. **Distributed Key Generation (DKG)**

   * DKG Protocols Explained
   * Refreshing Keys (Proactive Security)
   * Comparison: DKG vs Centralized Key Setup

---

## Part III – Wallet Architecture

7. **System Design of MPC Wallets**

   * Client–Server vs Peer-to-Peer
   * Share Storage & Recovery
   * Session Management

8. **Transaction Lifecycle**

   * Key Generation
   * Signing a Transaction
   * Broadcast & Verification

9. **Security Engineering**

   * Attack Vectors (Key Leakage, Rogue Key Attacks)
   * Side-Channel Protections
   * Collusion-Resistance

10. **Performance & Scalability**

    * Computation vs Communication Costs
    * Latency Optimizations
    * Load Balancing in MPC Networks

---

## Part IV – Advanced Topics

11. **Zero Knowledge in MPC Wallets**

    * ZK Proofs for Wallet Integrity
    * Confidential Transactions with MPC+ZK
    * Case Study: zk-SNARKs in Wallet Security

12. **Proactive Security & Refresh Protocols**

    * Offline Proactive Refresh
    * Adaptive Security Considerations

13. **Cross-Chain and Interoperability**

    * Multi-chain MPC Wallets
    * Bridging Assets with MPC
    * MPC in DeFi & Cross-chain Bridges

---

## Part V – Practical Engineering

14. **Implementation Strategies**

    * Language & Library Choices (Go, Rust, Python, C++)
    * Using Cryptographic Primitives Safely
    * Integrating with Blockchain Networks

15. **Testing & Verification**

    * Formal Verification of Protocols
    * Unit & Integration Testing in MPC
    * Fuzzing & Adversarial Testing

16. **Deployment & Real-world Challenges**

    * Key Share Storage (Cloud, HSM, TEEs)
    * Disaster Recovery Plans
    * Compliance & Regulatory Perspectives

---

## Part VI – Case Studies & Future

17. **Case Studies**

    * Zengo Wallet
    * Fireblocks
    * Coinbase MPC

18. **The Future of MPC Wallets**

    * MPC + Hardware Enclaves
    * Post-Quantum MPC Wallets
    * Decentralized Identity & MPC
