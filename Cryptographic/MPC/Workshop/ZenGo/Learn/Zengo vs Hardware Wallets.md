# 🛡️ ZenGo vs. Hardware Wallets – *The Deep Dive*
 Zengo uses a 3-factor Recovery Kit to ensure you can recover your wallet.

However, it requires that you always have access to these items: 1) Email address, 2) Recovery File in your cloud drive, and 3) 3D FaceLock, which is optional.

> Zero ZenGo wallets ever hacked, phished, or taken over – **1 M+ users since 2019**.

---

## 1. The Two Problems Every Wallet Must Solve
1. **Secure Storage** – keep coins safe *today*.
2. **Secure Recovery** – keep coins safe *tomorrow* (new phone, lost device, etc.).

---

## 2. The Seed-Phrase Catastrophe
> **~$250 B** in BTC alone lost or stolen in 10 yrs – mostly **seed mismanagement**.

| Weakness | Real-World Fail |
|----------|-----------------|
| 12/24 words = **single treasure map** | Body-cam photo ➜ emptied wallet |
| Copy online ➜ instant drain | Phishing “support” ➜ instant drain |
| No “undo” | Lose paper ➜ **permanent loss** |

---

## 3. ZenGo’s MPC Core – *No Seed, No Single Point*

### 🔐 **2-of-2 Secret-Sharing Model**
| Share | Location | Power |
|-------|----------|-------|
| **Personal** | Your phone (secure hardware) | **Can initiate** tx |
| **Remote** | ZenGo server (encrypted + fragmented) | **Co-signs only** |

> Both shares **compute together**; **private key never exists**.

---

## 4. Recovery Without a Seed – **3-Factor System**
1. **Email access** – ownership proof.
2. **Recovery file** – non-secret entropy helper (store anywhere).
3. **3D FaceLock** – liveness biometrics, **$600 k bounty unbroken**.

> Lose phone ➜ reinstall app → scan face → open email → restore.

---

## 5. Stress-Tested in Public
| Event | Result |
|-------|--------|
| **10 BTC + Pudgy Penguin bounty** (Jan 2024) | **0** successful hacks in 2 weeks |
| Ongoing audits | Passed every year |
| Open-source MPC lib | [github.com/zengo](https://github.com/zengo) |

---

## 6. ZenGo Pro – *Things a Hardware Wallet Can’t Do*

### 🚦 **Theft Protection**
- Set **USD threshold** (e.g. **≥ $420**).
- Any tx above it → **forced 3D FaceLock**.
- **PIN or phone unlock ≠ enough**.

### 🛠️ **Web3 Firewall**
- **Traffic-light UI**: 🟢 safe 🟡 caution 🔴 block.
- **Transaction simulation** before you sign.

### 🏰 **Legacy Transfer** (Inheritance)
| Step | Control |
|------|---------|
| Pick **recipient** (any ZenGo user) | ✅ |
| Set **inactivity timer** (4 m – 2 y) | ✅ |
| Cancel / re-assign **anytime** | ✅ |
| **No KYC, multi-chain, self-custodial** | ✅ |

---

## 7. Support & Education
- **24/7 live chat** – real humans, in-app.
- **Multi-language** – global coverage.
- **Built-in guides** – address format, network choice, etc.

---

## 🔍 Side-by-Side Snapshot

| Feature | Hardware Wallet | ZenGo MPC |
|---------|----------------|-----------|
| Seed phrase | ✅ **Vulnerability** | ❌ **Eliminated** |
| Single point of failure | Seed / device | **None** |
| On-chain footprint | Normal | Normal (single sig) |
| Recovery | Seed only | **3-Factor** |
| Theft Protection | PIN ≠ user | **FaceLock + rules** |
| Legacy / inheritance | Seed hand-off | **Automated, revocable** |
| Live support | ❌ | **24/7 in-app** |

---

> **Secure by default, simple by design – that’s ZenGo’s MPC engine.**