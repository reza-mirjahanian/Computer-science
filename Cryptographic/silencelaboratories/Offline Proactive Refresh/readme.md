## Background: Threshold Signature Schemes (TSS)

* TSS uses multi-party computation (MPC) to split control of a digital secret across *n* parties, requiring at least *t* parties to jointly sign a transaction. This eliminates single-point-of-failure vulnerabilities.

## What Is Proactive Refresh?

* Proactive refresh is a security practice where key shares are periodically renewed to invalidate stale ones, thwarting attackers who might accumulate shares over time.
* A downside of traditional proactive refresh: **all parties must be online and cooperative simultaneously**, which can be impractical and even vulnerable to denial-of-service (DoS) disruptions.

## The Offline Proactive Refresh Innovation

Offline Proactive Refresh is designed to overcome the practical limitations of standard approaches:

### Key Advantages

1. **Catch-Up Flexibility**
   Offline participants—those missing the refresh moment—can update their key share later by obtaining a single refresh packet. This ensures no one is left behind even after multiple missed refresh rounds.

2. **Resistance to DoS Attacks**
   Only *two* parties (a valid signing quorum) are needed to perform the refresh. Others don’t have to be online, reducing the risk that one uncooperative member could block refresh.

3. **Accountable Key Rotations**
   Every refresh generates a record of which quorum signed off on that operation, offering traceability and accountability.

4. **Lightweight Protocol**
   The refresh process involves just **three message exchanges** between two parties, followed by the usual signing flow.

### Illustrative Example

A TSS(2,3) setup is described where:

* A coin exchange holds one share.
* The user has shares on both a laptop and a phone.
* Ideally, both devices must be online simultaneously for a standard refresh—but that's inconvenient.
* With offline refresh, just the phone (always on) and the exchange participate to generate the refresh.
* When the laptop comes online later, it downloads a catch-up packet containing the refresh data and updates its share—no global coordination required.

The laptop can sequentially apply multiple missed refresh packets to always sync with the latest state, assuming none are lost.

## How It Works: The Protocol Mechanics

* The shared secret is represented using polynomial-based secret shares (each party’s share defined via a degree-*t* polynomial).
* In the *t = 2* case, each party holds a linear polynomial defined by parameters *(aᵢ, bᵢ)*.
* Refreshing involves generating a shared nonce Δ that updates the *aᵢ* values while maintaining *bᵢ*, so the public verification key remains unchanged.
* Two parties jointly generate Δ, update their shares, then publicly sign the hash of Δ (but not Δ itself).
* Offline participants later receive Δ and validate it using the signature, then update their share themselves.

The process requires minimal exchange and keeps the protocol efficient and secure.

---

