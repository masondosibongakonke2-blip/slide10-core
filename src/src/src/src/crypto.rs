use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CryptoError {
    #[error("invalid signature")]
    InvalidSignature,
    #[error("invalid key bytes")]
    InvalidKey,
}

pub struct Keypair {
    pub signing: SigningKey,
    pub verifying: VerifyingKey,
}

impl Keypair {
    pub fn generate() -> Self {
        let signing = SigningKey::generate(&mut OsRng);
        let verifying = signing.verifying_key();
        Self { signing, verifying }
    }

    pub fn sign(&self, msg: &[u8]) -> Vec<u8> {
        self.signing.sign(msg).to_bytes().to_vec()
    }
}

pub fn verify(pubkey: &VerifyingKey, msg: &[u8], sig: &[u8]) -> Result<(), CryptoError> {
    let sig = Signature::from_slice(sig).map_err(|_| CryptoError::InvalidKey)?;
    pubkey.verify(msg, &sig).map_err(|_| CryptoError::InvalidSignature)
}
