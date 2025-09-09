# Communication Models in MPC: Synchronous vs. Asynchronous

Secure multi-party computation (MPC) protocols are designed to allow a set of parties to compute a function on private inputs without revealing those inputs. A fundamental design choice is the **communication model**—the assumptions made about how and when messages are delivered. Broadly, MPC research distinguishes between two main models:

1. **Synchronous**  
2. **Asynchronous**

Below we unpack what each model means, how they differ, and why the choice matters for both security and performance.

---

## 1. Synchronous Communication Model

### 1.1 Definition and Assumptions
- A **global clock** is available to all parties.  
- Every message sent by an honest party is guaranteed to arrive within a **known, fixed bound Δ**.  
- Protocol proceeds in **rounds**: in round *r*, each party sends messages, then waits until Δ has elapsed, then moves to round *r*+1.

### 1.2 Security Perks
- **Optimal resilience**: up to *t* < *n*/2 malicious parties can be tolerated (information-theoretic) or *t* < *n*/2 computationally bounded (cryptographic).  
- **Guaranteed input inclusion**: every honest party’s input can be used because you can always wait long enough to hear from at least *n–t* parties.

### 1.3 Practical Drawbacks
- **Timeout brittleness**: if Δ is underestimated even once (e.g., sudden network congestion), the protocol can **fail to terminate** or **lose security**.  
- **Latency floor**: performance is capped by the worst-case Δ, even when the network is fast.  
- **Poor match for the open Internet**, where jitter and temporary outages are routine.

---

## 2. Asynchronous Communication Model

### 2.1 Definition and Assumptions
- **No global clock**, no upper bound on message delay.  
- The **adversary controls the scheduler**: it can delay any message arbitrarily, as long as every message is **eventually** delivered (liveness).  
- Parties advance when they have “enough” messages, not when a timer fires.

### 2.2 Security Realities
- **Weaker resilience**: perfect security possible only if *t* < *n*/4; cryptographic or statistical security up to *t* < *n*/3.  
- **Input exclusion**: you can never be sure a missing message is from a corrupted party or just late; therefore **up to *t* honest inputs may be dropped** to guarantee termination.

### 2.3 Practical Advantages
- **Robust on the Internet**: continues to make progress during DDoS attacks, temporary partitions, or high latency links.  
- **No parameter tuning**: no Δ to mis-estimate; protocols are **parameter-free** with respect to timing.  
- **Composability friendly**: easier to reason about security when arbitrary networks are plugged together.

---

## 3. Head-to-Head Comparison

| Dimension | Synchronous | Asynchronous |
|-----------|-------------|--------------|
| **Max faulty parties** | *t* < *n*/2 (crypto) | *t* < *n*/3 (crypto/stat) or *t* < *n*/4 (perfect) |
| **Input guarantee** | All honest inputs | ≥ *n–2t* inputs (typ.) |
| **Message delay assumption** | Bounded Δ | Arbitrary, adversarial |
| **Termination** | Deterministic | Probabilistic/Deterministic |
| **Communication cost** | Lower (O(*n*)–O(*n*²) per gate) | Higher (O(*n*²)–O(*n*³) per gate) |
| **Real-world fit** | LANs, controlled data centers | Internet, WANs, mobile |
| **Ease of implementation** | Conceptually simpler | Requires extra abstractions (ACS, RBC, ABA) |

---

## 4. Key Technical Challenges in the Asynchronous World

1. **Agreement on a Core Set (ACS)**  
   Parties must decide which *n–t* inputs to include without knowing who is slow vs. malicious. ACS is usually built from multiple parallel **reliable broadcasts** and **asynchronous Byzantine agreement** instances.

2. **Reliable Broadcast (RBC)**  
   Guarantees that if any honest party accepts a message, every honest party eventually accepts the same message. Needed because plain broadcast is impossible under asynchrony.

3. **Asynchronous Byzantine Agreement (ABA)**  
   Lets parties agree on a single bit (or value) despite *t* corruptions and arbitrary message delays. Typically requires expected constant or O(*n*) rounds.

4. **Communication Blow-Ups**  
   Early asynchronous protocols carried O(*n*⁵) field elements per multiplication gate. Recent works (e.g., Beerliová-Trubíniová & Hirt, 2020; Choudhuri et al., 2024) have reduced this to O(*n*³) or even **O(*n*)** for special functionalities, but synchronous protocols still win on raw throughput.

---

## 5. Hybrid Models: “Best of Both Worlds”

Because pure synchrony is fragile and pure asynchrony sacrifices resilience and inputs, researchers have proposed **hybrid** or **fallback** models:

- **Synchronous-first, asynchronous-fallback**  
  Run a fast synchronous protocol; if Δ is violated, automatically switch to an asynchronous sub-protocol that tolerates *t* < *n*/3. Tight feasibility condition: *tₐ* + 2*tₛ* < *n* where *tₐ* and *tₛ* are corruption thresholds in async and sync phases respectively.

- **Eventually synchronous**  
   The network can be asynchronous for an arbitrary period but must become synchronous eventually. Many blockchain consensus algorithms (e.g., PBFT, Tendermint) adopt this model.

- **Random-synchrony / Δ-bounded-with-errors**  
   Message delays follow an unknown distribution with occasional spikes; protocols self-adjust Δ on the fly.

---

## 6. Concrete Performance Snapshot (2024)

| Metric | State-of-the-Art Sync MPC | State-of-the-Art Async MPC |
|--------|---------------------------|----------------------------|
| **Per-multiplication comm.** | 6*n* field elements | 8*n*–12*n* field elements |
| **Rounds per multiplication** | 1–2 | expected 4–6 |
| **Resilience (perfect)** | *t* < *n*/3 | *t* < *n*/4 |
| **Input inclusion** | 100 % | ≥ 75 % (for *n*=16, *t*=4) |
| **WAN runtime (100 Mbit)** | ~40 ms | ~90 ms |

*(Figures are for semi-honest security; malicious adds 2-3× overhead.)*

---

## 7. Take-Away Guidelines for Practitioners

1. **Inside a single data center or LAN** → **Synchronous** is simpler, faster, and more secure.  
2. **Globally distributed clusters, mobile users, or public Internet** → **Asynchronous** (or hybrid) avoids catastrophic timeout failures.  
3. **Regulatory requirement to include every client’s input** → Prefer synchronous or hybrid with sync-first fallback.  
4. **High-value financial settlement** → Combine: run **async agreement** to commit input, then **sync phase** for fast computation, with async fallback if sync fails.  
5. **Parameter anxiety** → If you cannot confidently bound Δ, choose async; mis-estimating Δ breaks **liveness *and* security** in sync protocols.

---

## 8. Looking Forward

- **Communication-optimal async**: Very recent works (Eurocrypt 2024) achieve **O(*n*)** field elements per gate—closing the gap with synchronous protocols.  
- **Post-quantum async**: Lattice-based reliable broadcast and ACS are emerging, aiming to keep async MPC quantum-safe.  
- **Adaptive corruption**: Most async protocols already handle adaptive adversaries; synchronous counterparts are catching up.  
- **Standardization**: The MPC Alliance is drafting a **communication-model agnostic API** so that wallets and custody software can swap sync/async backends transparently.

---

## Bottom Line

The synchronous model gives you **maximum security and efficiency** at the cost of **brittle timing assumptions**. The asynchronous model gives you **robustness and Internet-friendliness** at the cost of **higher communication and weaker resilience**. Hybrid constructions are quickly maturing, letting system designers tune the dial between these extremes. Understanding the trade-offs—and selecting (or switching) models deliberately—is now a core engineering skill for anyone deploying MPC in the real world.