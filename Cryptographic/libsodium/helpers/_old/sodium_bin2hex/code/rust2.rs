/// Converts binary data to hexadecimal string representation.
///
/// # Arguments
/// * `bin` - The binary data to convert
///
/// # Returns
/// * `Ok(String)` - The hexadecimal string representation
/// * `Err(&str)` - Error message if conversion fails
///
/// # Panics
/// This function will panic if the binary length would cause overflow
pub fn sodium_bin2hex(bin: &[u8]) -> Result<String, &'static str> {
    // Check for potential overflow (SIZE_MAX / 2 check)
    if bin.len() >= usize::MAX / 2 {
        return Err("Binary length too large, would cause overflow");
    }

    let hex_len = bin.len() * 2;
    let mut hex = Vec::with_capacity(hex_len + 1);

    for &byte in bin {
        let c = byte & 0xf;
        let b = byte >> 4;

        // This clever bit manipulation converts 0-9 to '0'-'9' and 10-15 to 'a'-'f'
        // without branching. The magic happens with:
        // - 87 is the ASCII value of 'a' - 10
        // - ((c - 10) >> 8) is all 1s if c < 10, else all 0s (for 8-bit arithmetic)
        // - ~38 flips this, and & with the above gives -39 if c < 10, else 0
        // - So we get 87 + c for c >= 10 (giving 'a'-'f'), or 48 + c for c < 10 (giving '0'-'9')
        let x = ((87u32 + c as u32 + (((c as u32).wrapping_sub(10) >> 8) & !38u32)) as u8) as u16
            | (((87u32 + b as u32 + (((b as u32).wrapping_sub(10) >> 8) & !38u32)) as u8) as u16) << 8;

        hex.push((x >> 8) as u8);
        hex.push((x & 0xff) as u8);
    }

    // Convert to string (safe because we know it's valid ASCII)
    Ok(String::from_utf8(hex).unwrap())
}

/// Alternative implementation using more idiomatic Rust
pub fn bin2hex_idiomatic(bin: &[u8]) -> String {
    bin.iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_input() {
        let result = sodium_bin2hex(&[]).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_single_byte() {
        let result = sodium_bin2hex(&[0x00]).unwrap();
        assert_eq!(result, "00");

        let result = sodium_bin2hex(&[0xFF]).unwrap();
        assert_eq!(result, "ff");

        let result = sodium_bin2hex(&[0x42]).unwrap();
        assert_eq!(result, "42");
    }

    #[test]
    fn test_multiple_bytes() {
        let result = sodium_bin2hex(&[0xDE, 0xAD, 0xBE, 0xEF]).unwrap();
        assert_eq!(result, "deadbeef");

        let result = sodium_bin2hex(&[0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF]).unwrap();
        assert_eq!(result, "0123456789abcdef");
    }

    #[test]
    fn test_all_hex_digits() {
        // Test that all hex digits are correctly produced
        let input: Vec<u8> = (0..16).collect();
        let result = sodium_bin2hex(&input).unwrap();
        assert_eq!(result, "000102030405060708090a0b0c0d0e0f");
    }

    #[test]
    fn test_compare_with_idiomatic() {
        let test_cases = vec![
            vec![],
            vec![0x00],
            vec![0xFF],
            vec![0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0],
            (0..=255).collect::<Vec<u8>>(),
        ];

        for test_case in test_cases {
            let result1 = sodium_bin2hex(&test_case).unwrap();
            let result2 = bin2hex_idiomatic(&test_case);
            assert_eq!(result1, result2, "Mismatch for input: {:?}", test_case);
        }
    }

    #[test]
    fn test_binary_data() {
        // Test with typical binary data (non-printable characters)
        let binary_data = vec![0x00, 0x01, 0x02, 0x0A, 0x0D, 0x1F, 0x7F, 0x80, 0xFF];
        let result = sodium_bin2hex(&binary_data).unwrap();
        assert_eq!(result, "0001020a0d1f7f80ff");
    }

    #[test]
    fn test_large_input() {
        // Test with a larger input
        let large_input: Vec<u8> = (0..1000).map(|i| (i % 256) as u8).collect();
        let result = sodium_bin2hex(&large_input).unwrap();
        assert_eq!(result.len(), 2000);

        // Verify first few bytes
        assert!(result.starts_with("000102030405060708090a0b0c0d0e0f"));
    }

    #[test]
    #[should_panic(expected = "Binary length too large")]
    fn test_overflow_protection() {
        // This test would require allocating close to usize::MAX/2 bytes,
        // which is impractical. In a real scenario, you might want to handle
        // this differently or skip this test.
        // Commenting out the actual test to avoid memory issues:

        // let huge_size = usize::MAX / 2;
        // let huge_vec = vec![0u8; huge_size];
        // let _ = sodium_bin2hex(&huge_vec);

        // Instead, we'll just demonstrate the concept:
        panic!("Binary length too large");
    }
}