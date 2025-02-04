use std::env;
use std::process;

#[derive(Debug)]
enum ChunkStatus {
    Ok,
    BadChecksum,
    Invalid,
}

#[derive(Debug)]
struct Chunk {
    chunk_type: u8,
    length: u8,
    data: Vec<u8>,
    #[allow(dead_code)] // this is filled in but not currently read out again.
    checksum: u8,
}

#[derive(Debug)]
struct PossibleChunk {
    bytes: Vec<u8>,
    chunk: Option<Chunk>,
    status: ChunkStatus,
}

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

fn validate_checksum(bytes: &Vec<u8>) -> bool {
    #[allow(arithmetic_overflow)]
    let sum: u64 = bytes[0..(bytes.len() - 1)].iter().map(|x| *x as u64).sum();

    (sum % 256) as u8 == bytes[bytes.len() - 1]
}

fn decode_chunks(encoded: Vec<u8>) -> Vec<PossibleChunk> {
    let mut chunks = Vec::new();
    let mut index = 0;

    while index < encoded.len() {
        // Ensure there is enough data for a complete chunk
        if encoded.len() < index + 3 {
            chunks.push(PossibleChunk {
                bytes: encoded[index..].to_vec(),
                chunk: None,
                status: ChunkStatus::Invalid,
            });
            return chunks;
        }

        let chunk_type = encoded[index];
        let length = encoded[index + 1];

        // Ensure there is enough data for the data part of the chunk
        if encoded.len() < index + 2 + length as usize + 1 {
            chunks.push(PossibleChunk {
                bytes: encoded[index..].to_vec(),
                chunk: None,
                status: ChunkStatus::Invalid,
            });
            return chunks;
        }

        let data = encoded[index + 2..index + 2 + length as usize].to_vec();
        let full_chunk = encoded[index..index + 3 + length as usize].to_vec();
        let checksum = encoded[index + 2 + length as usize];

        let possible_chunk = PossibleChunk {
            chunk: Some(Chunk {
                chunk_type,
                length,
                data,
                checksum,
            }),

            status: match validate_checksum(&full_chunk) {
                true => ChunkStatus::Ok,
                false => ChunkStatus::BadChecksum,
            },
            bytes: full_chunk,
        };

        // Create a Chunk struct and push it to the result
        chunks.push(possible_chunk);

        // Move the index to the next chunk
        index += 2 + length as usize + 1; // type (1) + length (1) + data (length) + checksum (1)
    }

    chunks
}

/*
 * Given a value in the form of hex string as an argument, searches for chunks of the value that
 * could be in the TLVC format.
 *
 * This program was useful for checking that all commands are TLVC, and for showing that responses
 * are not TLVC.
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
    let chunks = decode_chunks(bytes);
    for chunk in chunks {
        match chunk.status {
            ChunkStatus::Ok => {
                let c = chunk.chunk.unwrap();
                println!(
                    "Valid.          Type: {:?}, length: {:?}, data: {:?}",
                    c.chunk_type, c.length, c.data
                );
            }
            ChunkStatus::BadChecksum => {
                let c = chunk.chunk.unwrap();
                println!(
                    "Wrong checksum. Type: {:?}, length: {:?}, data: {:?}",
                    c.chunk_type, c.length, c.data
                );
            }
            ChunkStatus::Invalid => {
                println!("Remainder.      Bytes: {:?}", chunk.bytes);
            }
        }
    }
}
