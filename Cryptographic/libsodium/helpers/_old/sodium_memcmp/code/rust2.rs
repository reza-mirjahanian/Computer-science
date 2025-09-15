/// Compares two byte slices in constant time.
///
/// Returns 0 if the first `len` bytes of `b1` and `b2` are identical,
/// and -1 if any byte differs.
///
/// # Panics
///
/// Panics if `b1.len() < len` or `b2.len() < len`.
pub fn sodium_memcmp(b1: &[u8], b2: &[u8]) -> i32 {
    if b1.len() != b2.len() {
        return -1;
    }

    let len = b1.len();

    // Accumulate all XOR differences into `d`
    let mut d: u8 = 0;
    for i in 0..len {
        d |= b1[i] ^ b2[i];
    }

    // Replicate C’s (1 & ((d - 1) >> 8)) - 1 logic:
    //  - If d == 0, then (0 - 1) >> 8 = -1 (arithmetic shift),
    //    so (1 & -1) - 1 = 1 - 1 = 0.
    //  - If d != 0, then (d - 1) ≥ 0, so (d - 1) >> 8 = 0,
    //    so (1 & 0) - 1 = 0 - 1 = -1.
    let t = ((d as i32) - 1) >> 8;
    (1 & t) - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equal() {
        assert_eq!(sodium_memcmp(b"hello", b"hello"), 0);
    }

    #[test]
    fn test_not_equal() {
        assert_eq!(sodium_memcmp(b"hello", b"world"), -1);
        assert_eq!(sodium_memcmp(b"hello", b"hell"), -1); // different length
        assert_eq!(sodium_memcmp(b"hello", b"hello!"), -1); // different length
    }

    #[test]
    fn test_empty() {
        assert_eq!(sodium_memcmp(b"", b""), 0);
    }
}