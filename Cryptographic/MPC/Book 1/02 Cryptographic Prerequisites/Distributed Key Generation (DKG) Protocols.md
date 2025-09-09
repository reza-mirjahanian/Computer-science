
## 1. **Pedersen DKG (1991)**
- **Foundation**: Based on Feldman's VSS with commitment to zero
- **Properties**: Provides uniform distribution of generated keys
- **Security**: Secure against passive adversaries
- **Use Cases**: Threshold signatures, distributed cryptosystems
- **Threshold**: Works with $t < n/2$ corrupted parties

## 2. **Feldman's VSS-based DKG (1987)**
- **Foundation**: Verifiable Secret Sharing using polynomial commitments
- **Properties**: Simple and efficient, but doesn't guarantee uniform key distribution
- **Security**: Computationally secure
- **Use Cases**: Basic threshold cryptography applications
- **Note**: Often used as a building block for other protocols

## 3. **Gennaro-Jarecki-Krawczyk-Rabin (GJKR) DKG (1999/2007)**
- **Foundation**: Improved version of Pedersen DKG
- **Properties**: 
  - Guarantees uniform distribution
  - Robust against active adversaries
  - Two versions: GJKR99 and GJKR07 (improved)
- **Security**: Secure against $t < n/2$ malicious parties
- **Use Cases**: Threshold DSA/ECDSA, distributed randomness beacons

## 4. **Canetti-Gennaro-Jarecki-Krawczyk-Rabin DKG (1999)**
- **Foundation**: Adaptive security improvements
- **Properties**: Secure against adaptive adversaries
- **Security**: UC-secure variant
- **Use Cases**: Long-term key generation systems

## 5. **Kate-Goldberg DKG (2010)**
- **Foundation**: Based on polynomial commitments using pairings
- **Properties**: 
  - Constant-size broadcasts
  - Efficient verification
- **Security**: Computationally secure
- **Use Cases**: Large-scale distributed systems

## 6. **Neji-Gurkan-Sekulic (NGS) DKG (2021)**
- **Foundation**: Optimized for blockchain applications
- **Properties**: 
  - Reduced communication complexity
  - Aggregatable signatures
- **Use Cases**: Ethereum 2.0 validators, blockchain consensus

## 7. **FROST DKG (2020)**
- **Full Name**: Flexible Round-Optimized Schnorr Threshold
- **Foundation**: Optimized for Schnorr signatures
- **Properties**: 
  - Two-round protocol
  - Efficient threshold signatures
- **Use Cases**: Bitcoin Taproot, threshold wallets

## 8. **ETHDKG (2017)**
- **Foundation**: Designed specifically for Ethereum
- **Properties**: 
  - Byzantine fault-tolerant
  - Optimized for EVM
- **Security**: Handles up to $t < n/3$ Byzantine nodes
- **Use Cases**: Ethereum randomness beacon, sharding

## 9. **Groth's DKG (2021)**
- **Foundation**: Non-interactive DKG using class groups
- **Properties**: 
  - Publicly verifiable
  - No private channels needed
- **Use Cases**: Decentralized networks without established communication

## 10. **Fouque-Stern DKG (2000)**
- **Foundation**: Based on Paillier cryptosystem
- **Properties**: Suitable for RSA key generation
- **Use Cases**: Distributed RSA key generation

## Common Applications by Protocol:

### **Blockchain & Cryptocurrencies:**
- GJKR variants (threshold ECDSA)
- FROST (Bitcoin/Schnorr)
- NGS DKG (Ethereum 2.0)
- ETHDKG (Ethereum-specific)

### **Secure Multi-party Computation:**
- Pedersen DKG
- GJKR protocols
- Kate-Goldberg (for large parties)

### **Randomness Beacons:**
- GJKR07
- ETHDKG
- NGS DKG

### **Threshold Wallets:**
- FROST
- GJKR (for ECDSA)
- GG18/GG20 protocols (built on GJKR)

## Selection Criteria:
When choosing a DKG protocol, consider:
1. **Security model** (honest majority vs. dishonest majority)
2. **Communication complexity**
3. **Computation requirements**
4. **Network assumptions** (synchronous vs. asynchronous)
5. **Specific cryptographic scheme** (RSA, DSA, ECDSA, Schnorr, etc.)
6. **Number of participants**
7. **Required security properties** (robustness, fairness, etc.)