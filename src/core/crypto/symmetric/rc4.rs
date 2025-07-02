/// Disabled due to security concerns with RC4.
use anyhow::{anyhow, Result};
use base64::{engine::general_purpose, Engine as _};
use hex;
use rc4::{consts::U256, KeyInit, StreamCipher};

fn validate_rc4_key(key: &str) -> Result<()> {
    if key.is_empty() || key.len() > 256 {
        return Err(anyhow!(
            "RC4 key must be between 1 and 256 bytes long, got {} bytes",
            key.len()
        ));
    }
    Ok(())
}

fn rc4_process(data: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    let mut cipher: rc4::Rc4<U256> =
        rc4::Rc4::new_from_slice(key).map_err(|e| anyhow!("Invalid RC4 key: {}", e))?;
    let mut result = data.to_vec();
    cipher.apply_keystream(&mut result);
    Ok(result)
}

// RC4 Functions
fn rc4_encrypt(plaintext: &str, key: &str) -> Result<String> {
    validate_rc4_key(key)?;

    let key_bytes = key.as_bytes();
    let plaintext_bytes = plaintext.as_bytes();

    let ciphertext = rc4_process(plaintext_bytes, key_bytes)?;
    Ok(hex::encode(ciphertext))
}

pub fn rc4_decrypt(ciphertext: &str, key: &str) -> Result<String> {
    validate_rc4_key(key)?;

    let key_bytes = key.as_bytes();

    // Try to decode as hex first, then base64
    let ciphertext_bytes = hex::decode(ciphertext)
        .or_else(|_| general_purpose::STANDARD.decode(ciphertext))
        .map_err(|_| anyhow!("Invalid ciphertext format - must be valid hex or base64"))?;

    let plaintext_bytes = rc4_process(&ciphertext_bytes, key_bytes)?;
    String::from_utf8(plaintext_bytes)
        .map_err(|e| anyhow!("Invalid UTF-8 in decrypted text: {}", e))
}
