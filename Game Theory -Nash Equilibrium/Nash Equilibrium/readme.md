# Nash Equilibrium: A Simple Guide

## What is Nash Equilibrium?

**Nash Equilibrium** is a situation in a game where no player wants to change their strategy, assuming everyone else keeps their strategy the same. In other words, everyone is doing the best they can given what others are doing.

Think of it like this: *"I'm happy with my choice as long as you're happy with yours, and you're happy with yours as long as I'm happy with mine."*

---

## Key Idea

At Nash Equilibrium:
- Each player's strategy is the **best response** to other players' strategies
- No one can improve their outcome by changing their strategy **alone**

---

## Example 1: The Prisoner's Dilemma 🚔

Two criminals are arrested. Police offer each a deal:

| | Prisoner B Stays Silent | Prisoner B Betrays |
|---|---|---|
| **Prisoner A Stays Silent** | Both get 1 year | A gets 10 years, B goes free |
| **Prisoner A Betrays** | A goes free, B gets 10 years | Both get 5 years |

**Analysis:**
- If B stays silent → A's best move is to **betray** (0 years vs 1 year)
- If B betrays → A's best move is to **betray** (5 years vs 10 years)
- Same logic applies to B

**Nash Equilibrium:** Both betray and get 5 years each!

Even though both staying silent (1 year each) is better, **both betraying** is the Nash Equilibrium because neither wants to risk being the "sucker" who stays silent.

---

## Example 2: Rock-Paper-Scissors ✊✋✌️

This game has **NO pure strategy Nash Equilibrium**. Why?

- If you always play Rock, I should play Paper
- But if I always play Paper, you should play Scissors
- But if you play Scissors, I should play Rock...

The Nash Equilibrium here is a **mixed strategy**: play each option randomly with probability $\frac{1}{3}$.

---

## Example 3: Traffic Intersection 🚗

Two cars approach an intersection from different directions:

| | Car B Stops | Car B Goes |
|---|---|---|
| **Car A Stops** | Both waste time (-1, -1) | B passes safely (0, 1) |
| **Car A Goes** | A passes safely (1, 0) | **CRASH! (-10, -10)** |

**Nash Equilibria:** 
1. A goes, B stops
2. A stops, B goes

Both are Nash Equilibria! No one wants to change if the other stays put. This is why we need traffic lights — to coordinate which equilibrium we're in.

---

## Example 4: Price Competition 💰

Two gas stations across the street:

| | Station B: High Price | Station B: Low Price |
|---|---|---|
| **Station A: High Price** | Both profit well ($50, $50) | A loses customers ($10, $80) |
| **Station A: Low Price** | A gets customers ($80, $10) | Both profit less ($30, $30) |

**Nash Equilibrium:** Both charge **low prices** and earn $30 each.

Why? If B charges high, A should charge low (to get $80). If B charges low, A should also charge low (to avoid getting only $10).

This is why gas stations often have similar prices!

---

## Example 5: Dating Coordination 💑

Alex and Bailey want to meet but lost their phones:

| | Bailey → Movie | Bailey → Concert |
|---|---|---|
| **Alex → Movie** | Happy together! (2, 2) | Alone and sad (0, 0) |
| **Alex → Concert** | Alone and sad (0, 0) | Happy together! (2, 2) |

**Nash Equilibria:** 
1. Both go to movie
2. Both go to concert

**Two equilibria!** The challenge is coordinating which one. This is called a "coordination game."

---

## How to Find Nash Equilibrium

**Step-by-step method:**

1. **For each player**, look at each possible choice the other player could make
2. Find the **best response** (circle it or mark it)
3. Where **all players' best responses meet** = Nash Equilibrium

### Visual Example:

        Player B
         L    R
    U  [3,2] [1,3]
A      
    D  [2,4] [4,1]


**Player A's best responses:**
- If B plays L: A should play U (3 > 2) ✓
- If B plays R: A should play D (4 > 1) ✓

**Player B's best responses:**
- If A plays U: B should play R (3 > 2) ✓
- If A plays D: B should play L (4 > 1) ✓

**Nash Equilibrium:** D, L where both get (2, 4)

---

## Common Misconceptions ❌

1. **Nash Equilibrium ≠ Best outcome for everyone**
   - Example: Prisoner's Dilemma — both betraying is worse than both cooperating!

2. **Games can have multiple Nash Equilibria**
   - Example: Traffic intersection, dating coordination

3. **Some games have no pure strategy Nash Equilibrium**
   - Example: Rock-Paper-Scissors needs mixed strategies

---

## Why is it Important? 🌟

Nash Equilibrium helps us understand:
- **Economics:** Why companies set certain prices
- **Politics:** Why countries make certain alliances
- **Biology:** Why animals behave certain ways (evolutionary stable strategies)
- **Technology:** How to design auctions, networks, and algorithms

---

## Summary

**Nash Equilibrium** = A stable situation where no one wants to change their strategy unilaterally.

**Remember:** 
- It's about **stability**, not necessarily the best outcome
- Everyone is doing their personal best given others' choices
- It's named after mathematician John Nash (the movie "A Beautiful Mind" is about him!)

Want to practice finding Nash Equilibria in more games? Just ask! 🎮