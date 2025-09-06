# ZenGo's Security Model: A Guide to Secure Self-Custody

ZenGo is a self-custodial crypto wallet designed to eliminate the vulnerabilities found in traditional wallets. Its security is built on three core pillars that work together to protect your assets.

***

## The Three Pillars of Security

ZenGo's approach focuses on a comprehensive security model covering every aspect of managing your crypto.
* **🔐 Secure Storage**: Protecting your assets for the long term without a single point of failure.
* **🔑 Secure Recovery**: Ensuring you can easily recover your wallet if your phone is lost or stolen, while making it nearly impossible for others.
* **🛡️ Transactional Security**: Protecting you during day-to-day activities like buying, selling, and interacting with Web3 applications.

***

## 🔐 Pillar 1: Secure Storage with MPC

Traditional wallets rely on a **seed phrase** (12-24 words), which represents a single point of failure. Mismanagement of these phrases has led to the loss of over **$100 billion** in Bitcoin alone.

> Whether it's a software or hardware wallet, if you can't secure the seed phrase, the wallet isn't secure. ZenGo eliminates this vulnerability entirely.

### How Multi-Party Computation (MPC) Works

Instead of a seed phrase, ZenGo uses **Multi-Party Computation (MPC)**, a technology trusted by major institutions like Coinbase and PayPal.

1.  **No Single Key**: ZenGo never creates a complete `private key`.
2.  **Two Secret Shares**: When you create a wallet, two independent mathematical "secret shares" are generated simultaneously:
    * **Share 1 (Personal Share)**: Stored on your mobile device. This is the controlling share that initiates transactions.
    * **Share 2 (Server Share)**: Stored on the ZenGo server. This share co-signs transactions initiated by your personal share.
3.  **Combined Power**: For any transaction to be approved, both shares must work together cryptographically. This process removes the single point of failure, as a hacker would need to compromise two completely separate systems (your phone and ZenGo's server) at the same time.


***

## 🔑 Pillar 2: Secure Recovery with 3FA

Since there is no seed phrase, ZenGo uses a robust **Three-Factor Authentication (3FA)** process for wallet recovery. This method is so secure that **zero** ZenGo wallets have been hacked or taken over since its launch in 2019.

The three factors required for recovery are:

1.  📧 **Something You Have: Your Email**
    * You must authenticate yourself using the email address linked to your account. This doesn't need to be a personal email; it can be any address you control.
2.  💾 **Something You Store: The Recovery File**
    * An encrypted recovery file is created and stored in your personal cloud service (iCloud, Google Drive, or Dropbox).
    * *Important*: This file is **not** a private key or your secret share. It is useless to a hacker on its own.
3.  👤 **Something You Are: 3D FaceLock**
    * An advanced, 3D biometric scan of your face is created during setup. This is a "liveness" scan that maps your facial structure and how it moves.
    * This is **not KYC** (Know Your Customer). The data is encrypted, scrambled, and stored securely; ZenGo cannot see it or associate it with your identity.

| Factor | Description | Role in Recovery |
| :--- | :--- | :--- |
| **Email** | Your registered email account. | Verifies your identity. |
| **Recovery File** | Encrypted file in your personal cloud. | A necessary component for restoring the wallet structure. |
| **3D FaceLock** | A biometric map of your face. | The final, powerful key to prove you are the owner. |

> You are in control. As a self-custodial wallet, you are responsible for maintaining access to these three factors. Losing any one of them will result in the loss of access to your assets. ZenGo provides options to add backup emails and a trusted friend or family member for your 3D FaceLock.

***

## 🛡️ Pillar 3: Transactional Security with Web3 Firewall

Web3 has become a new hunting ground for hackers, with nearly **$4 billion** stolen from Web3 attacks in 2022 alone. Users often approve malicious transactions without understanding what they are signing.

### ClearSign: Your Built-in Firewall

ZenGo's Web3 firewall, called **ClearSign**, analyzes every transaction *before* you approve it, translating complex code into simple, human-readable alerts. It works like a traffic light system.

* 🟢 **Green**: Go ahead. The transaction is with a verified, whitelisted application (like OpenSea or Uniswap) and is safe.
* 🟡 **Yellow**: Proceed with caution. The transaction has some unusual elements that require extra scrutiny.
* 🔴 **Red**: Danger. This is a high-risk transaction, such as giving a private address broad access to all of your NFTs. ZenGo will issue strong warnings to prevent you from draining your wallet.


By providing this clarity, ClearSign ensures that what you approve is what you actually *intend* to approve, protecting you from phishing scams and wallet-draining attacks in real-time.