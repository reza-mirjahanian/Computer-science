

## 🔎 Normal `memcmp`

A regular `memcmp` usually works like this:

```c
for (i = 0; i < len; i++) {
    if (b1[i] != b2[i]) {
        return (b1[i] - b2[i]);  // exits immediately
    }
}
return 0;
```

### Timeline:

* Compare **byte 1** → different? Stop immediately.
* Compare **byte 1–2–3–4…** until mismatch → stop.
* The **time taken depends on where the first mismatch happens**.

👉 An attacker can measure how long it took and guess how many bytes were correct before the mismatch.
That leaks information (used in cryptographic attacks, e.g., on passwords or MACs).

---

## 🔒 `sodium_memcmp`

In contrast, `sodium_memcmp` works like this:

```c
d = 0
for (i = 0; i < len; i++) {
    d |= b1[i] ^ b2[i];   // still goes through all bytes
}
return (d == 0) ? 0 : -1;
```

### Timeline:

* Compare **byte 1** → still continue.
* Compare **byte 2** → still continue.
* …
* Compare **last byte** → only then compute result.

**Always takes the same amount of time** (`len` steps), no matter where the mismatch is.

---

## 📊 Side-by-Side Timeline

| Case              | `memcmp` (normal)         | `sodium_memcmp` (constant-time) |
| ----------------- | ------------------------- | ------------------------------- |
| 1st byte differs  | Stops after 1 step ⏱      | Runs full `len` steps ⏱⏱⏱       |
| Middle differs    | Stops halfway ⏱⏱          | Runs full `len` steps ⏱⏱⏱       |
| Last byte differs | Runs almost all steps ⏱⏱⏱ | Runs full `len` steps ⏱⏱⏱       |
| All bytes equal   | Runs all steps ⏱⏱⏱        | Runs all steps ⏱⏱⏱              |

---

### ✅ Key takeaway

* **`memcmp` leaks timing information** (an attacker can guess how many bytes matched).
* **`sodium_memcmp` prevents timing leaks** by always taking exactly the same time.

---
