#[derive(Debug, Eq, PartialEq)]
pub struct Checksum {
    pub value: u8,
    pub valid: bool,
}

pub fn calc_checksum(bytes: &[u8]) -> u8 {
    #[allow(arithmetic_overflow)]
    let sum: u64 = bytes[0..bytes.len()].iter().map(|x| *x as u64).sum();
    (sum % 256) as u8
}
