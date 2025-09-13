Below is a walk-through that treats the routine exactly the way a C teacher would in class:  
“Here is a byte-to-hex converter that is deliberately branch-free and table-free.  
Let’s see how it works, why it is safe, and what every single operator does.”

------------------------------------------------
1.  WHAT THE FUNCTION PROMISES
------------------------------------------------
Prototype  
char *sodium_bin2hex(char * hex,            /* output buffer               */
                     size_t hex_maxlen,     /* size of that buffer         */
                     const unsigned char *bin,  /* raw bytes to convert    */
                     size_t bin_len);       /* how many bytes              */

Guarantee  
Convert bin_len raw bytes into 2·bin_len hexadecimal characters  
(lower-case, NUL-terminated) and return the pointer hex.

------------------------------------------------
2.  THE SAFETY CHECK (the only branch in the code)
------------------------------------------------
if (bin_len >= SIZE_MAX/2 || hex_maxlen <= bin_len*2U)  
    sodium_misuse();          /* never returns – aborts the program */

Reasoning  
- 2 hex chars per byte → worst-case output length is 2·bin_len.  
- If bin_len ≥ SIZE_MAX/2 the multiplication 2·bin_len would overflow.  
- If the supplied buffer is too small we cannot fulfil the contract.  
Both situations are a programming error, so the library kills the process.

------------------------------------------------
3.  HEX ENCODING IN ONE LINE (the magic!)
------------------------------------------------
For every byte we need two ASCII characters:

high nibble  low nibble  
┌────┬────┐  
│  b │  c │  
└────┴────┘

ASCII values we want  
0-9  → '0'..'9'  (0x30..0x39)  
10-15→ 'a'..'f'  (0x61..0x66)

The code computes both characters without an if-statement:

x = (unsigned char)(87U + c + (((c - 10U) >> 8) & ~38U)) << 8 |
    (unsigned char)(87U + b + (((b - 10U) >> 8) & ~38U));

Let’s dissect the sub-expression for one nibble (c or b):

step 1:   t = nibble - 10;           // 0-5 for 10-15, large negative for 0-9  
step 2:   t >> 8                     // arithmetic right shift gives 0x00 or 0xFF  
step 3:   (t >> 8) & ~38U            // 0 or (~38U = 0xFFFFFFDA)  
step 4:   87U + nibble + …           // 87 is 'W' (0x57), the magic offset

Truth table for the low nibble c

c | c-10 | (c-10)>>8 | ((…)&~38) | 87+c+(…) | char
--|------|-----------|-----------|----------|------
0 | -10  | 0xFF      | 0xFFFFFFDA| 0x30     | '0'
… |  …   |    …      |    …      |  …       |  …
9 | -1   | 0xFF      | 0xFFFFFFDA| 0x39     | '9'
10|  0   | 0x00      | 0x00      | 0x61     | 'a'
11|  1   | 0x00      | 0x00      | 0x62     | 'b'
… |  …   |    …      |    …      |  …       |  …
15|  5   | 0x00      | 0x00      | 0x66     | 'f'

Exactly the ASCII codes we need.

------------------------------------------------
4.  STORING THE TWO CHARACTERS
------------------------------------------------
The 16-bit variable x now holds

high char in high byte   │   low char in low byte  
[  8 bits   ] [  8 bits  ]

hex[i*2]   = (char)x;      // low byte  → first hex char  
x >>= 8;                   // move high byte down  
hex[i*2+1] = (char)x;      // high byte → second hex char

------------------------------------------------
5.  TERMINATION
------------------------------------------------
After the loop finishes we have written 2·bin_len characters.  
The NUL terminator is placed at hex[i*2] (i == bin_len at that point).

------------------------------------------------
6.  COMPLETE PICTURE
------------------------------------------------
bin[]  (3 bytes)        ┌──┬──┬──┐
                        │4F│63│39│
                        └──┴──┴──┘
hex[] (7 bytes)  ┌──┬──┬──┬──┬──┬──┬──┐
                 │4f│63│39│00│  │  │  │
                 └──┴──┴──┴──┴──┴──┴──┘
                  ↑              ↑
               returned       terminator

------------------------------------------------
7.  KEY TAKE-AWAYS FOR A C STUDENT
------------------------------------------------
1.  Branch-free code can be faster and avoids timing leaks in crypto code.  
2.  Bit-wise operators (&, >>, ~) replace conditional logic.  
3.  Arithmetic right shift of a negative number gives all-1 bits (2-complement).  
4.  Always check for overflow before any pointer write; libraries prefer to abort
    rather than return an error that callers will ignore.  
5.  The constant 87 is not black magic: 87 − 10 + '0' = 'a' − 10, so the same
    offset works for both ranges after the conditional correction.

------------------------------------------------
8.  MINIMAL RE-WRITE FOR CLARITY
------------------------------------------------
If you want to see the same logic in plain C (not branch-free):

static char nib2hex(unsigned char n)
{
    return n < 10 ? (char)('0' + n) : (char)('a' + n - 10);
}

void bin2hex_simple(char *hex, const unsigned char *bin, size_t len)
{
    for (size_t i = 0; i < len; ++i) {
        hex[i*2]   = nib2hex(bin[i] >> 4);
        hex[i*2+1] = nib2hex(bin[i] & 0xF);
    }
    hex[len*2] = '\0';
}

Compare the two versions and you will appreciate the original’s ingenuity.