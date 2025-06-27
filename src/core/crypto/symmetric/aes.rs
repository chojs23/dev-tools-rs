use aes::{Aes128, Aes192, Aes256};
use anyhow::{anyhow, Result};
use cbc::{Decryptor, Encryptor};
use cipher::block_padding::Pkcs7;
use cipher::{BlockDecryptMut, BlockEncryptMut, KeyInit, KeyIvInit};
use ecb::{Decryptor as EcbDecryptor, Encryptor as EcbEncryptor};
use hex;
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
type Aes192EcbEnc = EcbEncryptor<Aes192>;
type Aes192EcbDec = EcbDecryptor<Aes192>;
type Aes256EcbEnc = EcbEncryptor<Aes256>;
type Aes256EcbDec = EcbDecryptor<Aes256>;

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

fn encrypt_aes_192_cbc(plaintext: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>> {
    let mut buf = vec![0u8; plaintext.len() + 16];

    let ct = Aes192CbcEnc::new(key.into(), iv.into())
        .encrypt_padded_b2b_mut::<Pkcs7>(plaintext, &mut buf)
        .unwrap();

    Ok(ct.to_vec())
}

fn encrypt_aes_256_cbc(plaintext: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>> {
    let mut buf = vec![0u8; plaintext.len() + 16];

    let ct = Aes256CbcEnc::new(key.into(), iv.into())
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

fn encrypt_aes_192_ecb(plaintext: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    let mut buf = vec![0u8; plaintext.len() + 16];

    let ct = Aes192EcbEnc::new(key.into())
        .encrypt_padded_b2b_mut::<Pkcs7>(plaintext, &mut buf)
        .unwrap();

    Ok(ct.to_vec())
}

fn encrypt_aes_256_ecb(plaintext: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    let mut buf = vec![0u8; plaintext.len() + 16];

    let ct = Aes256EcbEnc::new(key.into())
        .encrypt_padded_b2b_mut::<Pkcs7>(plaintext, &mut buf)
        .unwrap();

    Ok(ct.to_vec())
}

fn decrypt_aes_128_cbc(ciphertext: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>> {
    let mut buf = vec![0u8; ciphertext.len()];

    let pt = Aes128CbcDec::new(key.into(), iv.into())
        .decrypt_padded_b2b_mut::<Pkcs7>(ciphertext, &mut buf)
        .unwrap();

    Ok(pt.to_vec())
}

fn decrypt_aes_192_cbc(ciphertext: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>> {
    let mut buf = vec![0u8; ciphertext.len()];

    let pt = Aes192CbcDec::new(key.into(), iv.into())
        .decrypt_padded_b2b_mut::<Pkcs7>(ciphertext, &mut buf)
        .unwrap();

    Ok(pt.to_vec())
}

fn decrypt_aes_256_cbc(ciphertext: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>> {
    let mut buf = vec![0u8; ciphertext.len()];

    let pt = Aes256CbcDec::new(key.into(), iv.into())
        .decrypt_padded_b2b_mut::<Pkcs7>(ciphertext, &mut buf)
        .unwrap();

    Ok(pt.to_vec())
}

fn decrypt_aes_128_ecb(ciphertext: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    let mut buf = vec![0u8; ciphertext.len()];

    let pt = Aes128EcbDec::new(key.into())
        .decrypt_padded_b2b_mut::<Pkcs7>(ciphertext, &mut buf)
        .unwrap();

    Ok(pt.to_vec())
}

fn decrypt_aes_192_ecb(ciphertext: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    let mut buf = vec![0u8; ciphertext.len()];

    let pt = Aes192EcbDec::new(key.into())
        .decrypt_padded_b2b_mut::<Pkcs7>(ciphertext, &mut buf)
        .unwrap();

    Ok(pt.to_vec())
}

fn decrypt_aes_256_ecb(ciphertext: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    let mut buf = vec![0u8; ciphertext.len()];

    let pt = Aes256EcbDec::new(key.into())
        .decrypt_padded_b2b_mut::<Pkcs7>(ciphertext, &mut buf)
        .unwrap();

    Ok(pt.to_vec())
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

            let ct = match key_size {
                AesKeySize::Aes128 => encrypt_aes_128_cbc(plaintext_bytes, key_bytes, iv_bytes)?,
                AesKeySize::Aes192 => encrypt_aes_192_cbc(plaintext_bytes, key_bytes, iv_bytes)?,
                AesKeySize::Aes256 => encrypt_aes_256_cbc(plaintext_bytes, key_bytes, iv_bytes)?,
            };
            Ok(hex::encode(ct))
        }
        CipherMode::ECB => {
            let ct = match key_size {
                AesKeySize::Aes128 => encrypt_aes_128_ecb(plaintext_bytes, key_bytes)?,
                AesKeySize::Aes192 => encrypt_aes_192_ecb(plaintext_bytes, key_bytes)?,
                AesKeySize::Aes256 => encrypt_aes_256_ecb(plaintext_bytes, key_bytes)?,
            };

            Ok(hex::encode(ct))
        }
    }
}

pub fn aes_decrypt(
    ciphertext: &str,
    key: &str,
    key_size: AesKeySize,
    mode: CipherMode,
    iv: Option<&str>,
) -> Result<String> {
    validate_key_size(key, key_size)?;

    let key_bytes = key.as_bytes();
    let ciphertext_bytes =
        hex::decode(ciphertext).map_err(|_| anyhow!("Invalid hex ciphertext format"))?;

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
                &[0u8; 16]
            };

            let pt = match key_size {
                AesKeySize::Aes128 => decrypt_aes_128_cbc(&ciphertext_bytes, key_bytes, iv_bytes)?,
                AesKeySize::Aes192 => decrypt_aes_192_cbc(&ciphertext_bytes, key_bytes, iv_bytes)?,
                AesKeySize::Aes256 => decrypt_aes_256_cbc(&ciphertext_bytes, key_bytes, iv_bytes)?,
            };
            String::from_utf8(pt).map_err(|e| anyhow!("Invalid UTF-8 in decrypted text: {}", e))
        }
        CipherMode::ECB => {
            let pt = match key_size {
                AesKeySize::Aes128 => decrypt_aes_128_ecb(&ciphertext_bytes, key_bytes)?,
                AesKeySize::Aes192 => decrypt_aes_192_ecb(&ciphertext_bytes, key_bytes)?,
                AesKeySize::Aes256 => decrypt_aes_256_ecb(&ciphertext_bytes, key_bytes)?,
            };

            String::from_utf8(pt).map_err(|e| anyhow!("Invalid UTF-8 in decrypted text: {}", e))
        }
    }
}
