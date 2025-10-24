
## What is a Dominant Strategy?

Forget "equilibrium" for a second. Let's start with just a **Dominant Strategy**.

A dominant strategy is the **best possible move** a player can make, **no matter what** the other players decide to do.

Think of it as your "magic answer." If your friend chooses Rock, Paper, or Scissors, your magic answer *still* wins (or at least gives you the best possible outcome). In most games, a magic answer like this doesn't exist. But when it does, it's called a dominant strategy.

You find it by asking yourself two questions:
1.  If my opponent does **Option A**, what is my best move?
2.  If my opponent does **Option B**, what is my best move?

If your best move is the **same** in both cases, then that is your dominant strategy.

---

## What is a Dominant Strategy Equilibrium?

A **Dominant Strategy Equilibrium (DSE)** is the final outcome of a game when *every single player* has a dominant strategy and decides to play it.

It's an "equilibrium" because it's stable. Once everyone is playing their dominant strategy, **no one** has a good reason to change their mind. If you changed your move while everyone else stayed the same, you'd *only* be hurting yourself.

---

## Example 1: The Prisoner's Dilemma (The Classic)

This is the most famous example.

* **The Setup:** The police arrest two partners in crime, Alice and Bob. They put them in **separate rooms** so they can't communicate.
* **The Deal:** They offer each prisoner the same deal:
    * If you **Confess** and your partner stays **Silent**, you go free (0 years) and your partner gets 10 years.
    * If you stay **Silent** and your partner **Confesses**, you get 10 years and your partner goes free (0 years).
    * If you both **Confess**, you both get 5 years in prison.
    * If you both stay **Silent**, you both get just 1 year on a lesser charge.

Let's find Alice's dominant strategy. She has to decide whether to Confess or stay Silent *without* knowing what Bob will do.

1.  **Alice thinks:** "What if Bob **Confesses**?"
    * If I Confess, I get 5 years.
    * If I stay Silent, I get 10 years.
    * *My best move is to **Confess**.*

2.  **Alice thinks:** "What if Bob stays **Silent**?"
    * If I Confess, I go free (0 years).
    * If I stay Silent, I get 1 year.
    * *My best move is still to **Confess**.*

**Alice's Dominant Strategy:** No matter what Bob does, Alice's best move is to **Confess**.

Bob is in the exact same situation. He does the same math:
1.  **Bob thinks:** "What if Alice **Confesses**?" -> My best move is to **Confess** (5 years is better than 10).
2.  **Bob thinks:** "What if Alice stays **Silent**?" -> My best move is to **Confess** (0 years is better than 1).

**Bob's Dominant Strategy:** No matter what Alice does, Bob's best move is to **Confess**.

**The Equilibrium (DSE):**
The DSE is the outcome where both players play their dominant strategy.
* Alice **Confesses**.
* Bob **Confesses**.
* **Final Outcome:** Both get 5 years in prison.

🚨 **The "Dilemma":** Notice that this outcome (5 years each) is *worse for both of them* than if they had both stayed silent (1 year each). But because neither can trust the other, they are "locked" into confessing, which is the stable, rational, but ultimately worse, equilibrium.

---

## Example 2: The Advertising War

* **The Setup:** Two companies, **Coke** and **Pepsi**, are deciding whether to run a huge, expensive Super Bowl ad campaign.
* **The Payoffs (in millions of dollars of profit):**
    * If **both Advertise**, they spend a ton of money just to cancel each other out. Profit: **$100M each**.
    * If **neither Advertises**, they both save their money and keep their normal market share. Profit: **$150M each**.
    * If **Coke Advertises** and **Pepsi doesn't**, Coke steals lots of customers. Profit: Coke gets **$200M**, Pepsi gets **$75M**.
    * If **Pepsi Advertises** and **Coke doesn't**, Pepsi steals lots of customers. Profit: Pepsi gets **$200M**, Coke gets **$75M**.

Let's find Coke's dominant strategy.

1.  **Coke thinks:** "What if Pepsi **Advertises**?"
    * If I Advertise, I get $100M.
    * If I don't Advertise, I get $75M.
    * *My best move is to **Advertise**.*

2.  **Coke thinks:** "What if Pepsi **Doesn't Advertise**?"
    * If I Advertise, I get $200M.
    * If I don't Advertise, I get $150M.
    * *My best move is still to **Advertise**.*

**Coke's Dominant Strategy:** No matter what Pepsi does, Coke's best move is to **Advertise**.

Pepsi has the exact same choices and payoffs.
* If Coke Advertises, Pepsi should **Advertise** ($100M > $75M).
* If Coke Doesn't Advertise, Pepsi should **Advertise** ($200M > $150M).

**Pepsi's Dominant Strategy:** **Advertise**.

**The Equilibrium (DSE):**
The DSE is **(Advertise, Advertise)**. Both companies are forced to spend millions on ads, even though they would *both* be richer if they could magically agree to *not* advertise.

---

## Example 3: Grading on a Curve

* **The Setup:** You and your classmate, David, are in a class. The professor will give an **A** to the student with the higher score. If you tie, you both get a **B**. The low-scoring student gets a **C**.
* **The Choices:** You can either **Study Hard** or **Goof Off**. Studying is hard work.
* **The Payoffs (your personal happiness):**
    * Getting an A = +10 points
    * Getting a B = +5 points
    * Getting a C = 0 points
    * Studying Hard = -2 points (it's a drag)
    * Goofing Off = 0 points

Let's find your dominant strategy.

1.  **You think:** "What if David **Studies Hard**?"
    * If I Study Hard: We tie. I get a B (+5) but I studied (-2). Total: **+3 points**.
    * If I Goof Off: I lose. I get a C (0) and goofed off (0). Total: **0 points**.
    * *My best move is to **Study Hard**.*

2.  **You think:** "What if David **Goofs Off**?"
    * If I Study Hard: I win. I get an A (+10) but I studied (-2). Total: **+8 points**.
    * If I Goof Off: We tie. I get a B (+5) and goofed off (0). Total: **+5 points**.
    * *My best move is still to **Study Hard**.*

**Your Dominant Strategy:** No matter what David does, your best move is to **Study Hard**.

David has the same payoffs, so his logic is identical.
* If you Study, he should **Study** (+3 > 0).
* If you Goof Off, he should **Study** (+8 > +5).

**David's Dominant Strategy:** **Study Hard**.

**The Equilibrium (DSE):**
The DSE is **(Study Hard, Study Hard)**. You both work hard and end up with B's.

---

## Key Takeaways

* **Dominant Strategy:** Your single best move, regardless of what anyone else does.
* **Dominant Strategy Equilibrium (DSE):** The outcome that happens when *all* players use their dominant strategy.
* **Stability:** A DSE is very stable. No player can *unilaterally* (on their own) improve their situation by changing their mind.
* **Not Always the Best:** As the Prisoner's Dilemma and Advertising examples show, the DSE is often **not** the best *group* outcome. It's just the most stable, rational, individualistic one.
* **Not All Games Have One:** Many games (like Rock-Paper-Scissors) do not have a dominant strategy for any player.