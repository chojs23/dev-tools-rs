use anyhow::{anyhow, Result};
use hex;
use p256::SecretKey;
use rand::thread_rng;
use rsa::pkcs1::{DecodeRsaPrivateKey, DecodeRsaPublicKey};
use rsa::pkcs8::{DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey};
use rsa::{Pkcs1v15Encrypt, Pkcs1v15Sign, RsaPrivateKey, RsaPublicKey};
use sha2::{Digest, Sha256};

// RSA Functions
pub fn rsa_encrypt(plaintext: &str, public_key_pem: &str) -> Result<String> {
    let public_key = if public_key_pem.starts_with("-----BEGIN PUBLIC KEY-----") {
        RsaPublicKey::from_public_key_pem(public_key_pem)
            .map_err(|e| anyhow!("Failed to parse public key (PKCS#8): {}", e))?
    } else if public_key_pem.starts_with("-----BEGIN RSA PUBLIC KEY-----") {
        RsaPublicKey::from_pkcs1_pem(public_key_pem)
            .map_err(|e| anyhow!("Failed to parse public key (PKCS#1): {}", e))?
    } else {
        return Err(anyhow!("Invalid public key format. Must be PEM format."));
    };

    let plaintext_bytes = plaintext.as_bytes();
    let ciphertext = public_key
        .encrypt(&mut thread_rng(), Pkcs1v15Encrypt, plaintext_bytes)
        .map_err(|e| anyhow!("RSA encryption failed: {}", e))?;

    Ok(hex::encode(ciphertext))
}

pub fn rsa_decrypt(ciphertext: &str, private_key_pem: &str) -> Result<String> {
    let private_key = if private_key_pem.starts_with("-----BEGIN PRIVATE KEY-----") {
        RsaPrivateKey::from_pkcs8_pem(private_key_pem)
            .map_err(|e| anyhow!("Failed to parse private key (PKCS#8): {}", e))?
    } else if private_key_pem.starts_with("-----BEGIN RSA PRIVATE KEY-----") {
        RsaPrivateKey::from_pkcs1_pem(private_key_pem)
            .map_err(|e| anyhow!("Failed to parse private key (PKCS#1): {}", e))?
    } else {
        return Err(anyhow!("Invalid private key format. Must be PEM format."));
    };

    let ciphertext_bytes =
        hex::decode(ciphertext).map_err(|_| anyhow!("Invalid hex ciphertext format"))?;

    let plaintext = private_key
        .decrypt(Pkcs1v15Encrypt, &ciphertext_bytes)
        .map_err(|e| anyhow!("RSA decryption failed: {}", e))?;

    String::from_utf8(plaintext).map_err(|e| anyhow!("Invalid UTF-8 in decrypted text: {}", e))
}

pub fn rsa_sign(message: &str, private_key_pem: &str) -> Result<String> {
    let private_key = if private_key_pem.starts_with("-----BEGIN PRIVATE KEY-----") {
        RsaPrivateKey::from_pkcs8_pem(private_key_pem)
            .map_err(|e| anyhow!("Failed to parse private key (PKCS#8): {}", e))?
    } else if private_key_pem.starts_with("-----BEGIN RSA PRIVATE KEY-----") {
        RsaPrivateKey::from_pkcs1_pem(private_key_pem)
            .map_err(|e| anyhow!("Failed to parse private key (PKCS#1): {}", e))?
    } else {
        return Err(anyhow!("Invalid private key format. Must be PEM format."));
    };

    // Hash the message
    let mut hasher = Sha256::new();
    hasher.update(message.as_bytes());
    let hash = hasher.finalize();

    let signature = private_key
        .sign(Pkcs1v15Sign::new::<Sha256>(), &hash)
        .map_err(|e| anyhow!("RSA signing failed: {}", e))?;

    Ok(hex::encode(signature))
}

pub fn rsa_verify(message: &str, signature_hex: &str, public_key_pem: &str) -> Result<bool> {
    let public_key = if public_key_pem.starts_with("-----BEGIN PUBLIC KEY-----") {
        RsaPublicKey::from_public_key_pem(public_key_pem)
            .map_err(|e| anyhow!("Failed to parse public key (PKCS#8): {}", e))?
    } else if public_key_pem.starts_with("-----BEGIN RSA PUBLIC KEY-----") {
        RsaPublicKey::from_pkcs1_pem(public_key_pem)
            .map_err(|e| anyhow!("Failed to parse public key (PKCS#1): {}", e))?
    } else {
        return Err(anyhow!("Invalid public key format. Must be PEM format."));
    };

    let signature =
        hex::decode(signature_hex).map_err(|_| anyhow!("Invalid hex signature format"))?;

    // Hash the message
    let mut hasher = Sha256::new();
    hasher.update(message.as_bytes());
    let hash = hasher.finalize();

    match public_key.verify(Pkcs1v15Sign::new::<Sha256>(), &hash, &signature) {
        Ok(()) => Ok(true),
        Err(_) => Ok(false),
    }
}
// Key generation functions
pub fn generate_rsa_keypair() -> Result<(String, String)> {
    let mut rng = thread_rng();
    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits)
        .map_err(|e| anyhow!("Failed to generate RSA keypair: {}", e))?;

    let public_key = RsaPublicKey::from(&private_key);

    let private_key_pem = private_key
        .to_pkcs8_pem(rsa::pkcs8::LineEnding::LF)
        .map_err(|e| anyhow!("Failed to encode private key: {}", e))?;

    let public_key_pem = public_key
        .to_public_key_pem(rsa::pkcs8::LineEnding::LF)
        .map_err(|e| anyhow!("Failed to encode public key: {}", e))?;

    Ok((public_key_pem, private_key_pem.to_string()))
}

pub fn generate_ecdsa_keypair() -> Result<(String, String)> {
    let secret_key = SecretKey::random(&mut thread_rng());
    let public_key = secret_key.public_key();

    let private_key_hex = hex::encode(secret_key.to_bytes());
    let public_key_hex = hex::encode(public_key.to_sec1_bytes());

    Ok((public_key_hex, private_key_hex))
}
