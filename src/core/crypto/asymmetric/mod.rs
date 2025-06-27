use rand::{thread_rng, RngCore};

pub mod ecdsa;
pub mod rsa;

pub fn generate_des_key() -> String {
    let mut key = [0u8; 8 / 2];
    thread_rng().fill_bytes(&mut key);

    key.into_iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>()
}

pub fn generate_triple_des_key() -> String {
    let mut key = [0u8; 24 / 2];
    thread_rng().fill_bytes(&mut key);

    key.into_iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>()
}
pub fn generate_des_iv() -> String {
    let mut iv = [0u8; 8 / 2];
    thread_rng().fill_bytes(&mut iv);

    iv.into_iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>()
}
