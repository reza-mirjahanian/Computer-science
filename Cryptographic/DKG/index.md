### DKG
DKG completely eliminates this trusted **third party**. Instead, every participant plays the dual role of "**dealer**," contributing random input to the secret and ultimately receiving a share of the collaboratively generated private key. The security of the system is then governed by a **threshold parameter**: only a sufficient subset **(e.g., t out of n)** of parties is required to participate to perform cryptographic operations (such as signing or decryption), while no single party—or any coalition smaller than the threshold—can reconstruct the key.

In practice, this means that:

1. No party ever possesses the full private key.
2. The system as a whole is resilient to a defined number of malicious or faulty participants.
3. The system supports robustness (liveness, even in the presence of adversarial parties) and verifiability (participants can guarantee that shares are consistent and correct, preventing cheating attacks).

###  threshold cryptography
 In a threshold system, a secret (such as a private cryptographic key) is divided into shares distributed among participants. Shamir’s Secret Sharing is the classic scheme: the secret is embedded in the constant term of a randomly chosen polynomial of degree t-1, and each participant receives an evaluation of the polynomial at a distinct point.

 ### Verifiable Secret Sharing (VSS)
 a pure secret sharing scheme alone is insufficient in adversarial settings: there must be cryptographic guarantees that malicious parties cannot inject invalid or inconsistent shares undetectably. Here, Verifiable Secret Sharing (VSS) comes into play—most notably realized through the **Feldman and Pedersen** protocols and their verifiable polynomial commitments. VSS ensures that, at each step of the process, every party’s input and share can be audited against publicly available cryptographic commitments.