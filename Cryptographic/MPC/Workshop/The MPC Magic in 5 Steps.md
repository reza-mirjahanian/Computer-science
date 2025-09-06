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

Multi-Party Computation (MPC) lets a group compute something together using their private data without ever revealing that data to each other.

> 💡 **The Problem:** Let's say you and your colleagues want to know your average salary. It's useful info, but nobody wants to share their personal income. MPC solves this by letting you find the average without anyone revealing their specific salary.

---

## 🤝 The Scenario

It's a simple setup:
* **Who**: Four people (`P1`, `P2`, `P3`, `P4`).
* **What**: They each have a secret salary (`a`, `b`, `c`, `d`).
* **Goal**: Find the average salary `(a + b + c + d) / 4` without exposing their individual salaries.

---

## 🔢 The MPC Process Step-by-Step

Here's how they pull it off.

### 1. Split the Secret 쪼개기
First, each person splits their salary into four random numbers, called **shares**. The only rule is that the shares must add up to the original salary.
* `P1` splits salary `a` into `a1, a2, a3, a4` where `a1 + a2 + a3 + a4 = a`.
* `P2` splits salary `b` into `b1, b2, b3, b4` where `b1 + b2 + b3 + b4 = b`.
* And so on for `P3` and `P4`.

### 2. Distribute the Shares 📤
Next, everyone keeps one of their own shares and privately gives the other three away, one to each person. After the swap, the shares are distributed like this:

|              | Receives from P1 | Receives from P2 | Receives from P3 | Receives from P4 |
| :----------- | :--------------: | :--------------: | :--------------: | :--------------: |
| **Held by P1** |      `a1`      |      `b1`      |      `c1`      |      `d1`      |
| **Held by P2** |      `a2`      |      `b2`      |      `c2`      |      `d2`      |
| **Held by P3** |      `a3`      |      `b3`      |      `c3`      |      `d3`      |
| **Held by P4** |      `a4`      |      `b4`      |      `c4`      |      `d4`      |

**🔒 Crucially, no information is leaked.** The share `a4` that `P1` gives to `P4` is just a random number. It reveals nothing about `P1`'s actual salary `a`.

### 3. Compute Locally ➕
Now, each person adds up the shares they're holding.
* `P1` calculates: `S1 = a1 + b1 + c1 + d1`
* `P2` calculates: `S2 = a2 + b2 + c2 + d2`
* And so on.

### 4. Combine the Results 📢
Finally, everyone announces their sum (`S1`, `S2`, `S3`, `S4`). When you add these public sums together, you get the grand total of everyone's salaries.

`Total Sum = S1 + S2 + S3 + S4 = a + b + c + d`

From there, they just divide by four to get their average salary.

---

## 🤔 Why It Works

The process is both secure and accurate for two simple reasons.

* **It's Accurate** ✔️
    The math works because you're just adding up all 16 shares in a different order. Adding up the rows in the table gives you the secret salaries (`a`, `b`, `c`, `d`). Adding up the columns gives you the public sums (`S1`, `S2`, `S3`, `S4`). Since the order of addition doesn't matter, both methods lead to the same grand total.

* **It's Private** 🤫
    No one ever learns another person's secret. The only pieces of information you ever see are your own salary, meaningless random shares from others, and the final public sums. None of these pieces are enough to reconstruct anyone's private data.