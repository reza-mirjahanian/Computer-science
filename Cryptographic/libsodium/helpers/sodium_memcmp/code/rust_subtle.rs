//[dependencies]
// subtle = "2"


use subtle::ConstantTimeEq;

pub fn sodium_memcmp(b1: &[u8], b2: &[u8]) -> i32 {
    if b1.len() != b2.len() {
        return -1;
    }
    if b1.ct_eq(b2).into() {
        0
    } else {
        -1
    }
}