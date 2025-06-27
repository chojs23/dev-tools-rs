use aes::{Aes128, Aes192, Aes256};
use anyhow::{anyhow, Result};
use cbc::{Decryptor, Encryptor};
use cipher::block_padding::Pkcs7;
use cipher::{BlockDecryptMut, BlockEncryptMut, KeyInit, KeyIvInit};
use ecb::{Decryptor as EcbDecryptor, Encryptor as EcbEncryptor};
use hex;
use rand::{thread_rng, RngCore};
use serde::{Deserialize, Serialize};

use crate::core::crypto::CipherMode;

type Aes128CbcEnc = Encryptor<Aes128>;
type Aes128CbcDec = Decryptor<Aes128>;
type Aes192CbcEnc = Encryptor<Aes192>;
type Aes192CbcDec = Decryptor<Aes192>;
type Aes256CbcEnc = Encryptor<Aes256>;
type Aes256CbcDec = Decryptor<Aes256>;
type Aes128EcbEnc = EcbEncryptor<Aes128>;
type Aes128EcbDec = EcbDecryptor<Aes128>;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AesKeySize {
    Aes128,
    Aes192,
    Aes256,
}

fn validate_key_size(key: &str, key_size: AesKeySize) -> Result<()> {
    let key_length = key.len();
    match key_size {
        AesKeySize::Aes128 if key_length != 16 => Err(anyhow!(
            "AES-128 requires a 16-byte (32 hex characters) key"
        )),
        AesKeySize::Aes192 if key_length != 24 => Err(anyhow!(
            "AES-192 requires a 24-byte (48 hex characters) key"
        )),
        AesKeySize::Aes256 if key_length != 32 => Err(anyhow!(
            "AES-256 requires a 32-byte (64 hex characters) key"
        )),
        _ => Ok(()),
    }
}

fn encrypt_aes_128_cbc(plaintext: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>> {
    let mut buf = vec![0u8; plaintext.len() + 16]; // 16 bytes for padding

    let ct = Aes128CbcEnc::new(key.into(), iv.into())
        .encrypt_padded_b2b_mut::<Pkcs7>(plaintext, &mut buf)
        .unwrap();

    Ok(ct.to_vec())
}

fn encrypt_aes_128_ecb(plaintext: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    let mut buf = vec![0u8; plaintext.len() + 16];

    let ct = Aes128EcbEnc::new(key.into())
        .encrypt_padded_b2b_mut::<Pkcs7>(plaintext, &mut buf)
        .unwrap();

    Ok(ct.to_vec())
}

// AES Functions
pub fn aes_encrypt(
    plaintext: &str,
    key: &str,
    key_size: AesKeySize,
    mode: CipherMode,
    iv: Option<&str>,
) -> Result<String> {
    validate_key_size(key, key_size)?;

    let key_bytes = key.as_bytes();

    let plaintext_bytes = plaintext.as_bytes();

    match mode {
        CipherMode::CBC => {
            let iv_bytes: &[u8; 16] = if let Some(iv) = iv {
                if iv.len() != 16 {
                    return Err(anyhow!(
                        "CBC mode requires a 16-byte (32 hex characters) IV"
                    ));
                }
                iv.as_bytes()
                    .try_into()
                    .map_err(|_| anyhow!("IV must be exactly 16 bytes long"))?
            } else {
                &[0u8; 16] // Default IV of zeros if not provided
            };

            let ct = encrypt_aes_128_cbc(plaintext_bytes, key_bytes, iv_bytes)?;
            Ok(hex::encode(ct))
        }
        CipherMode::ECB => {
            let ct = encrypt_aes_128_ecb(plaintext_bytes, key_bytes)?;

            Ok(hex::encode(ct))
        }
    }
}

pub fn aes_decrypt(
    ciphertext: &str,
    key: &str,
    mode: CipherMode,
    iv: Option<&str>,
) -> Result<String> {
    if key.len() != 16 {
        return Err(anyhow!(
            "AES-128 requires a 16-byte (32 hex characters) key"
        ));
    }

    let key_bytes: [u8; 16] = key
        .as_bytes()
        .try_into()
        .map_err(|_| anyhow!("Key must be exactly 16 bytes long"))?;

    let ciphertext_bytes =
        hex::decode(ciphertext).map_err(|_| anyhow!("Invalid hex ciphertext format"))?;

    let ciphertext_bytes: [u8; 48] = ciphertext_bytes
        .try_into()
        .map_err(|_| anyhow!("Ciphertext must be exactly 48 bytes long"))?;

    match mode {
        CipherMode::CBC => {
            let iv_str = iv.ok_or_else(|| anyhow!("IV required for CBC mode"))?;
            let iv_bytes = hex::decode(iv_str).map_err(|_| anyhow!("Invalid hex IV format"))?;

            if iv_bytes.len() != 16 {
                return Err(anyhow!("AES requires a 16-byte (32 hex characters) IV"));
            }

            let iv_bytes: [u8; 16] = iv_bytes
                .try_into()
                .map_err(|_| anyhow!("IV must be exactly 16 bytes long"))?;

            let mut buf = [0u8; 48];
            let pt = Aes128CbcDec::new(&key_bytes.into(), &iv_bytes.into())
                .decrypt_padded_b2b_mut::<Pkcs7>(&ciphertext_bytes, &mut buf)
                .unwrap();

            String::from_utf8(ciphertext_bytes[..pt.len()].to_vec())
                .map_err(|e| anyhow!("Invalid UTF-8 in decrypted text: {}", e))
        }
        CipherMode::ECB => {
            let mut buf = [0u8; 48];
            let pt = Aes128EcbDec::new(&key_bytes.into())
                .decrypt_padded_b2b_mut::<Pkcs7>(&ciphertext_bytes, &mut buf)
                .unwrap();

            String::from_utf8(ciphertext_bytes[..pt.len()].to_vec())
                .map_err(|e| anyhow!("Invalid UTF-8 in decrypted text: {}", e))
        }
    }
}

// RC4 Functions
pub fn rc4_encrypt(plaintext: &str, key: &str) -> Result<String> {
    return Err(anyhow!("RC4 encryption is not implemented yet"));
    // let key_bytes = hex::decode(key).map_err(|_| anyhow!("Invalid hex key format"))?;
    //
    // if key_bytes.is_empty() || key_bytes.len() > 256 {
    //     return Err(anyhow!("RC4 key must be between 1 and 256 bytes"));
    // }
    //
    // let mut cipher = rc4::Rc4::new(&key_bytes);
    // let mut plaintext_bytes = plaintext.as_bytes().to_vec();
    // cipher.apply_keystream(&mut plaintext_bytes);
    //
    // Ok(hex::encode(plaintext_bytes))
}

pub fn rc4_decrypt(ciphertext: &str, key: &str) -> Result<String> {
    return Err(anyhow!("RC4 decryption is not implemented yet"));
    // let key_bytes = hex::decode(key).map_err(|_| anyhow!("Invalid hex key format"))?;
    //
    // if key_bytes.is_empty() || key_bytes.len() > 256 {
    //     return Err(anyhow!("RC4 key must be between 1 and 256 bytes"));
    // }
    //
    // let mut ciphertext_bytes =
    //     hex::decode(ciphertext).map_err(|_| anyhow!("Invalid hex ciphertext format"))?;
    //
    // let mut cipher = rc4::Rc4::new(&key_bytes);
    // cipher.apply_keystream(&mut ciphertext_bytes);
    //
    // String::from_utf8(ciphertext_bytes)
    //     .map_err(|e| anyhow!("Invalid UTF-8 in decrypted text: {}", e))
}

// Key generation functions
pub fn generate_aes_key() -> Result<String> {
    let mut key = [0u8; 16];
    thread_rng().fill_bytes(&mut key);
    Ok(hex::encode(key))
}

pub fn generate_des_key() -> Result<String> {
    let mut key = [0u8; 8];
    thread_rng().fill_bytes(&mut key);
    Ok(hex::encode(key))
}

pub fn generate_triple_des_key() -> Result<String> {
    let mut key = [0u8; 24];
    thread_rng().fill_bytes(&mut key);
    Ok(hex::encode(key))
}

pub fn generate_rc4_key() -> Result<String> {
    let mut key = [0u8; 16]; // 128-bit key
    thread_rng().fill_bytes(&mut key);
    Ok(hex::encode(key))
}

pub fn generate_aes_iv() -> Result<String> {
    let mut iv = [0u8; 16];
    thread_rng().fill_bytes(&mut iv);
    Ok(hex::encode(iv))
}

pub fn generate_des_iv() -> Result<String> {
    let mut iv = [0u8; 8];
    thread_rng().fill_bytes(&mut iv);
    Ok(hex::encode(iv))
}
