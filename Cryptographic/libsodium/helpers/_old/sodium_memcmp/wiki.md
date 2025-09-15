When a comparison involves secret data (e.g. a key, an authentication tag, etc.), it is critical to use a **constant-time** comparison function. This property does not relate to computational complexity: it means the time needed to perform the comparison is the same for all data of the same size. The goal is to mitigate **side-channel** attacks.

The `sodium_memcmp()` function can be used for this purpose.

The function returns `0` if the `len` bytes pointed to by `b1_` match the `len` bytes pointed to by `b2_`. Otherwise, it returns `-1`.

**Note:** `sodium_memcmp()` is not a lexicographic comparator and is not a generic replacement for `memcmp()`.