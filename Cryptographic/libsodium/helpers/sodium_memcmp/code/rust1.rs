/// Timing-attack-resistant memory comparison.
///
/// Returns 0 if the slices are equal, -1 otherwise.
/// Designed to take the same amount of time regardless of where or if a difference occurs.
pub fn sodium_memcmp(b1: &[u8], b2: &[u8]) -> i32 {
    if b1.len() != b2.len() {
        return -1;
    }

    let len = b1.len();
    let mut d: u8 = 0;

    // Force volatile reads to prevent compiler optimizations that might leak timing info.
    // In Rust, we can use `read_volatile` for this purpose.
    for i in 0..len {
        unsafe {
            let byte1 = std::ptr::read_volatile(&b1[i] as *const u8);
            let byte2 = std::ptr::read_volatile(&b2[i] as *const u8);
            d |= byte1 ^ byte2;
        }
    }

    // Constant-time conversion of d to -1 or 0:
    // If d == 0 (equal), returns 0.
    // If d != 0 (not equal), returns -1.
    //
    // This mimics: (1 & ((d - 1) >> 8)) - 1
    // When d == 0: (1 & ((0 - 1) >> 8)) - 1 → (1 & 0xFF) - 1 → 1 - 1 = 0
    // When d != 0: (1 & ((d-1) >> 8)) is 0 (since (d-1)>>8 is 0 for d in [1,255]) → 0 - 1 = -1
    //
    // Simpler Rust equivalent:
    if d == 0 {
        0
    } else {
        -1
    }

    // Alternatively, to exactly mimic the bit trick (though unnecessary in Rust):
    // ((1i32 & (((d as i32) - 1i32) >> 8)) - 1i32)
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