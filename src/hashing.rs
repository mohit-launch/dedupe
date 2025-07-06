use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use blake3;
use sha2::{Digest, Sha256};
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
                if n == 0 {
                    break;
                }
                hasher.update(&buffer[..n]);
            }
            Ok(format!("{:x}", hasher.finalize()))
        }

        HashAlgorithm::Blake3 => {
            let mut hasher = blake3::Hasher::new();
            let mut buffer = [0u8; 8192];
            while let Ok(n) = reader.read(&mut buffer) {
                if n == 0 {
                    break;
                }
                hasher.update(&buffer[..n]);
            }
            Ok(hasher.finalize().to_hex().to_string())
        }

        HashAlgorithm::XxHash => {
            let mut hasher = Xxh3::new();
            let mut buffer = [0u8; 8192];
            while let Ok(n) = reader.read(&mut buffer) {
                if n == 0 {
                    break;
                }
                hasher.update(&buffer[..n]);
            }
            Ok(format!("{:x}", hasher.digest()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::PathBuf;

    fn create_temp_file(content: &str) -> PathBuf {
        let path = PathBuf::from("temp_test_file.txt");
        let mut file = File::create(&path).unwrap();
        write!(file, "{}", content).unwrap();
        path
    }

    #[test]
    fn test_sha256_hash_output_length() {
        let path = create_temp_file("hello sha256");
        let hash = compute_hash(&path, HashAlgorithm::SHA256).unwrap();
        assert_eq!(hash.len(), 64);
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_blake3_hash_output_length() {
        let path = create_temp_file("hello blake3");
        let hash = compute_hash(&path, HashAlgorithm::Blake3).unwrap();
        assert_eq!(hash.len(), 64);
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_xxhash_output_length() {
        let path = create_temp_file("hello xxhash");
        let hash = compute_hash(&path, HashAlgorithm::XxHash).unwrap();
        assert_eq!(hash.len(), 16);
        fs::remove_file(path).unwrap();
    }
}
