### DKG
DKG completely eliminates this trusted **third party**. Instead, every participant plays the dual role of "**dealer**," contributing random input to the secret and ultimately receiving a share of the collaboratively generated private key. The security of the system is then governed by a **threshold parameter**: only a sufficient subset **(e.g., t out of n)** of parties is required to participate to perform cryptographic operations (such as signing or decryption), while no single party—or any coalition smaller than the threshold—can reconstruct the key.

In practice, this means that:

1. No party ever possesses the full private key.
2. The system as a whole is resilient to a defined number of malicious or faulty participants.
3. The system supports robustness (liveness, even in the presence of adversarial parties) and verifiability (participants can guarantee that shares are consistent and correct, preventing cheating attacks).

###  threshold cryptography
