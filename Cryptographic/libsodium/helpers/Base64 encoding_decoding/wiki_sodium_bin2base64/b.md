# 🧪 `sodium_bin2base64` – dissected byte-by-byte

## 1️⃣ Prototype anatomy
```c
char *
sodium_bin2base64(char * const b64,        // caller-supplied buffer
                  const size_t b64_maxlen, // physical size of that buffer
                  const unsigned char * const bin,
                  const size_t bin_len,
                  const int variant);      // flags controlling alphabet & padding
```
*Returns* the same pointer `b64` for convenience (fluent interface).

---

## 2️⃣ Local variables – what lives on the stack
| Declaration | Purpose |
|-------------|---------|
| `size_t acc_len = 0` | **bit-accumulator length** – counts how many raw bits are currently parked inside `acc` |
| `size_t b64_len` | **exact number of Base-64 characters** that *will* be produced (including `'='` padding) |
| `size_t b64_pos` | **write cursor** into the output buffer |
| `size_t bin_pos` | **read cursor** into the input buffer |
| `size_t nibbles` | how many *complete* 3-byte groups exist |
| `size_t remainder` | leftover bytes after those groups (`0,1,2`) |
| `unsigned int acc` | 32-bit **bit bucket** – bytes are shifted in from the left, 6-bit symbols are shifted out from the right |

---

## 3️⃣ Variant sanity gate
```c
sodium_base64_check_variant(variant);
```
🔒 If the caller passes an invalid/unsupported variant flag the library aborts via `sodium_misuse()` – a security-hardened `assert()` that never gets compiled out.

---

## 4️⃣ Pre-calculate output length – no surprises
```c
nibbles = bin_len / 3;          // 3 bytes → 4 chars
remainder = bin_len % 3;        // 0,1,2
b64_len = nibbles * 4;
```
Padding behaviour is **variant-driven**:
1. **Standard/Base64 with padding** → always multiple of 4 chars  
   `b64_len += 4;`
2. **URL-safe *without* padding** → add only what is strictly needed  
   `b64_len += 2 + (remainder>>1);`  
   - 1 remainder byte → 2 chars  
   - 2 remainder bytes → 3 chars  

---

## 5️⃣ Early abort if caller’s buffer too small
```c
if (b64_maxlen <= b64_len) sodium_misuse();
```
Again, **fail fast** rather than overflowing.

---

## 6️⃣ Two-track encoder – classic vs. URL-safe
The routine forks once, then the loops are identical except for the lookup helper:

| Branch | Lookup helper | Alphabet |
|--------|---------------|----------|
| `VARIANT_URLSAFE_MASK` set | `b64_byte_to_urlsafe_char` | `-` and `_` instead of `+` `/` |
| otherwise | `b64_byte_to_char` | RFC-4648 §4 |

---

## 7️⃣ Bit-grinding engine (generic for both branches)
```c
while (bin_pos < bin_len) {
    acc = (acc << 8) + bin[bin_pos++]; // feed another byte
    acc_len += 8;                      // we now own 8 more bits
    while (acc_len >= 6) {             // can emit a symbol?
        acc_len -= 6;
        b64[b64_pos++] = lookup[(acc >> acc_len) & 0x3F];
    }
}
```
After the loop ends we may still have **1–4 bits** left (`acc_len ∈ {2,4}`).  
A final symbol is manufactured by **left-shifting** to the 6-bit boundary:
```c
if (acc_len > 0)
    b64[b64_pos++] = lookup[(acc << (6 - acc_len)) & 0x3F];
```
🔧 *Key insight*: the code never branches on remainder; it simply keeps stuffing bits until the bucket is empty.

---

## 8️⃣ Padding phase
```c
while (b64_pos < b64_len) b64[b64_pos++] = '=';
```
If the variant requested **no padding**, `b64_len` was already trimmed, so this loop is skipped.

---

## 9️⃣ NUL terminator – defensive style
```c
do { b64[b64_pos++] = 0U; } while (b64_pos < b64_maxlen);
```
libsodium **zeroes the tail** of the buffer to avoid information leakage.  
The caller may pass a buffer larger than `b64_len+1`; every unused byte is cleared.

---

## 🔟 Return value
```c
return b64;
```
Useful for chaining:  
`printf("%s\n", sodium_bin2base64(buf, sizeof buf, key, sizeof key, sodium_base64_VARIANT_ORIGINAL));`

---

# 🎮 Hands-on examples

## Input 1 – 3 bytes (exact multiple)
Binary (hex) | `0x 14 FB 9C`  
Length       | 3  
Variant      | `sodium_base64_VARIANT_ORIGINAL` (standard, with padding)

**Encoding steps**  
1. `0x14FB9C` → `0001 0100 1111 1011 1001 1100` (24 bits)  
2. Split into four 6-bit indices:  
   `000101` `001111` `101110` `011100` → 5 15 46 28  
3. Lookup table → `F P u c`  
4. Already multiple of 4 → no `'='` needed  

**Output string**  
```
"FPuc"
```

---

## Input 2 – 1 byte (needs padding)
Binary | `0x 61` (`'a'`)  
Length | 1  

Bit stream: `0110 0001` (8 bits)  
After main loop:  
- acc = `0x61`, acc_len = 8  
- pop 6 bits → index `0x18` (24) → `'Y'`  
- acc becomes `0x1`, acc_len = 2  
Final symbol: `acc << 4` → `0x10` (16) → `'Q'`  

Total chars so far: 2 → pad to 4 with `'='`  

**Output**  
```
"YQ=="
```

---

## Input 3 – 2 bytes (URL-safe, no padding)
Binary | `0x 60 09`  
Variant | `sodium_base64_VARIANT_URLSAFE_NO_PADDING`

Bit stream: `0110 0000 0000 1001`  
6-bit indices:  
`011000` `000000` `001001` → 24 0 9 → `'Y'` `'A'` `'J'`  

No padding requested → **exactly 3 characters**  
**Output**  
```
"YAJ"
```

---

# 🧩 Companion helpers you should know

| Function | Why it matters |
|----------|----------------|
| `sodium_base64_check_variant` | Validates flags at run-time |
| `b64_byte_to_char` | Classic RFC-4648 table |
| `b64_byte_to_urlsafe_char` | Same indices but `'-' '_' '` |
| `sodium_misuse()` | Security panic – logs & aborts |

---

# 🛡️ Security notes & best practices
- **Always** supply `b64_maxlen` ≥ `sodium_base64_ENCODED_LEN(bin_len, variant)` macro.  
- libsodium **zeroises** the scratch accumulator (`acc`) and tail bytes – this mitigates cold-boot attacks.  
- The implementation is **constant-time** with respect to secret data: no secret-dependent branches or indices.  
- Re-use the same buffer for **multiple encodings** only if you wipe it between calls (`sodium_memzero`).

---

# 🧪 Mini test bench (copy-paste ready)
```c
#include <sodium.h>
#include <stdio.h>

int main(void) {
    sodium_init();

    unsigned char secret[5] = {0xDE,0xAD,0xBE,0xEF,0x42};
    char out[sodium_base64_ENCODED_LEN(sizeof secret,
                                       sodium_base64_VARIANT_ORIGINAL)];

    sodium_bin2base64(out, sizeof out, secret, sizeof secret,
                      sodium_base64_VARIANT_ORIGINAL);
    printf("standard:  %s\n", out);     // 3q2+70I=

    sodium_bin2base64(out, sizeof out, secret, sizeof secret,
                      sodium_base64_VARIANT_URLSAFE_NO_PADDING);
    printf("urlsafe:   %s\n", out);     // 3q2-70I   (no '=')
    return 0;
}
```
Compile:  
`gcc demo.c -lsodium -o demo`

---

# 🧠 Related concepts
- **Base-64 index table** – 64-entry map 0→'A', … 63→'/'.  
- **Radix-64 armour** – same idea, but includes a 24-bit CRC (PGP).  
- **Base64url** – RFC-4648 §5, used in JWT, JWS, JWE.  
- **Ascii85** – more compact but uses extra punctuation (`<~ ~>`).  
- **Z85** – ZeroMQ variant, avoids quotes & backslashes for embedding in JSON/C strings.

---

# 🗂️ See also
- `sodium_base642bin()` – reverse operation with **strict whitespace & padding validation**.  
- `sodium_base64_ENCODED_LEN()` – compile-time macro to size buffers.  
- `sodium_memzero()` – secure erase.

---

Enjoy crafting human-readable fingerprints of your binary secrets!