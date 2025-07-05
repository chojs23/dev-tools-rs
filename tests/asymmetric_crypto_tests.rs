use dev_tools_rs::core::crypto::{
    asymmetric::rsa::{generate_rsa_keypair, rsa_decrypt, rsa_encrypt, rsa_sign, rsa_verify},
    CryptoAlgorithm, CryptoOperation, CryptographyProcessor,
};

#[cfg(test)]
mod tests {
    use super::*;

    // RSA Tests
    #[test]
    fn test_rsa_generate_keypair() {
        let result = generate_rsa_keypair(2048);
        assert!(result.is_ok());
        let (public_key, private_key) = result.unwrap();
        assert!(!public_key.is_empty());
        assert!(!private_key.is_empty());
        assert!(public_key.contains("BEGIN PUBLIC KEY"));
        assert!(private_key.contains("BEGIN PRIVATE KEY"));
    }

    #[test]
    fn test_rsa_encrypt_decrypt() {
        let (public_key, private_key) = generate_rsa_keypair(2048).unwrap();
        let plaintext = "Hello, World!";

        let encrypted = rsa_encrypt(plaintext, &public_key).expect("Encryption should succeed");

        let decrypted =
            rsa_decrypt(&hex::encode(encrypted), &private_key).expect("Decryption should succeed");

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_rsa_sign_verify() {
        let (public_key, private_key) = generate_rsa_keypair(2048).unwrap();
        let message = "Hello, World!";

        let signature = rsa_sign(message, &private_key).expect("Signing should succeed");

        let is_valid = rsa_verify(message, &hex::encode(signature), &public_key)
            .expect("Verification should succeed");

        assert!(is_valid);
    }

    #[test]
    fn test_rsa_sign_verify_invalid_signature() {
        let (public_key, private_key) = generate_rsa_keypair(2048).unwrap();
        let message = "Hello, World!";
        let wrong_message = "Goodbye, World!";

        let signature = rsa_sign(message, &private_key).expect("Signing should succeed");

        let is_valid = rsa_verify(wrong_message, &hex::encode(signature), &public_key)
            .expect("Verification should succeed");

        assert!(!is_valid);
    }

    #[test]
    fn test_rsa_encrypt_with_invalid_key() {
        let invalid_key = "invalid_key";
        let plaintext = "Hello, World!";

        let result = rsa_encrypt(plaintext, invalid_key);
        assert!(result.is_err());
    }

    #[test]
    fn test_rsa_decrypt_with_invalid_key() {
        let (public_key, _) = generate_rsa_keypair(2048).unwrap();
        let plaintext = "Hello, World!";
        let encrypted = rsa_encrypt(plaintext, &public_key).unwrap();

        let invalid_key = "invalid_key";
        let result = rsa_decrypt(&hex::encode(encrypted), invalid_key);
        assert!(result.is_err());
    }

    #[test]
    fn test_rsa_sign_with_invalid_key() {
        let message = "Hello, World!";
        let invalid_key = "invalid_key";

        let result = rsa_sign(message, invalid_key);
        assert!(result.is_err());
    }

    #[test]
    fn test_rsa_verify_with_invalid_key() {
        let (_, private_key) = generate_rsa_keypair(2048).unwrap();
        let message = "Hello, World!";
        let signature = rsa_sign(message, &private_key).unwrap();

        let invalid_key = "invalid_key";
        let result = rsa_verify(message, &hex::encode(signature), invalid_key);
        assert!(result.is_err());
    }

    #[test]
    fn test_different_rsa_keypairs_dont_verify() {
        let (public_key1, _) = generate_rsa_keypair(2048).unwrap();
        let (_, private_key2) = generate_rsa_keypair(2048).unwrap();
        let message = "Hello, World!";

        let signature = rsa_sign(message, &private_key2).unwrap();
        let is_valid = rsa_verify(message, &hex::encode(signature), &public_key1).unwrap();

        assert!(!is_valid);
    }

    // Integration Tests with CryptographyProcessor
    #[test]
    fn test_crypto_processor_rsa_encrypt_decrypt() {
        let mut processor = CryptographyProcessor::new();
        processor.input.algorithm = CryptoAlgorithm::RSA;

        // Generate keypair
        processor.generate_random_key().unwrap();
        let public_key = processor.input.public_key.clone().unwrap();
        let private_key = processor.input.private_key.clone().unwrap();

        // Encrypt
        processor.input.operation = CryptoOperation::Encrypt;
        processor.input.input_text = "Hello, World!".to_string();
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
    fn test_crypto_processor_rsa_sign_verify() {
        let mut processor = CryptographyProcessor::new();
        processor.input.algorithm = CryptoAlgorithm::RSA;

        // Generate keypair
        processor.generate_random_key().unwrap();
        let public_key = processor.input.public_key.clone().unwrap();
        let private_key = processor.input.private_key.clone().unwrap();

        // Sign
        processor.input.operation = CryptoOperation::Sign;
        processor.input.input_text = "Hello, World!".to_string();
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
    fn test_crypto_processor_ecdsa_invalid_operation() {
        let mut processor = CryptographyProcessor::new();
        processor.input.operation = CryptoOperation::Encrypt; // Invalid for ECDSA
        processor.input.input_text = "Hello, World!".to_string();

        let result = processor.process();
        assert!(result.is_err());
        assert!(processor.error.is_some());
    }

    #[test]
    fn test_crypto_processor_rsa_missing_public_key() {
        let mut processor = CryptographyProcessor::new();
        processor.input.algorithm = CryptoAlgorithm::RSA;
        processor.input.operation = CryptoOperation::Encrypt;
        processor.input.input_text = "Hello, World!".to_string();
        // Missing public key

        let result = processor.process();
        assert!(result.is_err());
        assert!(processor.error.is_some());
    }

    #[test]
    fn test_crypto_processor_rsa_missing_private_key() {
        let mut processor = CryptographyProcessor::new();
        processor.input.algorithm = CryptoAlgorithm::RSA;
        processor.input.operation = CryptoOperation::Decrypt;
        processor.input.input_text = "encrypted_text".to_string();
        // Missing private key

        let result = processor.process();
        assert!(result.is_err());
        assert!(processor.error.is_some());
    }

    #[test]
    fn test_crypto_processor_ecdsa_missing_signature() {
        let mut processor = CryptographyProcessor::new();
        processor.input.operation = CryptoOperation::Verify;
        processor.input.input_text = "Hello, World!".to_string();
        processor.input.public_key = Some("public_key".to_string());
        // Missing signature

        let result = processor.process();
        assert!(result.is_err());
        assert!(processor.error.is_some());
    }
}
