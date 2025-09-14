```c
sodium_hex2bin(
    unsigned char *const bin,      // 1.
    const size_t bin_maxlen,       // 2.
    const char *const hex,         // 3.
    const size_t hex_len,          // 4.
    const char *const ignore,      // 5.
    size_t *const bin_len,         // 6.
    const char **const hex_end     // 7.
);
```

1. What is `size_t`?
   - It is **not** a Libsodium-specific type; it is defined by the C language in `<stddef.h>` (and therefore also in `<stdio.h>`, `<stdlib.h>`, …).
   - It is an **unsigned integer type** whose size is “big enough to hold the size in bytes of any object that can exist in memory”.
   - On 32-bit platforms it is usually `unsigned int` (32 bit), on 64-bit platforms it is usually `unsigned long` (64 bit).
   - You use it whenever you talk about object sizes, array lengths, or memory extents.  
     Example: `malloc(n * sizeof(int))` takes a `size_t`; `strlen()` returns a `size_t`.

2. What does “`*const`” mean?  
   Read C declarations **right-to-left**:

   ```
   unsigned char *const bin
   ^^^^^^^^^^^^^ ^^^^^ ^^^
        |          |    |
        |          |    +-- name of the parameter
        |          +------- “const” applies to the POINTER itself
        +------------------ type pointed to
   ```

   - `bin` is a **constant pointer** to `unsigned char`.
   - Inside the function you **may change the bytes** that `bin` points to (`bin[0] = 0xAB` is OK).
   - You **may NOT change the pointer value** (`bin = otherBuffer` is a compile-time error).
   - The caller’s pointer itself is unaffected; the qualifier only restricts what the *callee* can do.

3. What does “`**const`” mean?  
   Apply the same right-to-left rule:

   ```
   const char **const hex_end
   ^^^^^^^^^^^ ^^^^^ ^^^^^^^
        |         |     |
        |         |     +-- name
        |         +-------- “const” applies to the POINTER itself
        +------------------ type pointed to: “pointer to const char”
   ```

   - `hex_end` is a **constant pointer** (`*const`) to “pointer to `const char`”.
   - Inside the function you **may NOT repoint** `hex_end` (`hex_end = &somethingElse` is illegal).
   - You **may change** the thing it points **to** (i.e. you may write `*hex_end = hex + 16` to tell the caller where parsing stopped).
   - You **may NOT change the characters** that the inner pointer refers to, because they are `const char`.

Quick summary cheat-sheet:

| declaration               | can modify pointer? | can modify data? |
|---------------------------|---------------------|------------------|
| `T *p`                    | yes                 | yes              |
| `T *const p`              | no                  | yes              |
| `const T *p`              | yes                 | no               |
| `const T *const p`        | no                  | no               |
| `const T **const p`       | no (outer)          | no (inner data)  |

In the prototype of `sodium_hex2bin` every `*const` is simply telling the implementation “you are not allowed to redirect this pointer; you may only read or write the memory it already refers to”.