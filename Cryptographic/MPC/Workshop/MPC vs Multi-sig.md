# 🔐 **MPC vs Multi-sig**  
*Why Pulse Wallet picked MPC*

---

## 1️⃣ **Multi-sig at a Glance**  
> A **virtual vault** needing **multiple keys** to open.

### ✅ **Pros**
- **Distributed control** – no single spender  
- **Simple idea** – *“k-of-n”* signatures required  
- **Battle-tested** – widely supported (BTC, ETH, Gnosis Safe, etc.)

### ❌ **Cons**
| Pain Point | Real-World Effect |
|------------|-------------------|
| 🔧 **Complex setup** | Dev-ops heavy; easy to mis-configure |
| 💸 **Higher gas** | Each sig = more bytes → higher tx fees |
| ⏱️ **Slow human rounds** | Waiting for cosigners = stalled payments |
| 🗝️ **Lost-key deadlock** | One missing signer → **frozen funds** |

---

## 2️⃣ **MPC in One Line**  
> **“Split the secret, never the key.”**  
Private key **never exists in one place**; math replaces hardware.

---

## 3️⃣ **Head-to-Head**

| Feature | Multi-sig | MPC |
|---------|-----------|-----|
| **On-chain footprint** | Multiple public keys + signatures | **Single signature** (looks like a normal tx) |
| **Key management** | *n* distinct keys | *n* **shards of one key** |
| **Chain support** | Needs native multi-sig op-codes | **Ledger-agnostic** works on **any chain** |
| **Privacy** | Signers are **public** | Signers stay **hidden** |
| **Fee cost** | **↑** grows with signer count | **↓** constant |
| **Automation** | Manual cosigning steps | **Fully programmable** |
| **Recovery** | Scripted backup scripts | **Refresh shards** without moving assets |

---

## 4️⃣ **Pulse Wallet’s Choice**  
> 🚀 **MPC ticks every box**:  
- **Cold-level security** without hardware  
- **One-tap UX** – no cosigner delays  
- **Future-proof** – add new chains instantly