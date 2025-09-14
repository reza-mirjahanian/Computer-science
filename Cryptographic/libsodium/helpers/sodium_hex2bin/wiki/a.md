

This function converts a string of hexadecimal characters (like `"deadbeef"`) into its binary (raw byte) representation.

---

### Function signature

```c
int sodium_hex2bin(
    unsigned char *const bin,   // output buffer for bytes
    const size_t bin_maxlen,    // max size of bin
    const char *const hex,      // input hex string
    const size_t hex_len,       // length of hex string
    const char *const ignore,   // optional chars to skip (e.g. ":\n ")
    size_t *const bin_len,      // actual number of bytes written
    const char **const hex_end  // pointer to where parsing stopped
)
```

---

### Main variables

* `bin_pos`: where we are writing in the output buffer.
* `hex_pos`: where we are reading in the hex string.
* `state`: tracks whether we’re processing the **high nibble** (first hex digit of a byte) or the **low nibble** (second hex digit).
* `c_acc`: accumulator for the high nibble.
* `ret`: return status (`0` = success, `-1` = error).

---

### Core idea

Hexadecimal encodes bytes using two characters per byte:

* `"4F"` → `0x4F`
* `"A1"` → `0xA1`

So the function loops through the hex string, **two chars at a time**:

1. Read hex digit → convert `'0'..'9'`, `'a'..'f'`, `'A'..'F'` into a value 0–15.
2. If we’re at the first digit of the byte → store it in `c_acc * 16`.
3. If we’re at the second digit → combine with previous (`c_acc | c_val`) and write into `bin[bin_pos++]`.
4. Repeat.

---

### Hex digit parsing trick

Instead of using `if (c >= '0' && c <= '9') ...`, the code uses bit tricks:

```c
c_num    = c ^ 48U;         // distance from '0'
c_num0   = (c_num - 10U) >> 8;  // 0xFF.. if '0'..'9', else 0
c_alpha  = (c & ~32U) - 55U;    // distance from 'A'
c_alpha0 = ((c_alpha - 10U) ^ (c_alpha - 16U)) >> 8; // valid A–F?
```

This avoids branching → constant-time-ish, useful for cryptography.

Then the digit’s value is selected:

```c
c_val = (c_num0 & c_num) | (c_alpha0 & c_alpha);
```

---

### Ignore characters

If `ignore` is set (e.g. `":\n "`), the function skips them—but **only between bytes** (when `state == 0U`).

---

### Errors handled

* **Too many bytes**: if output would exceed `bin_maxlen`, set `errno = ERANGE`.
* **Odd number of hex digits**: if parsing ends in the middle of a byte (`state != 0U`), set `errno = EINVAL`.
* **Invalid character**: breaks out, returns error unless ignored.
* **Trailing garbage**: if `hex_end == NULL` but not at end of string, error.

---

### Outputs

* `bin` gets the decoded bytes.
* `*bin_len` (if not `NULL`) tells how many bytes were written.
* `*hex_end` (if not `NULL`) points to where parsing stopped in the hex string.
* Return value:

  * `0` = success
  * `-1` = error (and `errno` set)

---

✅ Example:

```c
const char *hex = "deadbeef";
unsigned char buf[4];
size_t out_len;
int ret = sodium_hex2bin(buf, sizeof buf, hex, strlen(hex), NULL, &out_len, NULL);

// buf = { 0xDE, 0xAD, 0xBE, 0xEF }
// out_len = 4
// ret = 0
```

---

