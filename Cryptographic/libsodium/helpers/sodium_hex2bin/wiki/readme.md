`sodium_hex2bin` is a *very* carefully written, **constant-time** hex decoder.  
It turns the text sequence

```
31 32 33 34 35 36 37 38 39 41 42 43 44 45 46
```

into the 8-byte binary value

```
0x12 0x34 0x56 0x78 0x9A 0xBC 0xDE 0xF0
```

without ever branching on secret data and without using any lookup tables that
could leak information through the cache.  
Below is a line-by-line walk-through that shows *what* is done and *why* it is
done that way.

--------------------------------------------------------------------
1.  Prototype
--------------------------------------------------------------------
```c
int sodium_hex2bin(unsigned char *const bin,     /* output buffer          */
                   const size_t bin_maxlen,      /* size of that buffer    */
                   const char *const hex,        /* NUL-terminated or not  */
                   const size_t hex_len,         /* #chars to scan         */
                   const char *const ignore,     /* optional skip-list     */
                   size_t *const bin_len,        /* #bytes written         */
                   const char **const hex_end)   /* 1st unprocessed char   */
```

Return value  
 0  – success  
-1  – failure (incomplete pair, illegal char, buffer too small, …)

--------------------------------------------------------------------
2.  Local variables
--------------------------------------------------------------------
```c
size_t bin_pos = 0;     /* next free slot in output buffer          */
size_t hex_pos = 0;     /* current position inside hex string       */
int ret = 0;            /* return code                              */
unsigned char c;        /* current character                        */
unsigned char c_acc;    /* accumulator: high nibble << 4            */
unsigned char state;    /* 0 = waiting for high nibble              */
                        /* 1 = waiting for low  nibble              */
```

The next six variables are the *constant-time* trick.  
They hold *boolean* flags (0 or 0xFF) that tell whether the character is
a decimal digit, an upper-case/lower-case hex letter, or something else.
No branches, no table lookups.

--------------------------------------------------------------------
3.  Main loop – one character at a time
--------------------------------------------------------------------
```c
while (hex_pos < hex_len) {
    c = (unsigned char)hex[hex_pos];
```

--------------------------------------------------------------------
4.  Classify the character in constant time
--------------------------------------------------------------------
```c
c_num    = c ^ 48U;               /* c_num = c - '0'   (if c in '0'..'9') */
c_num0   = (c_num - 10U) >> 8;    /* 0xFF if c_num < 10, 0 otherwise      */

c_alpha  = (c & ~32U) - 55U;      /* map A-F or a-f → 10-15               */
c_alpha0 = ((c_alpha - 10U) ^ (c_alpha - 16U)) >> 8;
/* 0xFF if 10 ≤ c_alpha ≤ 15, 0 otherwise */
```

`c_num0 | c_alpha0` is therefore 0xFF for a *valid* hex character and 0
for anything else.

--------------------------------------------------------------------
5.  Handle “ignore” list
--------------------------------------------------------------------
```c
if ((c_num0 | c_alpha0) == 0U) {          /* invalid hex char ?         */
    if (ignore != NULL && state == 0 && strchr(ignore, c))
    {   hex_pos++;  continue;   }         /* skip it, stay in state 0   */
    break;                                /* real syntax error          */
}
```

--------------------------------------------------------------------
6.  Convert the character to its 4-bit value
--------------------------------------------------------------------
```c
c_val = (c_num0 & c_num) | (c_alpha0 & c_alpha);
```
`c_val` is now 0-15 and *independent* of the branch taken above.

--------------------------------------------------------------------
7.  Build the byte
--------------------------------------------------------------------
```c
if (bin_pos >= bin_maxlen) { ret = -1; errno = ERANGE; break; }

if (state == 0) {
    c_acc = c_val * 16U;          /* high nibble shifted left by 4 */
} else {
    bin[bin_pos++] = c_acc | c_val;
}
state = ~state;                   /* toggle between 0 and 0xFF     */
hex_pos++;
```

--------------------------------------------------------------------
8.  Post-loop checks
--------------------------------------------------------------------
```c
if (state != 0) {          /* odd number of hex digits? */
    hex_pos--;
    errno = EINVAL;
    ret = -1;
}
```

--------------------------------------------------------------------
9.  Final bookkeeping
--------------------------------------------------------------------
```c
if (ret != 0) bin_pos = 0;          /* do not give partial output */

if (hex_end) {
    *hex_end = &hex[hex_pos];       /* let caller know where we stopped */
} else if (hex_pos != hex_len) {    /* caller wants full consumption */
    errno = EINVAL;
    ret  = -1;
}

if (bin_len) *bin_len = bin_pos;
return ret;
```

--------------------------------------------------------------------
10.  Usage example
--------------------------------------------------------------------
```c
unsigned char key[32];
size_t key_len;
const char *end;

if (sodium_hex2bin(key, sizeof key,
                   "deadBEEF…90 hex chars…", 64,
                   " \n",          /* ignore spaces and new-lines */
                   &key_len, &end) == 0) {
    /* key now contains 32 raw bytes */
}
```

--------------------------------------------------------------------
11.  Take-away
--------------------------------------------------------------------
- The function is **branch-free** on the *value* of the data (only on
  meta-conditions such as “buffer full”).  
- No lookup tables → immune to cache-timing attacks.  
- Accepts an optional skip-list so that human-readable separators can be
  inserted in the hex string.  
- Returns precise error information while leaving the output buffer
  untouched on failure.

That is the whole story—`sodium_hex2bin` is nothing more (and nothing
less) than a side-channel-resistant hex decoder.