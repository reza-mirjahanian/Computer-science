### **Key Concepts in Cryptography**  



#### **Core Terminology**  
- **Cryptography** (Greek origin):  
  - *Crypto* = **conceal, hidden, secret**.  
  - *Graphy* = **writing, recording, describing**.  
  - Essentially, the **"art of concealing information."**  

- **Cryptographers**:  
  - Individuals who **create or break** coded messages.  

- **Plain Text**:  
  - The **original message** before encryption.  

- **Cipher Text**:  
  - The **encrypted message** after applying a cipher.  

- **Encryption**:  
  - The process of converting **plain text → cipher text**.  

- **Decryption**:  
  - The process of converting **cipher text → plain text**.  

- **Cipher**:  
  - A **specific algorithm** used for encryption/decryption.  
  - Different ciphers produce **different cipher texts**.  

- **Algorithm**:  
  - A **step-by-step process** with:  
    - **Input → Process → Output**.  
  - In cryptography, algorithms are called **ciphers**.  

#### **Example: Caesar Cipher**  
- **Shift letters** by a fixed number (e.g., **shift by 3**).  
  - *Plain text*: **"secret"** → *Cipher text*: **"vhfuhw"** (shift +3).  
- Used by **Julius Caesar (58 BC)**.  
- **Process**:  
  1. Take the first letter (**"s"**).  
  2. Look up its shifted counterpart (**"v"**).  
  3. Repeat for all letters.  

#### **Cryptanalysis**  
- The **art of breaking codes** without full knowledge.  
- Techniques:  
  - Analyzing **repeated patterns** (e.g., frequency of letters like "e").  
  - Reverse-engineering cipher text.  

#### **Keys in Cryptography**  
- **Key**: Additional information needed to **encrypt/decrypt**.  
  - Example: **Shift by 8** instead of 3.  
    - *Plain text*: **"secret"** → *Cipher text*: **"amkzmb"** (shift +8).  

#### **Types of Cryptographic Methods**  
1. **Substitution**:  
   - Replacing letters with others (e.g., Caesar cipher).  
2. **Transposition**:  
   - **Rearranging letters** (no substitution).  
3. **Steganography**:  
   - **Hiding messages** within other media (e.g., images, sentences).  
     - Example:  
       - *Sentence*: **"Second word so if you take the first letter of every second word..."**  
       - *Hidden message*: Take **first letter of every second word** → **"secret"**.  

#### **Practical Exercise**  
- **Steganography challenge**:  
  - Find the hidden word **"secret"** in:  
    > *"Second word so if you take the first letter of every second word then we can see that it spells out secret."*  
  - **Solution**: First letter of **every second word** = **"secret"**.  

#### **Summary of Terms Covered**  
- Cryptography  
- Cryptographers  
- Plain Text  
- Cipher Text  
- Encryption & Decryption  
- Cipher & Algorithm  
- Cryptanalysis  
- Keys  
- Substitution & Transposition  
- Steganography