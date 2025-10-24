# Nash Equilibrium – Explained Simply

**Nash Equilibrium** is a situation in a game (like a competition or decision-making between people) where **no one wants to change their choice**, because everyone is already doing the **best they can**, given what the others are doing.

It’s named after John Nash (from the movie *A Beautiful Mind*).

---

## Key Idea (in plain words):
> “Everyone is playing their best move, assuming the others don’t change.”

If **anyone** could do better by switching their choice **alone**, it’s **not** a Nash Equilibrium.

---

## Simple Example 1: **The Prisoner’s Dilemma** (Classic!)

Two criminals, Alice and Bob, are caught. Police offer a deal:

|                | Bob stays quiet | Bob confesses |
|----------------|------------------|----------------|
| **Alice quiet**| Both get 1 year  | Alice: 3 years, Bob: free |
| **Alice confess**| Alice: free, Bob: 3 years | Both get 2 years |

### Payoffs (lower number = better for them):
- 0 = free
- 1 = 1 year
- 2 = 2 years
- 3 = 3 years

### What happens?

- If Alice thinks Bob will **stay quiet**, she should **confess** (0 > 1).
- If Alice thinks Bob will **confess**, she should **confess** (2 > 3).

Same for Bob.

So both **confess** → both get 2 years.

### Is this a Nash Equilibrium?
**Yes!**  
Neither wants to change:
- If Alice switches to quiet → she gets 3 years (worse).
- Same for Bob.

Even though **both would be better** if they both stayed quiet (1 year each), they can’t trust each other.

---

## Example 2: **Rock-Paper-Scissors** (Mixed Strategy)

You and I play Rock-Paper-Scissors.

|             | You: Rock | You: Paper | You: Scissors |
|-------------|-----------|------------|---------------|
| **I: Rock** | 0, 0       | –1, +1     | +1, –1        |
| **I: Paper**| +1, –1    | 0, 0       | –1, +1        |
| **I: Scissors**| –1, +1  | +1, –1     | 0, 0           |

(+1 = win, 0 = tie, –1 = lose)

### Pure strategy? No Nash!
- If I always play Rock → you play Paper → I switch → etc. Keeps changing.

### Nash Equilibrium = **Mix 1/3 each**
- Both play Rock, Paper, Scissors **randomly**, each with **33% chance**.
- Then, no matter what you do, I expect **0** (tie on average).
- You can’t gain by changing.

**Mixed Nash Equilibrium**

---

## Example 3: **Traffic – Left or Right Side?**

Two drivers approaching each other on a narrow road.

|             | Drive on Left | Drive on Right |
|-------------|---------------|----------------|
| **You Left**| Safe, Safe    | Crash, Crash   |
| **You Right**| Crash, Crash | Safe, Safe     |

### Nash Equilibria:
1. **Both drive on Left** → Safe. Neither wants to switch.
2. **Both drive on Right** → Safe. Neither wants to switch.

**Two Nash Equilibria!**

(Real life: countries pick one as a rule — like UK drives left, USA drives right.)

---

## Example 4: **Going to a Movie – Coordination Game**

You and your friend want to watch a movie together, but there are two cinemas: **Cinema A** or **Cinema B**.

You both prefer to be together.

|                | Friend: A | Friend: B |
|----------------|-----------|-----------|
| **You: A**     | 2, 2       | 0, 0       |
| **You: B**     | 0, 0       | 2, 2       |

(2 = happy together, 0 = alone)

### Nash Equilibria:
1. **Both go to A**
2. **Both go to B**

Both are stable — no one wants to switch alone.

But you need to **coordinate** (text, call, agree).

---

## Example 5: **Pricing War – Two Gas Stations**

Two gas stations across the street.

They can set price: **High** or **Low**.

|                 | Station B: High | Station B: Low |
|-----------------|-----------------|----------------|
| **A: High**     | 5, 5             | 1, 8           |
| **A: Low**      | 8, 1             | 2, 2           |

(Profit in thousands)

### What happens?
- If B sets High → A sets **Low** (8 > 5)
- If B sets Low → A sets **Low** (2 > 1)

So both set **Low** → profit 2 each.

### Nash Equilibrium?
**Yes: (Low, Low)**  
Neither can do better by switching to High (would drop to 1).

Like Prisoner’s Dilemma — both would be better at (High, High), but can’t trust.

---

## How to Find Nash Equilibrium (Step-by-Step)

1. **List all strategies** for each player.
2. **Make a payoff table**.
3. **For each player**, underline their **best response** to what the other does.
4. **Nash = where both are best-responding** (both underlined in same cell).

### Example: Gas Station (again)

|                 | B: High | **B: Low** |
|-----------------|---------|------------|
| **A: High**     | 5, 5    | 1, **8**   |
| **A: Low**      | **8**, 1| **2**, **2**|

→ Only **(Low, Low)** has both best responses.

---

## Summary: What Makes a Nash Equilibrium?

| Condition | Yes/No |
|--------|--------|
| Everyone is choosing their **best move** given others | ✅ |
| No one regrets their choice | ✅ |
| No one can gain by **changing alone** | ✅ |

---

## Real-Life Uses
- Economics (pricing, auctions)
- Politics (voting, arms race)
- Biology (animal behavior)
- Traffic rules
- Social media trends

---

## Final Thought

> **Nash Equilibrium doesn’t mean “best for everyone”** — just **“stable”**.

Like both confessing in prison: bad outcome, but no one moves.
