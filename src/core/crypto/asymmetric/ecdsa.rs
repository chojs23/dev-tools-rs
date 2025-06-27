use anyhow::{anyhow, Result};
use hex;
use p256::{
    ecdsa::{
        signature::{Signer, Verifier},
        Signature, SigningKey, VerifyingKey,
    },
    PublicKey, SecretKey,
};
use sha2::{Digest, Sha256};

// ECDSA Functions
pub fn ecdsa_sign(message: &str, private_key_hex: &str) -> Result<String> {
    let private_key_bytes =
        hex::decode(private_key_hex).map_err(|_| anyhow!("Invalid hex private key format"))?;

    let secret_key = SecretKey::from_slice(&private_key_bytes)
        .map_err(|e| anyhow!("Failed to create secret key: {}", e))?;

    let signing_key = SigningKey::from(secret_key);

    // Hash the message
    let mut hasher = Sha256::new();
    hasher.update(message.as_bytes());
    let hash = hasher.finalize();

    let signature: Signature = signing_key.sign(&hash);
    Ok(hex::encode(signature.to_bytes()))
}

pub fn ecdsa_verify(message: &str, signature_hex: &str, public_key_hex: &str) -> Result<bool> {
    let public_key_bytes =
        hex::decode(public_key_hex).map_err(|_| anyhow!("Invalid hex public key format"))?;

    let public_key = PublicKey::from_sec1_bytes(&public_key_bytes)
        .map_err(|e| anyhow!("Failed to create public key: {}", e))?;

    let verifying_key = VerifyingKey::from(public_key);

    let signature_bytes =
        hex::decode(signature_hex).map_err(|_| anyhow!("Invalid hex signature format"))?;

    let signature = Signature::from_slice(&signature_bytes)
        .map_err(|e| anyhow!("Failed to parse signature: {}", e))?;

    // Hash the message
    let mut hasher = Sha256::new();
    hasher.update(message.as_bytes());
    let hash = hasher.finalize();

    match verifying_key.verify(&hash, &signature) {
        Ok(()) => Ok(true),
        Err(_) => Ok(false),
    }
}
