# Non-Interactive Signing vs Interactive Rounds in MPC

When multiple parties jointly compute a signature without revealing their private shares, MPC-based signing protocols can either require live, back‐and‐forth communication (“interactive rounds”) or enable signing with minimal or no online interaction (“non‐interactive signing”). Understanding the trade‐offs helps choose the right design for latency, security, and deployment constraints.

---

## What Are Interactive Rounds?

Interactive signing protocols unfold in a sequence of message exchanges between all participants. Each “round” consists of every party sending one message, typically:
1. **Commitment Exchange**  
   Parties share commitments to randomness or partial computations.
2. **Challenge/Response**  
   Based on received commitments, they exchange challenges and auxiliary data.
3. **Aggregation**  
   Final partial signatures are combined into the complete signature.

The total round count equals the number of sequential exchanges needed to preserve security properties. For example, classic threshold ECDSA protocols often require 6–9 online rounds to generate a signature, leading to higher latency if parties are geographically dispersed or operating behind network constraints.

### Pros and Cons of Interactive Rounds

- Pros  
  - Strong, composable security in the Universal Composability (UC) framework  
  - Can adaptively resist malicious behavior with on‐the‐fly checks  

- Cons  
  - High latency when parties have high network latency  
  - Complex coordination required for every signing operation  
  - Harder to support air‐gapped or cold vault setups  

---

## What Is Non‐Interactive Signing?

Non‐interactive signing separates most cryptographic work into an offline (preprocessing) phase. During this phase, parties:
1. Generate and share random “signing tokens” or multiplication triples  
2. Store these tokens locally until a message needs signing  

When the message arrives, participants use precomputed tokens to produce their partial signatures in a single burst of computation—and often a single communication step—rather than multiple interactive rounds. In some designs, all necessary messages are embedded into zero‐knowledge proofs or non‐interactive proofs of correctness, eliminating live interaction altogether.

### Example: 4 Rounds with 3 Offline

Canetti, Makriyannis, and Peled’s threshold ECDSA protocol reduces the online signing stage to just **1** interactive round by pushing three of its four total rounds into a preprocessing stage. This effectively yields a non‐interactive signing experience when the message is known, because no extra exchanges are required during the live phase.

### Ultra‐Low Latency via 1 Online Round

Fireblocks’ MPC‐CMP protocol pushes this further: it requires exactly **1** round for the entire signing operation, delivering up to **8×** faster transaction signing compared to standard 9‐round protocols. By supporting a cold‐storage share and a single interactive step, it bridges the gap between high security and enterprise speed demands.

---

## Side‐by‐Side Comparison

| Aspect                       | Interactive Rounds                         | Non‐Interactive Signing                                      |
|------------------------------|--------------------------------------------|--------------------------------------------------------------|
| Online Latency               | Proportional to number of rounds           | Minimal (often 1 round or zero if proofs are pre‐packaged)   |
| Preprocessing Requirement    | Typically none                             | Significant token or triple generation upfront               |
| Cold/Hot Wallet Compatibility| Harder (requires live links)               | Easier (offline share can remain air‐gapped)                 |
| Security Model               | UC‐secure with adaptive checks             | UC‐secure but relies on precomputed tokens’ integrity        |
| Implementation Complexity    | Moderate to high                           | Higher (managing offline tokens, zero‐knowledge proofs)      |

---

## When to Choose Which?

- **Interactive Rounds** are preferable if you prioritize:
  - On‐the‐fly security checks against malicious behavior  
  - Simpler key management (no large preprocessing storage)  
  - Environments where network latency is low and parties stay online  

- **Non‐Interactive Signing** excels when you need:
  - Ultra‐low signing latency or single‐step signing SLAs  
  - Cold wallet or air‐gapped setups with minimal online exposure  
  - Bulk signing where preprocessing amortizes over many signatures  

---

## Looking Ahead

Beyond ECDSA, these principles apply to any threshold signature or MPC‐based cryptosystem (e.g., Schnorr, BLS). As zero‐knowledge proof tech matures, we’ll likely see more protocols embedding entire interactive phases into succinct proofs, pushing toward truly zero‐round signing experiences in permissioned and permissionless settings alike.

**Further reading:**
- Explore the UC framework’s treatment of online vs. preprocessing rounds in threshold signatures.  
- Compare MPC‐CMP against other one‐round MPC proposals in DAC conferences.  
- Investigate emerging Schnorr‐based MPC schemes with post‐quantum resilience.