pub fn sodium_bin2hex(bin: &[u8]) -> String {
    // In libsodium, this writes into a caller-provided buffer.
    // In Rust, we'll just return a String.
    // Each byte becomes 2 hex chars, plus null terminator in C (not needed here).
    let mut hex = String::with_capacity(bin.len() * 2);

    for &byte in bin {
        let b = (byte >> 4) as u32;
        let c = (byte & 0x0f) as u32;

        // branchless hex encoding, matching the libsodium trick
        let hi = (87 + b + (((b.wrapping_sub(10)) >> 8) & !38)) as u8 as char;
        let lo = (87 + c + (((c.wrapping_sub(10)) >> 8) & !38)) as u8 as char;

        hex.push(hi);
        hex.push(lo);
    }

    hex
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sodium_bin2hex_empty() {
        assert_eq!(sodium_bin2hex(&[]), "");
    }

    #[test]
    fn test_sodium_bin2hex_basic() {
        assert_eq!(sodium_bin2hex(&[0x00]), "00");
        assert_eq!(sodium_bin2hex(&[0x0f]), "0f");
        assert_eq!(sodium_bin2hex(&[0xa5]), "a5");
        assert_eq!(sodium_bin2hex(&[0xff]), "ff");
    }

    #[test]
    fn test_sodium_bin2hex_multi() {
        let input = b"ABC"; // ASCII bytes 0x41 0x42 0x43
        assert_eq!(sodium_bin2hex(input), "414243");
    }
}
