pub mod aes;
pub mod des;
pub mod tdes;

use aes::AesKeySize;
use rand::{thread_rng, RngCore};

// Key generation functions
pub fn generate_aes_key(key_size: AesKeySize) -> String {
    let key_length = match key_size {
        AesKeySize::Aes128 => 16,
        AesKeySize::Aes192 => 24,
        AesKeySize::Aes256 => 32,
    };
    let mut key = vec![0u8; key_length / 2];

    thread_rng().fill_bytes(&mut key);

    key.into_iter()
        .map(|b| format!("{:02x}", b)) // Convert to hex format
        .collect::<String>()
}

pub fn generate_rc4_key() -> String {
    let mut key = [0u8; 16 / 2]; // 128-bit key
    thread_rng().fill_bytes(&mut key);
    key.into_iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>()
}

pub fn generate_aes_iv() -> String {
    let mut iv = [0u8; 16 / 2];
    thread_rng().fill_bytes(&mut iv);
    iv.into_iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>()
}
