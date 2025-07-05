use anyhow::{anyhow, Result};
use base64::{engine::general_purpose, Engine as _};
use cbc::{Decryptor, Encryptor};
use cipher::block_padding::Pkcs7;
use cipher::{BlockDecryptMut, BlockEncryptMut, KeyInit, KeyIvInit};
use des::TdesEde3;
use ecb::{Decryptor as EcbDecryptor, Encryptor as EcbEncryptor};
use hex;

use crate::core::crypto::CipherMode;

type TdesEde3CbcEnc = Encryptor<TdesEde3>;
type TdesEde3CbcDec = Decryptor<TdesEde3>;
type TdesEde3EcbEnc = EcbEncryptor<TdesEde3>;
type TdesEde3EcbDec = EcbDecryptor<TdesEde3>;

fn validate_triple_des_key(key: &str) -> Result<()> {
    if key.len() != 24 {
        return Err(anyhow!(
            "Triple DES requires a 24-byte key, got {} characters",
            key.len()
        ));
    }
    Ok(())
}

fn encrypt_triple_des_cbc(plaintext: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>> {
    let mut buf = vec![0u8; plaintext.len() + 8]; // 8 bytes for DES block size padding

    let ct = TdesEde3CbcEnc::new(key.into(), iv.into())
        .encrypt_padded_b2b_mut::<Pkcs7>(plaintext, &mut buf)
        .map_err(|e| anyhow!("Triple DES CBC encryption failed: {}", e))?;

    Ok(ct.to_vec())
}

fn encrypt_triple_des_ecb(plaintext: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    let mut buf = vec![0u8; plaintext.len() + 8];

    let ct = TdesEde3EcbEnc::new(key.into())
        .encrypt_padded_b2b_mut::<Pkcs7>(plaintext, &mut buf)
        .map_err(|e| anyhow!("Triple DES ECB encryption failed: {}", e))?;

    Ok(ct.to_vec())
}

fn decrypt_triple_des_cbc(ciphertext: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>> {
    let mut buf = vec![0u8; ciphertext.len()];

    let pt = TdesEde3CbcDec::new(key.into(), iv.into())
        .decrypt_padded_b2b_mut::<Pkcs7>(ciphertext, &mut buf)
        .map_err(|e| anyhow!("Triple DES CBC decryption failed: {}", e))?;

    Ok(pt.to_vec())
}

fn decrypt_triple_des_ecb(ciphertext: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    let mut buf = vec![0u8; ciphertext.len()];

    let pt = TdesEde3EcbDec::new(key.into())
        .decrypt_padded_b2b_mut::<Pkcs7>(ciphertext, &mut buf)
        .map_err(|e| anyhow!("Triple DES ECB decryption failed: {}", e))?;

    Ok(pt.to_vec())
}

// Triple DES Functions
pub fn triple_des_encrypt(
    plaintext: &str,
    key: &str,
    mode: CipherMode,
    iv: Option<&str>,
) -> Result<Vec<u8>> {
    validate_triple_des_key(key)?;

    let key_bytes = key.as_bytes();
    let plaintext_bytes = plaintext.as_bytes();

    match mode {
        CipherMode::CBC => {
            let iv_bytes: &[u8; 8] = if let Some(iv) = iv {
                if iv.len() != 8 {
                    return Err(anyhow!(
                        "Triple DES CBC mode requires an 8-byte IV, got {} characters",
                        iv.len()
                    ));
                }
                iv.as_bytes()
                    .try_into()
                    .map_err(|_| anyhow!("IV must be exactly 8 bytes long"))?
            } else {
                &[0u8; 8] // Default IV of zeros if not provided
            };

            let ct = encrypt_triple_des_cbc(plaintext_bytes, key_bytes, iv_bytes)?;
            Ok(ct)
        }
        CipherMode::ECB => {
            let ct = encrypt_triple_des_ecb(plaintext_bytes, key_bytes)?;
            Ok(ct)
        }
    }
}

pub fn triple_des_decrypt(
    ciphertext: &str,
    key: &str,
    mode: CipherMode,
    iv: Option<&str>,
) -> Result<String> {
    validate_triple_des_key(key)?;

    let key_bytes = key.as_bytes();

    // Try to decode as hex first, then base64
    let ciphertext_bytes = hex::decode(ciphertext)
        .or_else(|_| general_purpose::STANDARD.decode(ciphertext))
        .map_err(|_| anyhow!("Invalid ciphertext format - must be valid hex or base64"))?;

    match mode {
        CipherMode::CBC => {
            let iv_bytes: &[u8; 8] = if let Some(iv) = iv {
                if iv.len() != 8 {
                    return Err(anyhow!(
                        "Triple DES CBC mode requires an 8-byte (16 hex characters) IV, got {} characters",
                        iv.len()
                    ));
                }
                iv.as_bytes()
                    .try_into()
                    .map_err(|_| anyhow!("IV must be exactly 8 bytes long"))?
            } else {
                &[0u8; 8]
            };

            let pt = decrypt_triple_des_cbc(&ciphertext_bytes, key_bytes, iv_bytes)?;
            String::from_utf8(pt).map_err(|e| anyhow!("Invalid UTF-8 in decrypted text: {}", e))
        }
        CipherMode::ECB => {
            let pt = decrypt_triple_des_ecb(&ciphertext_bytes, key_bytes)?;
            String::from_utf8(pt).map_err(|e| anyhow!("Invalid UTF-8 in decrypted text: {}", e))
        }
    }
}
