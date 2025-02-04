use std::{env, process};

fn hex_to_bytes(hex_str: &str) -> Result<Vec<u8>, String> {
    if hex_str.len() % 2 != 0 {
        return Err("Hex string must have an even length".to_string());
    }

    let mut bytes = Vec::new();
    for i in (0..hex_str.len()).step_by(2) {
        let byte_str = &hex_str[i..i + 2];
        match u8::from_str_radix(byte_str, 16) {
            Ok(byte) => bytes.push(byte),
            Err(_) => return Err(format!("Invalid hex character in: {}", byte_str)),
        }
    }
    Ok(bytes)
}

fn calculate_checksum(bytes: &[u8]) -> u8 {
    #[allow(arithmetic_overflow)]
    let sum: u64 = bytes[0..bytes.len()].iter().map(|x| *x as u64).sum();

    (sum % 256) as u8
}

/*
 * Takes a hex string and searches for a byte that could be a checksum of all previous bytes.
 * This has been useful when seeing if there is any structure to the notifications sent by the
 * thermometer.
 */
fn main() {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: <program> <hex_string>");
        process::exit(1);
    }

    let hex_str = &args[1];
    let bytes = hex_to_bytes(hex_str).unwrap();

    for i in 2..bytes.len() {
        let c = calculate_checksum(&bytes[0..i]);
        println!(
            "Length: {:?}, checksum: {:x}, expected: {:x}, match? {:?}",
            i,
            c,
            bytes[i],
            c == bytes[i]
        );
    }
}
