use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use sha2::{Sha256, Digest};
use blake3;
use xxhash_rust::xxh3::Xxh3;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum HashAlgorithm {
    SHA256,
    Blake3,
    XxHash,
}

pub fn compute_hash(path: &Path, algo: HashAlgorithm) -> std::io::Result<String> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    match algo {
        HashAlgorithm::SHA256 => {
            let mut hasher = Sha256::new();
            let mut buffer = [0u8; 8192];
            while let Ok(n) = reader.read(&mut buffer) {
                if n == 0 { break; }
                hasher.update(&buffer[..n]);
            }
            Ok(format!("{:x}", hasher.finalize()))
        }

        HashAlgorithm::Blake3 => {
            let mut hasher = blake3::Hasher::new();
            let mut buffer = [0u8; 8192];
            while let Ok(n) = reader.read(&mut buffer) {
                if n == 0 { break; }
                hasher.update(&buffer[..n]);
            }
            Ok(hasher.finalize().to_hex().to_string())
        }

        HashAlgorithm::XxHash => {
            let mut hasher = Xxh3::new();
            let mut buffer = [0u8; 8192];
            while let Ok(n) = reader.read(&mut buffer) {
                if n == 0 { break; }
                hasher.update(&buffer[..n]);
            }
            Ok(format!("{:x}", hasher.digest()))
        }
    }
}
