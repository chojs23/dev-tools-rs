use dev_tools_rs::core::crypto::{
    symmetric::{
        aes::{aes_decrypt, aes_encrypt, AesKeySize},
        des::{des_decrypt, des_encrypt},
        generate_aes_iv, generate_aes_key,
        tdes::{triple_des_decrypt, triple_des_encrypt},
    },
    CipherMode, CryptoAlgorithm, CryptoOperation, CryptographyProcessor, OutputEncoding,
};

#[cfg(test)]
mod tests {
    use super::*;
    use base64::Engine;

    // AES Tests
    #[test]
    fn test_aes_encrypt_decrypt_cbc() {
        let plaintext = "Hello, World!";
        let key = "0123456789abcdef0123456789abcdef";
        let iv = "0123456789abcdef";

        let encrypted = aes_encrypt(
            plaintext,
            key,
            AesKeySize::Aes256,
            CipherMode::CBC,
            Some(iv),
        )
        .expect("Encryption should succeed");

        let decrypted = aes_decrypt(
            &hex::encode(encrypted),
            key,
            AesKeySize::Aes256,
            CipherMode::CBC,
            Some(iv),
        )
        .expect("Decryption should succeed");

        assert_eq!(String::from_utf8(decrypted).unwrap(), plaintext);
    }

    #[test]
    fn test_aes_encrypt_decrypt_ecb() {
        let plaintext = "Hello, World!";
        let key = "0123456789abcdef0123456789abcdef"; // 32 hex chars for AES-256

        let encrypted = aes_encrypt(plaintext, key, AesKeySize::Aes256, CipherMode::ECB, None)
            .expect("Encryption should succeed");

        let decrypted = aes_decrypt(
            &hex::encode(encrypted),
            key,
            AesKeySize::Aes256,
            CipherMode::ECB,
            None,
        )
        .expect("Decryption should succeed");

        assert_eq!(String::from_utf8(decrypted).unwrap(), plaintext);
    }

    #[test]
    fn test_aes_128_encrypt_decrypt() {
        let plaintext = "Hello, World!";
        let key = "0123456789abcdef"; // 16 hex chars for AES-128
        let iv = "0123456789abcdef"; // 16 hex chars for 64-bit IV

        let encrypted = aes_encrypt(
            plaintext,
            key,
            AesKeySize::Aes128,
            CipherMode::CBC,
            Some(iv),
        )
        .expect("Encryption should succeed");

        let decrypted = aes_decrypt(
            &hex::encode(encrypted),
            key,
            AesKeySize::Aes128,
            CipherMode::CBC,
            Some(iv),
        )
        .expect("Decryption should succeed");

        assert_eq!(String::from_utf8(decrypted).unwrap(), plaintext);
    }

    #[test]
    fn test_aes_192_encrypt_decrypt() {
        let plaintext = "Hello, World!";
        let key = "051ee51c9acec69c5bf5d7a6";
        let iv = "0123456789abcdef";

        let encrypted = aes_encrypt(
            plaintext,
            key,
            AesKeySize::Aes192,
            CipherMode::CBC,
            Some(iv),
        )
        .expect("Encryption should succeed");

        let decrypted = aes_decrypt(
            &hex::encode(encrypted),
            key,
            AesKeySize::Aes192,
            CipherMode::CBC,
            Some(iv),
        )
        .expect("Decryption should succeed");

        assert_eq!(String::from_utf8(decrypted).unwrap(), plaintext);
    }

    #[test]
    fn test_aes_invalid_key_length() {
        let plaintext = "Hello, World!";
        let key = "short"; // Too short for AES-256
        let iv = "0123456789abcdef0123456789abcdef";

        let result = aes_encrypt(
            plaintext,
            key,
            AesKeySize::Aes256,
            CipherMode::CBC,
            Some(iv),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_aes_invalid_iv_length() {
        let plaintext = "Hello, World!";
        let key = "0123456789abcdef0123456789abcdef";
        let iv = "short"; // Too short for IV

        let result = aes_encrypt(
            plaintext,
            key,
            AesKeySize::Aes256,
            CipherMode::CBC,
            Some(iv),
        );
        assert!(result.is_err());
    }

    // DES Tests
    #[test]
    fn test_des_encrypt_decrypt_cbc() {
        let plaintext = "Hello, World!";
        let key = "01234567";
        let iv = "01234567";

        let encrypted = des_encrypt(plaintext, key, CipherMode::CBC, Some(iv))
            .expect("Encryption should succeed");

        let decrypted = des_decrypt(&hex::encode(encrypted), key, CipherMode::CBC, Some(iv))
            .expect("Decryption should succeed");

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_des_encrypt_decrypt_ecb() {
        let plaintext = "Hello, World!";
        let key = "01234567";

        let encrypted =
            des_encrypt(plaintext, key, CipherMode::ECB, None).expect("Encryption should succeed");

        let decrypted = des_decrypt(&hex::encode(encrypted), key, CipherMode::ECB, None)
            .expect("Decryption should succeed");

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_des_invalid_key_length() {
        let plaintext = "Hello, World!";
        let key = "short"; // Too short for DES

        let result = des_encrypt(plaintext, key, CipherMode::ECB, None);
        assert!(result.is_err());
    }

    // Triple DES Tests
    #[test]
    fn test_triple_des_encrypt_decrypt_cbc() {
        let plaintext = "Hello, World!";
        let key = "382614643025737a9cceeb79";
        let iv = "060ded27";

        let encrypted = triple_des_encrypt(plaintext, key, CipherMode::CBC, Some(iv))
            .expect("Encryption should succeed");

        let decrypted = triple_des_decrypt(&hex::encode(encrypted), key, CipherMode::CBC, Some(iv))
            .expect("Decryption should succeed");

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_triple_des_encrypt_decrypt_ecb() {
        let plaintext = "Hello, World!";
        let key = "382614643025737a9cceeb79";

        let encrypted = triple_des_encrypt(plaintext, key, CipherMode::ECB, None)
            .expect("Encryption should succeed");

        let decrypted = triple_des_decrypt(&hex::encode(encrypted), key, CipherMode::ECB, None)
            .expect("Decryption should succeed");

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_triple_des_invalid_key_length() {
        let plaintext = "Hello, World!";
        let key = "short"; // Too short for 3DES

        let result = triple_des_encrypt(plaintext, key, CipherMode::ECB, None);
        assert!(result.is_err());
    }

    // Key and IV Generation Tests
    #[test]
    fn test_generate_aes_key_128() {
        let key = generate_aes_key(AesKeySize::Aes128);
        assert_eq!(key.len(), 16); // 128 bits = 16 hex chars
        assert!(key.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_generate_aes_key_192() {
        let key = generate_aes_key(AesKeySize::Aes192);
        assert_eq!(key.len(), 24); // 192 bits = 24 hex chars
        assert!(key.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_generate_aes_key_256() {
        let key = generate_aes_key(AesKeySize::Aes256);
        assert_eq!(key.len(), 32); // 256 bits = 32 hex chars
        assert!(key.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_generate_aes_iv() {
        let iv = generate_aes_iv();
        assert_eq!(iv.len(), 16); // 128 bits = 16 hex chars
        assert!(iv.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_generate_different_keys() {
        let key1 = generate_aes_key(AesKeySize::Aes256);
        let key2 = generate_aes_key(AesKeySize::Aes256);
        assert_ne!(key1, key2); // Generated keys should be different
    }

    #[test]
    fn test_generate_different_ivs() {
        let iv1 = generate_aes_iv();
        let iv2 = generate_aes_iv();
        assert_ne!(iv1, iv2); // Generated IVs should be different
    }

    // Integration Tests with CryptographyProcessor
    #[test]
    fn test_crypto_processor_aes_encrypt_hex() {
        let mut processor = CryptographyProcessor::new();
        processor.input.algorithm = CryptoAlgorithm::AES;
        processor.input.operation = CryptoOperation::Encrypt;
        processor.input.mode = Some(CipherMode::CBC);
        processor.input.aes_key_size = Some(AesKeySize::Aes256);
        processor.input.input_text = "Hello, World!".to_string();
        processor.input.key = "0123456789abcdef0123456789abcdef".to_string();
        processor.input.iv = Some("0123456789abcdef".to_string());
        processor.input.encoding = OutputEncoding::Hex;

        let result = processor.process();
        assert!(result.is_ok());
        let encrypted = result.unwrap();
        assert!(!encrypted.is_empty());
        assert!(encrypted.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_crypto_processor_aes_encrypt_base64() {
        let mut processor = CryptographyProcessor::new();
        processor.input.algorithm = CryptoAlgorithm::AES;
        processor.input.operation = CryptoOperation::Encrypt;
        processor.input.mode = Some(CipherMode::CBC);
        processor.input.aes_key_size = Some(AesKeySize::Aes256);
        processor.input.input_text = "Hello, World!".to_string();
        processor.input.key = "0123456789abcdef0123456789abcdef".to_string();
        processor.input.iv = Some("0123456789abcdef".to_string());
        processor.input.encoding = OutputEncoding::Base64;

        let result = processor.process();
        assert!(result.is_ok());
        let encrypted = result.unwrap();
        assert!(!encrypted.is_empty());
        // Base64 encoded string should be decodable
        assert!(base64::engine::general_purpose::STANDARD
            .decode(&encrypted)
            .is_ok())
    }

    #[test]
    fn test_crypto_processor_des_encrypt_decrypt() {
        let mut processor = CryptographyProcessor::new();
        processor.input.algorithm = CryptoAlgorithm::DES;
        processor.input.operation = CryptoOperation::Encrypt;
        processor.input.mode = Some(CipherMode::CBC);
        processor.input.input_text = "Hello, World!".to_string();
        processor.input.key = "20c5f825".to_string();
        processor.input.iv = Some("6a5da673".to_string());
        processor.input.encoding = OutputEncoding::Hex;

        let encrypt_result = processor.process();
        assert!(encrypt_result.is_ok());
        let encrypted = encrypt_result.unwrap();
        assert!(!encrypted.is_empty());

        // Now decrypt
        processor.input.operation = CryptoOperation::Decrypt;
        processor.input.input_text = encrypted;

        let decrypt_result = processor.process();
        assert!(decrypt_result.is_ok());
        let decrypted = decrypt_result.unwrap();
        assert_eq!(decrypted, "Hello, World!");
    }

    #[test]
    fn test_crypto_processor_triple_des_encrypt_decrypt() {
        let mut processor = CryptographyProcessor::new();
        processor.input.algorithm = CryptoAlgorithm::TripleDES;
        processor.input.operation = CryptoOperation::Encrypt;
        processor.input.mode = Some(CipherMode::CBC);
        processor.input.input_text = "Hello, World!".to_string();
        processor.input.key = "a7342af1e13ae62f849c3e4b".to_string();
        processor.input.iv = Some("01234567".to_string());
        processor.input.encoding = OutputEncoding::Hex;

        let encrypt_result = processor.process();
        assert!(encrypt_result.is_ok());
        let encrypted = encrypt_result.unwrap();
        assert!(!encrypted.is_empty());

        // Now decrypt
        processor.input.operation = CryptoOperation::Decrypt;
        processor.input.input_text = encrypted;

        let decrypt_result = processor.process();
        assert!(decrypt_result.is_ok());
        let decrypted = decrypt_result.unwrap();
        assert_eq!(decrypted, "Hello, World!");
    }

    #[test]
    fn test_crypto_processor_invalid_operation() {
        let mut processor = CryptographyProcessor::new();
        processor.input.algorithm = CryptoAlgorithm::AES;
        processor.input.operation = CryptoOperation::Sign; // Invalid for AES
        processor.input.input_text = "Hello, World!".to_string();
        processor.input.key = "0123456789abcdef0123456789abcdef".to_string();

        let result = processor.process();
        assert!(result.is_err());
        assert!(processor.error.is_some());
    }
}
