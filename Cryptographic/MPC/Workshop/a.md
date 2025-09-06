# 🤝 **Multi-Party Computation (MPC)**  
*How to compute on secret data without revealing it*

---

## 🎯 **The Problem**  
Four colleagues want to know:  
> **What is the average of our salaries?**  
But *nobody* wants to reveal their own salary.

---

## 🔐 **The MPC Magic in 5 Steps**

### 1️⃣ **Setup**  
- **Players**: P₁, P₂, P₃, P₄  
- **Secret inputs**: salaries `a`, `b`, `c`, `d`

---

### 2️⃣ **Splitting Secrets into Random Shares**  
Each player breaks their salary into 4 random numbers that **sum to the salary**.

| Player | Salary | Random Shares (sum = salary) |
|--------|--------|-------------------------------|
| P₁     | `a`    | `a₁, a₂, a₃, a₄`               |
| P₂     | `b`    | `b₁, b₂, b₃, b₄`               |
| P₃     | `c`    | `c₁, c₂, c₃, c₄`               |
| P₄     | `d`    | `d₁, d₂, d₃, d₄`               |

> 🎲 **Key point**: Each share looks totally random on its own.

---

### 3️⃣ **Private Distribution**  
Every player **quietly hands one share to each colleague** (keeping one for themselves).

```
P₁ sends: a₂→P₂, a₃→P₃, a₄→P₄, keeps a₁
P₂ sends: b₁→P₁, b₃→P₃, b₄→P₄, keeps b₂
P₃ sends: c₁→P₁, c₂→P₂, c₄→P₄, keeps c₃
P₄ sends: d₁→P₁, d₂→P₂, d₃→P₃, keeps d₄
```

---

### 4️⃣ **Local Addition**  
Each player adds up **all shares they received** (their *column*).

> **P₁ computes**: `S₁ = a₁ + b₁ + c₁ + d₁`  
> **P₂ computes**: `S₂ = a₂ + b₂ + c₂ + d₂`  
> **P₃ computes**: `S₃ = a₃ + b₃ + c₃ + d₃`  
> **P₄ computes**: `S₄ = a₄ + b₄ + c₄ + d₄`

---

### 5️⃣ **Revealing Only the Final Sum**  
Everyone **broadcasts** their `Sᵢ`.  
The **total salary sum** is:

```
Total = S₁ + S₂ + S₃ + S₄
       = (a₁+…+d₁) + … + (a₄+…+d₄)
       = (a+b+c+d)
```

Divide by 4 → **average salary**  
🔒 **No individual salary ever exposed.**

---

## 🧠 **Why It Works**  
- **Commutative law**: order of addition doesn’t matter.  
- **Randomness hides secrets**: single shares leak zero info.  
- **Local computation + public sum** = privacy-preserving result.

---

## 🪄 **The MPC Mantra**  
> **“Shuffle random shares, compute locally, reveal only the answer.”**

----------------

