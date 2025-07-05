use dev_tools_rs::core::crypto::{
    symmetric::aes::AesKeySize, CipherMode, CryptoAlgorithm, CryptoInput, CryptoOperation,
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
    fn test_crypto_algorithm_is_symmetric() {
        assert!(CryptoAlgorithm::AES.is_symmetric());
        assert!(CryptoAlgorithm::DES.is_symmetric());
        assert!(CryptoAlgorithm::TripleDES.is_symmetric());
        assert!(!CryptoAlgorithm::RSA.is_symmetric());
    }

    #[test]
    fn test_crypto_algorithm_is_asymmetric() {
        assert!(!CryptoAlgorithm::AES.is_asymmetric());
        assert!(!CryptoAlgorithm::DES.is_asymmetric());
        assert!(!CryptoAlgorithm::TripleDES.is_asymmetric());
        assert!(CryptoAlgorithm::RSA.is_asymmetric());
    }

    #[test]
    fn test_cipher_mode_variants() {
        let variants = CipherMode::variants();
        assert_eq!(variants.len(), 2);
        assert!(variants.contains(&CipherMode::ECB));
        assert!(variants.contains(&CipherMode::CBC));
    }

    #[test]
    fn test_output_encoding_variants() {
        let variants = OutputEncoding::variants();
        assert_eq!(variants.len(), 2);
        assert!(variants.contains(&OutputEncoding::Hex));
        assert!(variants.contains(&OutputEncoding::Base64));
    }

    #[test]
    fn test_output_encoding_default() {
        let default_encoding = OutputEncoding::default();
        assert_eq!(default_encoding, OutputEncoding::Hex);
    }

    #[test]
    fn test_crypto_input_default() {
        let input = CryptoInput::default();
        assert_eq!(input.algorithm, CryptoAlgorithm::AES);
        assert_eq!(input.operation, CryptoOperation::Encrypt);
        assert_eq!(input.mode, Some(CipherMode::CBC));
        assert_eq!(input.aes_key_size, Some(AesKeySize::Aes128));
        assert_eq!(input.encoding, OutputEncoding::Hex);
        assert_eq!(input.input_text, "");
        assert_eq!(input.key, "");
        assert!(input.iv.is_none());
        assert!(input.public_key.is_none());
        assert!(input.private_key.is_none());
        assert!(input.signature.is_none());
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
    fn test_display_traits() {
        assert_eq!(format!("{}", CryptoAlgorithm::AES), "AES");
        assert_eq!(format!("{}", CryptoAlgorithm::DES), "DES");
        assert_eq!(format!("{}", CryptoAlgorithm::TripleDES), "Triple DES");
        assert_eq!(format!("{}", CryptoAlgorithm::RSA), "RSA");

        assert_eq!(format!("{}", CipherMode::ECB), "ECB");
        assert_eq!(format!("{}", CipherMode::CBC), "CBC");

        assert_eq!(format!("{}", CryptoOperation::Encrypt), "Encrypt");
        assert_eq!(format!("{}", CryptoOperation::Decrypt), "Decrypt");
        assert_eq!(format!("{}", CryptoOperation::Sign), "Sign");
        assert_eq!(format!("{}", CryptoOperation::Verify), "Verify");

        assert_eq!(format!("{}", OutputEncoding::Hex), "Hex");
        assert_eq!(format!("{}", OutputEncoding::Base64), "Base64");
    }

    #[test]
    fn test_process_without_valid_input() {
        let mut processor = CryptographyProcessor::new();
        processor.input.input_text = "test".to_string();

        let result = processor.process();
        assert!(result.is_err());
        assert!(processor.error.is_some());
        assert_eq!(processor.output, "");
    }

    #[test]
    fn test_generate_random_key_aes() {
        let mut processor = CryptographyProcessor::new();
        processor.input.algorithm = CryptoAlgorithm::AES;
        processor.input.aes_key_size = Some(AesKeySize::Aes128);

        let result = processor.generate_random_key();
        assert!(result.is_ok());
        assert!(!processor.input.key.is_empty());
        assert_eq!(processor.input.key.len(), 16); // 128 bits = 16 hex chars
        assert!(processor.input.key.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_generate_random_key_des() {
        let mut processor = CryptographyProcessor::new();
        processor.input.algorithm = CryptoAlgorithm::DES;

        let result = processor.generate_random_key();
        assert!(result.is_ok());
        assert!(!processor.input.key.is_empty());
        assert_eq!(processor.input.key.len(), 8); // 64 bits = 8 hex chars
        assert!(processor.input.key.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_generate_random_key_triple_des() {
        let mut processor = CryptographyProcessor::new();
        processor.input.algorithm = CryptoAlgorithm::TripleDES;

        let result = processor.generate_random_key();
        assert!(result.is_ok());
        assert!(!processor.input.key.is_empty());
        assert_eq!(processor.input.key.len(), 24); // 192 bits = 24 hex chars
        assert!(processor.input.key.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_generate_random_key_rsa() {
        let mut processor = CryptographyProcessor::new();
        processor.input.algorithm = CryptoAlgorithm::RSA;

        let result = processor.generate_random_key();
        assert!(result.is_ok());
        assert!(processor.input.public_key.is_some());
        assert!(processor.input.private_key.is_some());
        assert!(!processor.input.public_key.as_ref().unwrap().is_empty());
        assert!(!processor.input.private_key.as_ref().unwrap().is_empty());
    }

    #[test]
    fn test_generate_random_iv_aes() {
        let mut processor = CryptographyProcessor::new();
        processor.input.algorithm = CryptoAlgorithm::AES;
        processor.input.mode = Some(CipherMode::CBC);

        let result = processor.generate_random_iv();
        assert!(result.is_ok());
        assert!(processor.input.iv.is_some());
        assert!(!processor.input.iv.as_ref().unwrap().is_empty());
        assert_eq!(processor.input.iv.as_ref().unwrap().len(), 16); // 128 bits = 16 hex chars
        assert!(processor
            .input
            .iv
            .as_ref()
            .unwrap()
            .chars()
            .all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_generate_random_iv_des() {
        let mut processor = CryptographyProcessor::new();
        processor.input.algorithm = CryptoAlgorithm::DES;
        processor.input.mode = Some(CipherMode::CBC);

        let result = processor.generate_random_iv();
        assert!(result.is_ok());
        assert!(processor.input.iv.is_some());
        assert!(!processor.input.iv.as_ref().unwrap().is_empty());
        assert_eq!(processor.input.iv.as_ref().unwrap().len(), 8); // 64 bits = 8 hex chars
        assert!(processor
            .input
            .iv
            .as_ref()
            .unwrap()
            .chars()
            .all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_generate_random_iv_triple_des() {
        let mut processor = CryptographyProcessor::new();
        processor.input.algorithm = CryptoAlgorithm::TripleDES;
        processor.input.mode = Some(CipherMode::CBC);

        let result = processor.generate_random_iv();
        assert!(result.is_ok());
        assert!(processor.input.iv.is_some());
        assert!(!processor.input.iv.as_ref().unwrap().is_empty());
        assert_eq!(processor.input.iv.as_ref().unwrap().len(), 8); // 64 bits = 8 hex chars
        assert!(processor
            .input
            .iv
            .as_ref()
            .unwrap()
            .chars()
            .all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_generate_random_iv_ecb_mode() {
        let mut processor = CryptographyProcessor::new();
        processor.input.algorithm = CryptoAlgorithm::AES;
        processor.input.mode = Some(CipherMode::ECB);

        let result = processor.generate_random_iv();
        assert!(result.is_ok());
        assert!(processor.input.iv.is_none()); // ECB mode doesn't use IV
    }

    #[test]
    fn test_generate_random_iv_asymmetric_error() {
        let mut processor = CryptographyProcessor::new();
        processor.input.algorithm = CryptoAlgorithm::RSA;
        processor.input.mode = Some(CipherMode::CBC);

        let result = processor.generate_random_iv();
        assert!(result.is_err());
    }
}
