### Concise Answer  
DKG in blockchains lets **n** nodes jointly create a **single public key** whose matching **private key never exists in one place**.  
Each node holds a **secret share**; any **t-of-n** shares can sign/decrypt, but **no t-1** can.  
Protocols run **asynchronous** **Pedersen-VSS** plus **distributed discrete-log** proofs, terminate in **11 rounds**, tolerate **f < n/3** Byzantine, and cost **O(κ n²)** off-chain messages .  

---

### Detailed Explanation  
In the story, 256 **guild masters** sit around a huge round table in the **Ethereum tavern**.  
They must mint a **magic treasury chest** that can be locked by **one public spell**, yet opened only when **at least 85 masters** simultaneously turn their **individual enchanted keys**.  
No single master may ever hold the **entire master key**, or the guild’s gold could be stolen by one traitor.  

The **architect** hands each master a **blank metal shard** and a **tiny scrying orb**.  
Round by round, they:  
1. Whisper **random runes** into their shard and broadcast a **homomorphic commitment**—a **crystal sphere** that hides the runes but lets others verify arithmetic.  
2. Encrypt their shard under every other master’s **public sigil** and slide the **sealed scrolls** down the table.  
3. Check that the **crystal spheres** match the scrolls; if a scroll is gibberish, they **publicly accuse** the sender and **disqualify** him.  
4. Sum the **valid shards** locally; the **public spell** materializes as the **product of all commitments**, while each master keeps a **private fragment** of the never-assembled master key.  

After **eleven narrative beats**, the chest appears, sealed by the **joint public spell**.  
Any future **t=85** masters can jointly **sign a transaction** or **decrypt a message**, yet **84 colluding masters** learn nothing—**perfect forward secrecy** is preserved.  

**Distributed Key Generation (DKG):**  
A cryptographic protocol that allows a set of **n** mutually distrusting parties to collaboratively compute a **joint public key** and an **n-out-of-n** (or **t-out-of-n**) **secret sharing** of the corresponding **private key**, such that the **private key never exists in entirety** at any single party, and any **qualified subset** of **t** parties can later perform cryptographic operations (signing/decryption) while **t-1** parties cannot, achieved through **verifiable secret sharing (VSS)**, **discrete-logarithm proofs**, and **asynchronous Byzantine agreement**, tolerating up to **f < n/3** malicious parties .