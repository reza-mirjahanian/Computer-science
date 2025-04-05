### Practical Byzantine Fault Tolerance

Practical Byzantine Fault Tolerance (pBFT) is a particular type of BFT. It was designed as an optimization to the original algorithm that removes some of the main barriers to using BFT in large networks like blockchains.

#### The Limitations of BFT

BFT provides a usable solution to the Byzantine Generals' Problem. As long as a certain percentage of the nodes in the network are honest, the blockchain will come to a consensus on the current state of the distributed ledger.

However, BFT's approach to doing so is inefficient and unscalable. One of the main limitations of BFT is that it relies on direct communications between each pair of nodes in the blockchain network.

This means that, for a network with n nodes, there will be n(n-1) messages to achieve consensus. While this might be workable for small networks, it means that the total number of messages scales with the square of the number of nodes. While a network of 4 nodes needs to send a total of 12 messages to achieve consensus, a network twice its size (8 nodes) would send 56 messages, over four times as many.

At the time of publication, the Bitcoin network had over 17,000 active nodes, meaning that achieving consensus would require over 289 million messages. This would need to be accomplished for each block in the blockchain, making it an unworkable and unscalable solution.

Another issue with direct node-to-node communications is the potential for nodes to be spun up and go down unexpectedly. This happens regularly on the Bitcoin network and other blockchains, making it nearly impossible for nodes in the network to maintain an up-to-date list of the nodes that they would need to communicate with to achieve consensus.

### pBFT Improves on BFT

pBFT is an optimization of the BFT algorithm designed to make BFT practical for large networks. One of the main ways that it accomplishes this is by eliminating the communications between every node in the blockchain network.

pBFT increases efficiency by defining an ordering of the nodes in the network with a primary node and backup nodes. The consensus process is broken into a few phases in which the leader proposes a block, and each node in the network validates it and publishes a message stating that they have validated and approved it. Once a certain number of nodes have accomplished this, the block is considered finalized.

Pros and Cons of pBFT
---------------------

Variants of pBFT are used by some blockchain protocols as a consensus algorithm.

Some of the ***advantages*** of pBFT for blockchain consensus include:

-   **Fault Tolerance:** pBFT is specifically designed to be a fault-tolerant algorithm, enabling it to deal with node failures. This is important in applications like blockchain consensus where nodes may go down without warning.

-   **Transaction Finality:** Many blockchain consensus algorithms, such as Proof of Work or Proof of Stake, only have probabilistic finality, meaning that an accepted block may be removed from the distributed ledger after a reorganization. pBFT offers finality, where a transaction approved by a certain number of nodes cannot be reverted.

-   **Byzantine Fault Tolerance:** pBFT is a Byzantine Fault Tolerant algorithm. This means that it can handle the presence of a certain number of malicious nodes in the network.

In addition to these benefits, pBFT also has its ***downsides***. Some of these include:

-   **Centralization:** pBFT uses a leader node to define the content of the next block on the blockchain and drive the process of getting it approved. This creates a level of centralization that may run counter to the core principles of blockchain.

-   **Scalability:** pBFT requires messages from a certain percentage of the nodes in the network to finalize a block's contents. This means that the number of messages scales with the size of the network, which limits its scalability.

-   **Network Overhead:** Network bandwidth usage is a concern for pBFT for the same reason as scalability. The more nodes that need to communicate with one another, the more bandwidth it consumes.

-   [**Sybil Attacks**](https://www.halborn.com/blog/post/what-is-a-sybil-attack)**:** pBFT makes decisions based on receiving messages from a certain number of nodes in the network. If an attacker controls many different nodes, they have an outsized vote and may be able to approve malicious blocks.

The scalability concerns of pBFT have led it to be used in conjunction with other blockchain consensus algorithms. For example, a consensus algorithm may combine Delegate Proof of Stake (DPoS) with pBFT so that the pool of nodes that participate in pBFT is limited to the number of delegates elected via DPoS.

Achieving Consensus with pBFT
-----------------------------

pBFT is an effective solution to the BGP, enabling the nodes in the network to achieve consensus even in the presence of malicious nodes. However, its scalability and network bandwidth concerns mean that it is rarely used on its own by blockchains.

Instead, the blockchains that use a variant of BFT like pBFT commonly combine it with another algorithm to cut down on the number of voting nodes. This could be a decentralized consensus algorithm like DPoS or use a centralized authority to choose the delegates in a permissioned blockchain.