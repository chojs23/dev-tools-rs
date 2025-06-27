use anyhow::{anyhow, Result};
use cbc::{Decryptor, Encryptor};
use des::TdesEde3;
use ecb::{Decryptor as EcbDecryptor, Encryptor as EcbEncryptor};

use crate::core::crypto::CipherMode;

type TdesEde3CbcEnc = Encryptor<TdesEde3>;
type TdesEde3CbcDec = Decryptor<TdesEde3>;
type TdesEde3EcbEnc = EcbEncryptor<TdesEde3>;
type TdesEde3EcbDec = EcbDecryptor<TdesEde3>;

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
