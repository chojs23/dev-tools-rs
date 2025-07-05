use dev_tools_rs::core::crypto::{
    asymmetric::{
        ecdsa::{ecdsa_sign, ecdsa_verify},
        rsa::{generate_rsa_keypair, rsa_decrypt, rsa_encrypt, rsa_sign, rsa_verify},
    },
    symmetric::{
        aes::{aes_decrypt, aes_encrypt, AesKeySize},
        des::{des_decrypt, des_encrypt},
        tdes::{triple_des_decrypt, triple_des_encrypt},
    },
    CipherMode, CryptoAlgorithm, CryptoOperation, CryptographyProcessor,
};

#[cfg(test)]
mod tests {
    use super::*;

    // AES Error Handling Tests
    #[test]
    fn test_aes_encrypt_invalid_key_length() {
        let plaintext = "Hello, World!";
        let invalid_key = "short"; // Too short for any AES key size
        let iv = "0123456789abcdef0123456789abcdef";

        let result = aes_encrypt(
            plaintext,
            invalid_key,
            AesKeySize::Aes128,
            CipherMode::CBC,
            Some(iv),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_aes_encrypt_invalid_key_hex() {
        let plaintext = "Hello, World!";
        let invalid_key = "ghijklmnopqrstuv"; // Invalid hex characters
        let iv = "0123456789abcdef0123456789abcdef";

        let result = aes_encrypt(
            plaintext,
            invalid_key,
            AesKeySize::Aes128,
            CipherMode::CBC,
            Some(iv),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_aes_encrypt_invalid_iv_length() {
        let plaintext = "Hello, World!";
        let key = "0123456789abcdef0123456789abcdef";
        let invalid_iv = "short"; // Too short for IV

        let result = aes_encrypt(
            plaintext,
            key,
            AesKeySize::Aes256,
            CipherMode::CBC,
            Some(invalid_iv),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_aes_encrypt_invalid_iv_hex() {
        let plaintext = "Hello, World!";
        let key = "0123456789abcdef0123456789abcdef";
        let invalid_iv = "ghijklmnopqrstuv123"; // Invalid hex characters

        let result = aes_encrypt(
            plaintext,
            key,
            AesKeySize::Aes256,
            CipherMode::CBC,
            Some(invalid_iv),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_aes_decrypt_invalid_ciphertext() {
        let invalid_ciphertext = "invalid_hex"; // Invalid hex
        let key = "0123456789abcdef0123456789abcdef";
        let iv = "0123456789abcdef0123456789abcdef";

        let result = aes_decrypt(
            invalid_ciphertext,
            key,
            AesKeySize::Aes256,
            CipherMode::CBC,
            Some(iv),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_aes_decrypt_invalid_iv() {
        let plaintext = "Hello, World!";
        let key = "0123456789abcdef0123456789abcdef";
        let iv = "0123456789abcdef";
        let invalid_iv = "fedcba9876543210fedcba9876543210";

        // Encrypt with correct IV
        let encrypted = aes_encrypt(
            plaintext,
            key,
            AesKeySize::Aes256,
            CipherMode::CBC,
            Some(iv),
        )
        .unwrap();

        // Try to decrypt with invalid IV
        let result = aes_decrypt(
            &hex::encode(encrypted),
            key,
            AesKeySize::Aes256,
            CipherMode::CBC,
            Some(invalid_iv),
        );
        assert!(result.is_err());
    }

    // DES Error Handling Tests
    #[test]
    fn test_des_encrypt_invalid_key_length() {
        let plaintext = "Hello, World!";
        let invalid_key = "short"; // Too short for DES

        let result = des_encrypt(plaintext, invalid_key, CipherMode::ECB, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_des_encrypt_invalid_key_hex() {
        let plaintext = "Hello, World!";
        let invalid_key = "ghijklmnopqrstuv"; // Invalid hex characters

        let result = des_encrypt(plaintext, invalid_key, CipherMode::ECB, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_des_decrypt_invalid_ciphertext() {
        let invalid_ciphertext = "invalid_hex"; // Invalid hex
        let key = "0123456789abcdef";

        let result = des_decrypt(invalid_ciphertext, key, CipherMode::ECB, None);
        assert!(result.is_err());
    }

    // Triple DES Error Handling Tests
    #[test]
    fn test_triple_des_encrypt_invalid_key_length() {
        let plaintext = "Hello, World!";
        let invalid_key = "short"; // Too short for Triple DES

        let result = triple_des_encrypt(plaintext, invalid_key, CipherMode::ECB, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_triple_des_encrypt_invalid_key_hex() {
        let plaintext = "Hello, World!";
        let invalid_key = "ghijklmnopqrstuvwxyz123456789012345678901234567890"; // Invalid hex characters

        let result = triple_des_encrypt(plaintext, invalid_key, CipherMode::ECB, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_triple_des_decrypt_invalid_ciphertext() {
        let invalid_ciphertext = "invalid_hex"; // Invalid hex
        let key = "0123456789abcdef0123456789abcdef0123456789abcdef";

        let result = triple_des_decrypt(invalid_ciphertext, key, CipherMode::ECB, None);
        assert!(result.is_err());
    }

    // RSA Error Handling Tests
    #[test]
    fn test_rsa_encrypt_invalid_public_key() {
        let plaintext = "Hello, World!";
        let invalid_key = "invalid_key";

        let result = rsa_encrypt(plaintext, invalid_key);
        assert!(result.is_err());
    }

    #[test]
    fn test_rsa_decrypt_invalid_private_key() {
        let ciphertext = "encrypted_text";
        let invalid_key = "invalid_key";

        let result = rsa_decrypt(ciphertext, invalid_key);
        assert!(result.is_err());
    }

    #[test]
    fn test_rsa_decrypt_invalid_ciphertext() {
        let (_, private_key) = generate_rsa_keypair(2048).unwrap();
        let invalid_ciphertext = "invalid_ciphertext";

        let result = rsa_decrypt(invalid_ciphertext, &private_key);
        assert!(result.is_err());
    }

    #[test]
    fn test_rsa_sign_invalid_private_key() {
        let message = "Hello, World!";
        let invalid_key = "invalid_key";

        let result = rsa_sign(message, invalid_key);
        assert!(result.is_err());
    }

    #[test]
    fn test_rsa_verify_invalid_public_key() {
        let message = "Hello, World!";
        let signature = "signature";
        let invalid_key = "invalid_key";

        let result = rsa_verify(message, signature, invalid_key);
        assert!(result.is_err());
    }

    #[test]
    fn test_rsa_verify_invalid_signature() {
        let (public_key, _) = generate_rsa_keypair(2048).unwrap();
        let message = "Hello, World!";
        let invalid_signature = "invalid_signature";

        let result = rsa_verify(message, invalid_signature, &public_key);
        assert!(result.is_err());
    }

    #[test]
    fn test_rsa_decrypt_with_public_key() {
        let (public_key, _private_key) = generate_rsa_keypair(2048).unwrap();
        let plaintext = "Hello, World!";

        // Encrypt with public key
        let encrypted = rsa_encrypt(plaintext, &public_key).unwrap();

        // Try to decrypt with public key (should fail)
        let result = rsa_decrypt(&hex::encode(encrypted), &public_key);
        assert!(result.is_err());
    }

    #[test]
    fn test_rsa_encrypt_with_private_key() {
        let (_, private_key) = generate_rsa_keypair(2048).unwrap();
        let plaintext = "Hello, World!";

        // Try to encrypt with private key (should fail)
        let result = rsa_encrypt(plaintext, &private_key);
        assert!(result.is_err());
    }

    // ECDSA Error Handling Tests
    #[test]
    fn test_ecdsa_sign_invalid_private_key() {
        let message = "Hello, World!";
        let invalid_key = "invalid_key";

        let result = ecdsa_sign(message, invalid_key);
        assert!(result.is_err());
    }

    #[test]
    fn test_ecdsa_verify_invalid_public_key() {
        let message = "Hello, World!";
        let signature = "signature";
        let invalid_key = "invalid_key";

        let result = ecdsa_verify(message, signature, invalid_key);
        assert!(result.is_err());
    }

    #[test]
    fn test_ecdsa_verify_invalid_signature() {
        let (public_key, _) =
            dev_tools_rs::core::crypto::asymmetric::ecdsa::generate_ecdsa_keypair().unwrap();
        let message = "Hello, World!";
        let invalid_signature = "invalid_signature";

        let result = ecdsa_verify(message, invalid_signature, &public_key);
        assert!(result.is_err());
    }

    #[test]
    fn test_ecdsa_sign_invalid_hex_private_key() {
        let message = "Hello, World!";
        let invalid_key = "ghijklmnopqrstuvwxyz123456789012345678901234567890123456789012345678"; // Invalid hex

        let result = ecdsa_sign(message, invalid_key);
        assert!(result.is_err());
    }

    #[test]
    fn test_ecdsa_verify_invalid_hex_public_key() {
        let message = "Hello, World!";
        let signature = "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";
        let invalid_key =
            "ghijklmnopqrstuvwxyz123456789012345678901234567890123456789012345678901234"; // Invalid hex

        let result = ecdsa_verify(message, signature, invalid_key);
        assert!(result.is_err());
    }

    #[test]
    fn test_ecdsa_verify_invalid_hex_signature() {
        let (public_key, _) =
            dev_tools_rs::core::crypto::asymmetric::ecdsa::generate_ecdsa_keypair().unwrap();
        let message = "Hello, World!";
        let invalid_signature = "ghijklmnopqrstuvwxyz123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678"; // Invalid hex

        let result = ecdsa_verify(message, invalid_signature, &public_key);
        assert!(result.is_err());
    }

    // CryptographyProcessor Error Handling Tests
    #[test]
    fn test_processor_aes_missing_key() {
        let mut processor = CryptographyProcessor::new();
        processor.input.algorithm = CryptoAlgorithm::AES;
        processor.input.operation = CryptoOperation::Encrypt;
        processor.input.input_text = "Hello, World!".to_string();
        // Missing key

        let result = processor.process();
        assert!(result.is_err());
        assert!(processor.error.is_some());
        assert_eq!(processor.output, "");
    }

    #[test]
    fn test_processor_aes_invalid_key_size() {
        let mut processor = CryptographyProcessor::new();
        processor.input.algorithm = CryptoAlgorithm::AES;
        processor.input.operation = CryptoOperation::Encrypt;
        processor.input.input_text = "Hello, World!".to_string();
        processor.input.key = "0123456789abcdef".to_string(); // 128-bit key
        processor.input.aes_key_size = Some(AesKeySize::Aes256); // But expecting 256-bit
        processor.input.iv = Some("0123456789abcdef0123456789abcdef".to_string());

        let result = processor.process();
        assert!(result.is_err());
        assert!(processor.error.is_some());
        assert_eq!(processor.output, "");
    }

    #[test]
    fn test_processor_invalid_operation_for_algorithm() {
        let mut processor = CryptographyProcessor::new();
        processor.input.algorithm = CryptoAlgorithm::AES;
        processor.input.operation = CryptoOperation::Sign; // Invalid for AES
        processor.input.input_text = "Hello, World!".to_string();
        processor.input.key = "0123456789abcdef0123456789abcdef".to_string();

        let result = processor.process();
        assert!(result.is_err());
        assert!(processor.error.is_some());
        assert_eq!(processor.output, "");
    }

    #[test]
    fn test_processor_rsa_missing_keys() {
        let mut processor = CryptographyProcessor::new();
        processor.input.algorithm = CryptoAlgorithm::RSA;
        processor.input.operation = CryptoOperation::Encrypt;
        processor.input.input_text = "Hello, World!".to_string();
        // Missing public key

        let result = processor.process();
        assert!(result.is_err());
        assert!(processor.error.is_some());
        assert_eq!(processor.output, "");
    }

    #[test]
    fn test_processor_rsa_missing_signature() {
        let mut processor = CryptographyProcessor::new();
        processor.input.algorithm = CryptoAlgorithm::RSA;
        processor.input.operation = CryptoOperation::Verify;
        processor.input.input_text = "Hello, World!".to_string();
        processor.input.public_key = Some("public_key".to_string());
        // Missing signature

        let result = processor.process();
        assert!(result.is_err());
        assert!(processor.error.is_some());
        assert_eq!(processor.output, "");
    }

    #[test]
    fn test_processor_generate_key_missing_key_size() {
        let mut processor = CryptographyProcessor::new();
        processor.input.algorithm = CryptoAlgorithm::AES;
        processor.input.aes_key_size = None; // Missing key size

        let result = processor.generate_random_key();
        assert!(result.is_err());
    }

    #[test]
    fn test_processor_generate_iv_for_asymmetric() {
        let mut processor = CryptographyProcessor::new();
        processor.input.algorithm = CryptoAlgorithm::RSA;
        processor.input.mode = Some(CipherMode::CBC);

        let result = processor.generate_random_iv();
        assert!(result.is_err());
    }

    #[test]
    fn test_processor_error_message_format() {
        let mut processor = CryptographyProcessor::new();
        processor.input.algorithm = CryptoAlgorithm::AES;
        processor.input.operation = CryptoOperation::Encrypt;
        processor.input.input_text = "Hello, World!".to_string();
        processor.input.key = "invalid_key".to_string();

        let result = processor.process();
        assert!(result.is_err());
        assert!(processor.error.is_some());

        let error_msg = processor.error.as_ref().unwrap();
        assert!(error_msg.contains("Cryptography error"));
        assert_eq!(processor.output, "");
    }

    #[test]
    fn test_processor_error_clears_previous_output() {
        let mut processor = CryptographyProcessor::new();
        processor.output = "previous output".to_string();

        processor.input.algorithm = CryptoAlgorithm::AES;
        processor.input.operation = CryptoOperation::Encrypt;
        processor.input.input_text = "Hello, World!".to_string();
        processor.input.key = "invalid_key".to_string();

        let result = processor.process();
        assert!(result.is_err());
        assert!(processor.error.is_some());
        assert_eq!(processor.output, ""); // Should be cleared on error
    }

    #[test]
    fn test_processor_success_clears_previous_error() {
        let mut processor = CryptographyProcessor::new();
        processor.error = Some("previous error".to_string());

        processor.input.algorithm = CryptoAlgorithm::AES;
        processor.input.operation = CryptoOperation::Encrypt;
        processor.input.mode = Some(CipherMode::CBC);
        processor.input.aes_key_size = Some(AesKeySize::Aes256);
        processor.input.input_text = "Hello, World!".to_string();
        processor.input.key = "0123456789abcdef0123456789abcdef".to_string();
        processor.input.iv = Some("0123456789abcdef".to_string());

        let result = processor.process();
        assert!(result.is_ok());
        assert!(processor.error.is_none()); // Should be cleared on success
        assert!(!processor.output.is_empty());
    }
}
