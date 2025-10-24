

### 1. The 30-Second Story
Imagine you and a friend are picking **movies** on Netflix at the same time.  
If you both pick **Action**, you’re both “meh” (2 happy points each).  
If you pick **Action** and she picks **Rom-com**, you hate it (0), she loves it (3).  
If you both pick **Rom-com**, you’re both “pretty good” (3 each).  

**Nash Equilibrium** is the pair of choices where **NEITHER of you regrets your pick AFTER seeing what the other chose**.  
You can’t improve your own happiness by *unilaterally* switching.

---

### 2. Mini-Example (Numbers Make It Real)
Payoff matrix (your points first, hers second):

|               | She: Action | She: Rom-com |
|---------------|-------------|--------------|
| **You: Action**   | (2, 2)      | (0, 3)       |
| **You: Rom-com**  | (3, 0)      | (3, 3)       |

- **Cell (Action, Action)**: You’d rather switch to Rom-com (3 > 2) → **Not NE**.  
- **Cell (Rom-com, Rom-com)**: If you switch alone you drop to 0; if she switches alone she drops to 0 → **No incentive to deviate** → **This is a Nash Equilibrium**.

---

### 3. The Formal Definition (Translate to Human)
A **strategy profile** (list of what every player does) is a Nash Equilibrium if  
**each player’s strategy is a best response to the others’ strategies**.  
Key word: **best response** – you can’t do better *given what everyone else is doing*.

---

### 4. Quick Checklist to Spot NE in a Matrix
1. Pick one cell.  
2. Ask: “If I *alone* change my move, do I get a higher payoff?”  
   - If **yes**, discard the cell.  
3. Repeat for the other player.  
4. If **neither** can gain by a lone switch → **Bingo, Nash Equilibrium**.

---

### 5. Classic 2×2 Game: Prisoner’s Dilemma
|               | Partner: Quiet | Partner: Confess |
|---------------|----------------|------------------|
| **You: Quiet**    | (-1, -1)       | (-10, 0)         |
| **You: Confess**  | (0, -10)       | (-5, -5)         |

- **Both confess** is the **only** Nash Equilibrium.  
  - If you alone stay quiet while partner confesses, you get -10 → worse.  
  - Same logic for partner.  
- **Moral**: Equilibrium isn’t necessarily the **best joint outcome**; it’s the **stable** one.

---

### 6. Three Common Questions
**Q1**: Can there be **multiple** Nash Equilibria?  
**A**: Yep. “Battle of the Sexes” has two: (Opera, Opera) and (Football, Football).

**Q2**: Can there be **zero**?  
**A**: In pure strategies, sometimes. But if we let players **randomize** (mixed strategies), at least one always exists – that’s Nash’s Nobel-winning theorem.

**Q3**: Is the **socially best** outcome always an NE?  
**A**: Nope. Prisoner’s Dilemma shows everyone can be worse off at equilibrium.

---

### 7. One-Sentence Takeaway
Nash Equilibrium is the **“no regrets”** point: after the dust settles, **nobody wishes they had done something else unilaterally**.

---
