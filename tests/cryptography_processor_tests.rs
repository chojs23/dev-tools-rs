use dev_tools_rs::core::crypto::{
    symmetric::aes::AesKeySize, CipherMode, CryptoAlgorithm, CryptoOperation,
    CryptographyProcessor, OutputEncoding,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cryptography_processor_new() {
        let processor = CryptographyProcessor::new();
        assert_eq!(processor.output, "");
        assert!(processor.error.is_none());
        assert_eq!(processor.input.algorithm, CryptoAlgorithm::AES);
        assert_eq!(processor.input.operation, CryptoOperation::Encrypt);
        assert_eq!(processor.input.mode, Some(CipherMode::CBC));
        assert_eq!(processor.input.aes_key_size, Some(AesKeySize::Aes128));
        assert_eq!(processor.input.encoding, OutputEncoding::Hex);
    }

    #[test]
    fn test_cryptography_processor_default() {
        let processor = CryptographyProcessor::default();
        assert_eq!(processor.output, "");
        assert!(processor.error.is_none());
        assert_eq!(processor.input.algorithm, CryptoAlgorithm::AES);
        assert_eq!(processor.input.operation, CryptoOperation::Encrypt);
        assert_eq!(processor.input.mode, Some(CipherMode::CBC));
        assert_eq!(processor.input.aes_key_size, Some(AesKeySize::Aes128));
        assert_eq!(processor.input.encoding, OutputEncoding::Hex);
    }

    #[test]
    fn test_clear_output() {
        let mut processor = CryptographyProcessor::new();
        processor.output = "test output".to_string();
        processor.error = Some("test error".to_string());

        processor.clear_output();

        assert_eq!(processor.output, "");
        assert!(processor.error.is_none());
    }

    #[test]
    fn test_process_aes_success() {
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
        let output = result.unwrap();
        assert!(!output.is_empty());
        assert_eq!(processor.output, output);
        assert!(processor.error.is_none());
        assert!(output.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_process_aes_base64_encoding() {
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
        let output = result.unwrap();
        assert!(!output.is_empty());
        assert_eq!(processor.output, output);
        assert!(processor.error.is_none());
        // Base64 encoded string should be decodable
        use base64::{engine::general_purpose, Engine};
        assert!(general_purpose::STANDARD.decode(&output).is_ok());
    }

    #[test]
    fn test_process_aes_decrypt() {
        let mut processor = CryptographyProcessor::new();
        processor.input.algorithm = CryptoAlgorithm::AES;
        processor.input.operation = CryptoOperation::Encrypt;
        processor.input.mode = Some(CipherMode::CBC);
        processor.input.aes_key_size = Some(AesKeySize::Aes256);
        processor.input.input_text = "Hello, World!".to_string();
        processor.input.key = "0123456789abcdef0123456789abcdef".to_string();
        processor.input.iv = Some("0123456789abcdef".to_string());
        processor.input.encoding = OutputEncoding::Hex;

        // First encrypt
        let encrypt_result = processor.process();
        assert!(encrypt_result.is_ok());
        let encrypted = encrypt_result.unwrap();

        // Then decrypt
        processor.input.operation = CryptoOperation::Decrypt;
        processor.input.input_text = encrypted;

        let decrypt_result = processor.process();
        assert!(decrypt_result.is_ok());
        let decrypted = decrypt_result.unwrap();
        assert_eq!(decrypted, "Hello, World!");
    }

    #[test]
    fn test_process_des_encrypt_decrypt() {
        let mut processor = CryptographyProcessor::new();
        processor.input.algorithm = CryptoAlgorithm::DES;
        processor.input.operation = CryptoOperation::Encrypt;
        processor.input.mode = Some(CipherMode::CBC);
        processor.input.input_text = "Hello, World!".to_string();
        processor.input.key = "01234567".to_string();
        processor.input.iv = Some("01234567".to_string());
        processor.input.encoding = OutputEncoding::Hex;

        // First encrypt
        let encrypt_result = processor.process();
        assert!(encrypt_result.is_ok());
        let encrypted = encrypt_result.unwrap();
        assert!(!encrypted.is_empty());

        // Then decrypt
        processor.input.operation = CryptoOperation::Decrypt;
        processor.input.input_text = encrypted;

        let decrypt_result = processor.process();
        assert!(decrypt_result.is_ok());
        let decrypted = decrypt_result.unwrap();
        assert_eq!(decrypted, "Hello, World!");
    }

    #[test]
    fn test_process_triple_des_encrypt_decrypt() {
        let mut processor = CryptographyProcessor::new();
        processor.input.algorithm = CryptoAlgorithm::TripleDES;
        processor.input.operation = CryptoOperation::Encrypt;
        processor.input.mode = Some(CipherMode::CBC);
        processor.input.input_text = "Hello, World!".to_string();
        processor.input.key = "0123456789abcdef01234567".to_string();
        processor.input.iv = Some("01234567".to_string());
        processor.input.encoding = OutputEncoding::Hex;

        // First encrypt
        let encrypt_result = processor.process();
        assert!(encrypt_result.is_ok());
        let encrypted = encrypt_result.unwrap();
        assert!(!encrypted.is_empty());

        // Then decrypt
        processor.input.operation = CryptoOperation::Decrypt;
        processor.input.input_text = encrypted;

        let decrypt_result = processor.process();
        assert!(decrypt_result.is_ok());
        let decrypted = decrypt_result.unwrap();
        assert_eq!(decrypted, "Hello, World!");
    }

    #[test]
    fn test_process_rsa_encrypt_decrypt() {
        let mut processor = CryptographyProcessor::new();
        processor.input.algorithm = CryptoAlgorithm::RSA;
        processor.input.input_text = "Hello, World!".to_string();

        // Generate keypair
        processor.generate_random_key().unwrap();
        let public_key = processor.input.public_key.clone().unwrap();
        let private_key = processor.input.private_key.clone().unwrap();

        // Encrypt
        processor.input.operation = CryptoOperation::Encrypt;
        processor.input.public_key = Some(public_key);

        let encrypt_result = processor.process();
        assert!(encrypt_result.is_ok());
        let encrypted = encrypt_result.unwrap();
        assert!(!encrypted.is_empty());

        // Decrypt
        processor.input.operation = CryptoOperation::Decrypt;
        processor.input.input_text = encrypted;
        processor.input.private_key = Some(private_key);

        let decrypt_result = processor.process();
        assert!(decrypt_result.is_ok());
        let decrypted = decrypt_result.unwrap();
        assert_eq!(decrypted, "Hello, World!");
    }

    #[test]
    fn test_process_rsa_sign_verify() {
        let mut processor = CryptographyProcessor::new();
        processor.input.algorithm = CryptoAlgorithm::RSA;
        processor.input.input_text = "Hello, World!".to_string();

        // Generate keypair
        processor.generate_random_key().unwrap();
        let public_key = processor.input.public_key.clone().unwrap();
        let private_key = processor.input.private_key.clone().unwrap();

        // Sign
        processor.input.operation = CryptoOperation::Sign;
        processor.input.private_key = Some(private_key);

        let sign_result = processor.process();
        assert!(sign_result.is_ok());
        let signature = sign_result.unwrap();
        assert!(!signature.is_empty());

        // Verify
        processor.input.operation = CryptoOperation::Verify;
        processor.input.signature = Some(signature);
        processor.input.public_key = Some(public_key);

        let verify_result = processor.process();
        assert!(verify_result.is_ok());
        let verification = verify_result.unwrap();
        assert!(verification.contains("true"));
    }

    #[test]
    fn test_process_error_handling() {
        let mut processor = CryptographyProcessor::new();
        processor.input.algorithm = CryptoAlgorithm::AES;
        processor.input.operation = CryptoOperation::Encrypt;
        processor.input.input_text = "Hello, World!".to_string();
        processor.input.key = "invalid_key".to_string(); // Invalid key

        let result = processor.process();
        assert!(result.is_err());
        assert!(processor.error.is_some());
        assert_eq!(processor.output, "");
        assert!(processor
            .error
            .as_ref()
            .unwrap()
            .contains("Cryptography error"));
    }

    #[test]
    fn test_process_invalid_operation_for_aes() {
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
    fn test_process_missing_public_key_for_rsa() {
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
    fn test_process_missing_private_key_for_rsa() {
        let mut processor = CryptographyProcessor::new();
        processor.input.algorithm = CryptoAlgorithm::RSA;
        processor.input.operation = CryptoOperation::Decrypt;
        processor.input.input_text = "encrypted_text".to_string();
        // Missing private key

        let result = processor.process();
        assert!(result.is_err());
        assert!(processor.error.is_some());
        assert_eq!(processor.output, "");
    }

    #[test]
    fn test_process_missing_signature_for_verification() {
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
    fn test_process_multiple_operations() {
        let mut processor = CryptographyProcessor::new();
        processor.input.algorithm = CryptoAlgorithm::AES;
        processor.input.mode = Some(CipherMode::CBC);
        processor.input.aes_key_size = Some(AesKeySize::Aes256);
        processor.input.input_text = "Hello, World!".to_string();
        processor.input.key = "0123456789abcdef0123456789abcdef".to_string();
        processor.input.iv = Some("0123456789abcdef".to_string());
        processor.input.encoding = OutputEncoding::Hex;

        // First operation - encrypt
        processor.input.operation = CryptoOperation::Encrypt;
        let encrypt_result = processor.process();
        assert!(encrypt_result.is_ok());
        let encrypted = encrypt_result.unwrap();

        // Second operation - decrypt
        processor.input.operation = CryptoOperation::Decrypt;
        processor.input.input_text = encrypted.clone();
        let decrypt_result = processor.process();
        assert!(decrypt_result.is_ok());
        let decrypted = decrypt_result.unwrap();
        assert_eq!(decrypted, "Hello, World!");

        // Third operation - encrypt with different text
        processor.input.operation = CryptoOperation::Encrypt;
        processor.input.input_text = "Different text".to_string();
        let encrypt_result2 = processor.process();
        assert!(encrypt_result2.is_ok());
        let encrypted2 = encrypt_result2.unwrap();
        assert_ne!(encrypted2, encrypted); // Should be different
    }

    #[test]
    fn test_process_with_different_modes() {
        let mut processor = CryptographyProcessor::new();
        processor.input.algorithm = CryptoAlgorithm::AES;
        processor.input.operation = CryptoOperation::Encrypt;
        processor.input.aes_key_size = Some(AesKeySize::Aes256);
        processor.input.input_text = "Hello, World!".to_string();
        processor.input.key = "0123456789abcdef0123456789abcdef".to_string();
        processor.input.encoding = OutputEncoding::Hex;

        // Test CBC mode
        processor.input.mode = Some(CipherMode::CBC);
        processor.input.iv = Some("0123456789abcdef".to_string());
        let cbc_result = processor.process();
        assert!(cbc_result.is_ok());
        let cbc_encrypted = cbc_result.unwrap();

        // Test ECB mode
        processor.input.mode = Some(CipherMode::ECB);
        processor.input.iv = None;
        let ecb_result = processor.process();
        assert!(ecb_result.is_ok());
        let ecb_encrypted = ecb_result.unwrap();

        // Results should be different
        assert_ne!(cbc_encrypted, ecb_encrypted);
    }

    #[test]
    fn test_process_with_different_key_sizes() {
        let mut processor = CryptographyProcessor::new();
        processor.input.algorithm = CryptoAlgorithm::AES;
        processor.input.operation = CryptoOperation::Encrypt;
        processor.input.mode = Some(CipherMode::CBC);
        processor.input.input_text = "Hello, World!".to_string();
        processor.input.iv = Some("0123456789abcdef".to_string());
        processor.input.encoding = OutputEncoding::Hex;

        // Test AES-128
        processor.input.aes_key_size = Some(AesKeySize::Aes128);
        processor.input.key = "0123456789abcdef".to_string();
        let aes128_result = processor.process();
        assert!(aes128_result.is_ok());

        // Test AES-192
        processor.input.aes_key_size = Some(AesKeySize::Aes192);
        processor.input.key = "16dd3db6c0dc49b2f8c8c59d".to_string();
        let aes192_result = processor.process();
        assert!(aes192_result.is_ok());

        // Test AES-256
        processor.input.aes_key_size = Some(AesKeySize::Aes256);
        processor.input.key = "0123456789abcdef0123456789abcdef".to_string();
        let aes256_result = processor.process();
        assert!(aes256_result.is_ok());

        // All should succeed but produce different results
        assert_ne!(aes128_result.unwrap(), aes192_result.unwrap());
    }
}
