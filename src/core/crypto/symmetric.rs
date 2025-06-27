use aes::Aes128;
use anyhow::{anyhow, Result};
use cbc::{Decryptor, Encryptor};
use cipher::block_padding::Pkcs7;
use cipher::{BlockDecryptMut, BlockEncryptMut, KeyInit, KeyIvInit, StreamCipher};
use des::Des;
use des::TdesEde3;
use ecb::{Decryptor as EcbDecryptor, Encryptor as EcbEncryptor};
use hex;
use rand::{thread_rng, RngCore};

use super::CipherMode;

type Aes128CbcEnc = Encryptor<Aes128>;
type Aes128CbcDec = Decryptor<Aes128>;
type Aes128EcbEnc = EcbEncryptor<Aes128>;
type Aes128EcbDec = EcbDecryptor<Aes128>;
type DesCbcEnc = Encryptor<Des>;
type DesCbcDec = Decryptor<Des>;
type DesEcbEnc = EcbEncryptor<Des>;
type DesEcbDec = EcbDecryptor<Des>;
type TdesEde3CbcEnc = Encryptor<TdesEde3>;
type TdesEde3CbcDec = Decryptor<TdesEde3>;
type TdesEde3EcbEnc = EcbEncryptor<TdesEde3>;
type TdesEde3EcbDec = EcbDecryptor<TdesEde3>;

// AES Functions
pub fn aes_encrypt(
    plaintext: &str,
    key: &str,
    mode: CipherMode,
    iv: Option<&str>,
) -> Result<String> {
    let key_bytes = hex::decode(key).map_err(|_| anyhow!("Invalid hex key format"))?;

    if key_bytes.len() != 16 {
        return Err(anyhow!(
            "AES-128 requires a 16-byte (32 hex characters) key"
        ));
    }

    let key_bytes: [u8; 16] = key_bytes
        .try_into()
        .map_err(|_| anyhow!("Key must be exactly 16 bytes long"))?;

    // make key_bytes a fixed slice

    let plaintext_bytes = plaintext.as_bytes();

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
            let pt_len = plaintext.len();
            buf[..pt_len].copy_from_slice(plaintext_bytes);
            let ct = Aes128CbcEnc::new(&key_bytes.into(), &iv_bytes.into())
                .encrypt_padded_mut::<Pkcs7>(&mut buf, pt_len)
                .unwrap();

            Ok(hex::encode(&ct[..pt_len]))
        }
        CipherMode::ECB => {
            let mut buf = [0u8; 48];
            let pt_len = plaintext.len();
            buf[..pt_len].copy_from_slice(plaintext_bytes);
            let ct = Aes128EcbEnc::new(&key_bytes.into())
                .encrypt_padded_mut::<Pkcs7>(&mut buf, pt_len)
                .unwrap();

            Ok(hex::encode(&ct[..pt_len]))
        }
    }
}

pub fn aes_decrypt(
    ciphertext: &str,
    key: &str,
    mode: CipherMode,
    iv: Option<&str>,
) -> Result<String> {
    let key_bytes = hex::decode(key).map_err(|_| anyhow!("Invalid hex key format"))?;

    if key_bytes.len() != 16 {
        return Err(anyhow!(
            "AES-128 requires a 16-byte (32 hex characters) key"
        ));
    }

    let key_bytes: [u8; 16] = key_bytes
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

// DES Functions
pub fn des_encrypt(
    plaintext: &str,
    key: &str,
    mode: CipherMode,
    iv: Option<&str>,
) -> Result<String> {
    return Err(anyhow!("DES encryption is not implemented yet"));
    // let key_bytes = hex::decode(key).map_err(|_| anyhow!("Invalid hex key format"))?;
    //
    // if key_bytes.len() != 8 {
    //     return Err(anyhow!("DES requires an 8-byte (16 hex characters) key"));
    // }
    //
    // let plaintext_bytes = plaintext.as_bytes();
    //
    // match mode {
    //     CipherMode::CBC => {
    //         let iv_str = iv.ok_or_else(|| anyhow!("IV required for CBC mode"))?;
    //         let iv_bytes = hex::decode(iv_str).map_err(|_| anyhow!("Invalid hex IV format"))?;
    //
    //         if iv_bytes.len() != 8 {
    //             return Err(anyhow!("DES requires an 8-byte (16 hex characters) IV"));
    //         }
    //
    //         let cipher = DesCbc::new_from_slices(&key_bytes, &iv_bytes)
    //             .map_err(|e| anyhow!("Failed to create cipher: {}", e))?;
    //
    //         let ciphertext = cipher.encrypt_vec(plaintext_bytes);
    //         Ok(hex::encode(ciphertext))
    //     }
    //     CipherMode::ECB => {
    //         let cipher = DesEcb::new_from_slice(&key_bytes)
    //             .map_err(|e| anyhow!("Failed to create cipher: {}", e))?;
    //
    //         let ciphertext =
    //             cipher.encrypt_padded_vec::<cipher::block_padding::Pkcs7>(plaintext_bytes);
    //         Ok(hex::encode(ciphertext))
    //     }
    // }
}

pub fn des_decrypt(
    ciphertext: &str,
    key: &str,
    mode: CipherMode,
    iv: Option<&str>,
) -> Result<String> {
    return Err(anyhow!("DES decryption is not implemented yet"));
    // let key_bytes = hex::decode(key).map_err(|_| anyhow!("Invalid hex key format"))?;
    //
    // if key_bytes.len() != 8 {
    //     return Err(anyhow!("DES requires an 8-byte (16 hex characters) key"));
    // }
    //
    // let ciphertext_bytes =
    //     hex::decode(ciphertext).map_err(|_| anyhow!("Invalid hex ciphertext format"))?;
    //
    // match mode {
    //     CipherMode::CBC => {
    //         let iv_str = iv.ok_or_else(|| anyhow!("IV required for CBC mode"))?;
    //         let iv_bytes = hex::decode(iv_str).map_err(|_| anyhow!("Invalid hex IV format"))?;
    //
    //         if iv_bytes.len() != 8 {
    //             return Err(anyhow!("DES requires an 8-byte (16 hex characters) IV"));
    //         }
    //
    //         let cipher = DesCbcEnc::new_from_slices(&key_bytes, &iv_bytes)
    //             .map_err(|e| anyhow!("Failed to create cipher: {}", e))?;
    //
    //         let plaintext = cipher
    //             .decrypt_vec(&ciphertext_bytes)
    //             .map_err(|e| anyhow!("Decryption failed: {}", e))?;
    //
    //         String::from_utf8(plaintext)
    //             .map_err(|e| anyhow!("Invalid UTF-8 in decrypted text: {}", e))
    //     }
    //     CipherMode::ECB => {
    //         let cipher = DesEcb::new_from_slice(&key_bytes)
    //             .map_err(|e| anyhow!("Failed to create cipher: {}", e))?;
    //
    //         let plaintext = cipher
    //             .decrypt_padded_vec::<cipher::block_padding::Pkcs7>(&ciphertext_bytes)
    //             .map_err(|e| anyhow!("Decryption failed: {}", e))?;
    //
    //         String::from_utf8(plaintext)
    //             .map_err(|e| anyhow!("Invalid UTF-8 in decrypted text: {}", e))
    //     }
    // }
}

// Triple DES Functions
pub fn triple_des_encrypt(
    plaintext: &str,
    key: &str,
    mode: CipherMode,
    iv: Option<&str>,
) -> Result<String> {
    return Err(anyhow!("Triple DES encryption is not implemented yet"));
    // let key_bytes = hex::decode(key).map_err(|_| anyhow!("Invalid hex key format"))?;
    //
    // if key_bytes.len() != 24 {
    //     return Err(anyhow!(
    //         "Triple DES requires a 24-byte (48 hex characters) key"
    //     ));
    // }
    //
    // let plaintext_bytes = plaintext.as_bytes();
    //
    // match mode {
    //     CipherMode::CBC => {
    //         let iv_str = iv.ok_or_else(|| anyhow!("IV required for CBC mode"))?;
    //         let iv_bytes = hex::decode(iv_str).map_err(|_| anyhow!("Invalid hex IV format"))?;
    //
    //         if iv_bytes.len() != 8 {
    //             return Err(anyhow!(
    //                 "Triple DES requires an 8-byte (16 hex characters) IV"
    //             ));
    //         }
    //
    //         let cipher = TdesEde3Cbc::new_from_slices(&key_bytes, &iv_bytes)
    //             .map_err(|e| anyhow!("Failed to create cipher: {}", e))?;
    //
    //         let ciphertext = cipher.encrypt_vec(plaintext_bytes);
    //         Ok(hex::encode(ciphertext))
    //     }
    //     CipherMode::ECB => {
    //         let cipher = TdesEde3Ecb::new_from_slice(&key_bytes)
    //             .map_err(|e| anyhow!("Failed to create cipher: {}", e))?;
    //
    //         let ciphertext =
    //             cipher.encrypt_padded_vec::<cipher::block_padding::Pkcs7>(plaintext_bytes);
    //         Ok(hex::encode(ciphertext))
    //     }
    // }
}

pub fn triple_des_decrypt(
    ciphertext: &str,
    key: &str,
    mode: CipherMode,
    iv: Option<&str>,
) -> Result<String> {
    return Err(anyhow!("Triple DES decryption is not implemented yet"));
    // let key_bytes = hex::decode(key).map_err(|_| anyhow!("Invalid hex key format"))?;
    //
    // if key_bytes.len() != 24 {
    //     return Err(anyhow!(
    //         "Triple DES requires a 24-byte (48 hex characters) key"
    //     ));
    // }
    //
    // let ciphertext_bytes =
    //     hex::decode(ciphertext).map_err(|_| anyhow!("Invalid hex ciphertext format"))?;
    //
    // match mode {
    //     CipherMode::CBC => {
    //         let iv_str = iv.ok_or_else(|| anyhow!("IV required for CBC mode"))?;
    //         let iv_bytes = hex::decode(iv_str).map_err(|_| anyhow!("Invalid hex IV format"))?;
    //
    //         if iv_bytes.len() != 8 {
    //             return Err(anyhow!(
    //                 "Triple DES requires an 8-byte (16 hex characters) IV"
    //             ));
    //         }
    //
    //         let cipher = TdesEde3Cbc::new_from_slices(&key_bytes, &iv_bytes)
    //             .map_err(|e| anyhow!("Failed to create cipher: {}", e))?;
    //
    //         let plaintext = cipher
    //             .decrypt_vec(&ciphertext_bytes)
    //             .map_err(|e| anyhow!("Decryption failed: {}", e))?;
    //
    //         String::from_utf8(plaintext)
    //             .map_err(|e| anyhow!("Invalid UTF-8 in decrypted text: {}", e))
    //     }
    //     CipherMode::ECB => {
    //         let cipher = TdesEde3Ecb::new_from_slice(&key_bytes)
    //             .map_err(|e| anyhow!("Failed to create cipher: {}", e))?;
    //
    //         let plaintext = cipher
    //             .decrypt_padded_vec::<cipher::block_padding::Pkcs7>(&ciphertext_bytes)
    //             .map_err(|e| anyhow!("Decryption failed: {}", e))?;
    //
    //         String::from_utf8(plaintext)
    //             .map_err(|e| anyhow!("Invalid UTF-8 in decrypted text: {}", e))
    //     }
    // }
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
