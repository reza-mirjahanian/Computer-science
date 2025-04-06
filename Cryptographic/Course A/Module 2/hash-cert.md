### **Hashing a File on Windows Using `certutil`**  

#### **Overview**  
- **Tool Used**: `certutil` (primarily for certificate management but includes file hashing).  
- **Purpose**: Generate hashes to verify file integrity.  

#### **Steps to Hash a File**  

1. **Open Command Prompt**  
   - Press **Windows key**, type `cmd`, and hit **Enter**.  

2. **Check `certutil` Hashing Options**  
   - Run:  
     ```cmd
     certutil -?
     ```  
   - Look for the **`-hashfile`** option.  

3. **Create a Test File**  
   - Navigate to `C:\` and create a folder (e.g., `temp`).  
   - Inside `temp`, create a **text file** (`plain_text_doc.txt`) with content:  
     ```
     This is plain text.
     ```  

4. **Generate a Hash (MD5 Example)**  
   - Run:  
     ```cmd
     certutil -hashfile "C:\temp\plain_text_doc.txt" MD5
     ```  
   - Output: A fixed-length **MD5 hash** (fingerprint).  

5. **Test the Avalanche Effect**  
   - **Modify the file**: Remove the period (`.`) → Save.  
   - Re-run the hash command → Observe the **completely different hash**.  

6. **Verify Consistency**  
   - **Restore original text** (re-add the period).  
   - Re-run the hash → Output matches the **original hash**.  

7. **Test with Different Algorithms**  
   - Example: **MD2**  
     ```cmd
     certutil -hashfile "C:\temp\plain_text_doc.txt" MD2
     ```  
   - Result: A **different hash** (same input, different algorithm).  

#### **Key Observations**  
- **Fixed-Length Output**: Hash length remains constant regardless of file size.  
- **Avalanche Effect**: Tiny changes (e.g., removing a `.`) drastically alter the hash.  
- **Consistency**: Same input → same hash every time.  
- **Algorithm-Dependent**: Different algorithms (e.g., MD5 vs. MD2) produce unique hashes.  

#### **Commands Summary**  
| Action | Command |  
|--------|---------|  
| Check `certutil` options | `certutil -?` |  
| Generate MD5 hash | `certutil -hashfile "C:\path\to\file" MD5` |  
| Generate SHA-256 hash | `certutil -hashfile "C:\path\to\file" SHA256` |  

#### **Notes**  
- **Avoid MD5 for security** (vulnerable to collisions).  
- Use **SHA-256** or **SHA-3** for stronger integrity checks.