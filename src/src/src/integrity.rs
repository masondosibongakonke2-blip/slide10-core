use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IntegrityError {
    #[error("hash mismatch")]
    HashMismatch,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentAddressedBlob {
    pub label: String,
    pub data: Vec<u8>,
    pub digest_hex: String,
}

impl ContentAddressedBlob {
    pub fn new(label: impl Into<String>, data: Vec<u8>) -> Self {
        let digest_hex = sha256_hex(&data);
        Self { label: label.into(), data, digest_hex }
    }

    pub fn verify(&self) -> Result<(), IntegrityError> {
        if sha256_hex(&self.data) != self.digest_hex {
            Err(IntegrityError::HashMismatch)
        } else {
            Ok(())
        }
    }
}

pub fn sha256_hex(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}
