use dev_tools_rs::core::crypto::{CryptographyProcessor, CryptoAlgorithm, KeyGenerationState};
use std::time::Duration;
use std::thread;

#[test]
fn test_async_rsa_key_generation() {
    let mut processor = CryptographyProcessor::new();
    
    // Set algorithm to RSA
    processor.input.algorithm = CryptoAlgorithm::RSA;
    
    // Start async key generation
    processor.generate_random_key().expect("Failed to start key generation");
    
    // Should be in generating state
    assert!(processor.is_key_generation_in_progress());
    assert!(!processor.is_key_generation_complete());
    
    // Check state initially
    assert_eq!(processor.key_generation_state, KeyGenerationState::Generating);
    
    // Wait for completion (with timeout)
    let mut attempts = 0;
    let max_attempts = 100; // 10 seconds timeout
    
    while !processor.is_key_generation_complete() && attempts < max_attempts {
        processor.check_key_generation_progress();
        thread::sleep(Duration::from_millis(100));
        attempts += 1;
    }
    
    // Should be completed
    assert!(processor.is_key_generation_complete());
    assert!(!processor.is_key_generation_in_progress());
    
    // Check if keys were generated
    match processor.key_generation_state {
        KeyGenerationState::Completed => {
            assert!(processor.input.public_key.is_some());
            assert!(processor.input.private_key.is_some());
            
            let public_key = processor.input.public_key.as_ref().unwrap();
            let private_key = processor.input.private_key.as_ref().unwrap();
            
            // Basic validation of PEM format
            println!("Public key format: {}", &public_key[..50]);
            println!("Private key format: {}", &private_key[..50]);
            
            // Check for standard PEM format
            assert!(public_key.starts_with("-----BEGIN PUBLIC KEY-----"));
            assert!(public_key.contains("-----END PUBLIC KEY-----"));
            assert!(private_key.starts_with("-----BEGIN PRIVATE KEY-----"));
            assert!(private_key.contains("-----END PRIVATE KEY-----"));
        }
        KeyGenerationState::Failed(error) => {
            panic!("Key generation failed: {}", error);
        }
        _ => {
            panic!("Key generation did not complete within timeout");
        }
    }
}

#[test]
fn test_reset_key_generation_state() {
    let mut processor = CryptographyProcessor::new();
    
    // Set algorithm to RSA
    processor.input.algorithm = CryptoAlgorithm::RSA;
    
    // Start async key generation
    processor.generate_random_key().expect("Failed to start key generation");
    
    // Should be in generating state
    assert!(processor.is_key_generation_in_progress());
    
    // Reset state
    processor.reset_key_generation_state();
    
    // Should be back to idle
    assert_eq!(processor.key_generation_state, KeyGenerationState::Idle);
    assert!(!processor.is_key_generation_in_progress());
    assert!(!processor.is_key_generation_complete());
}

#[test]
fn test_multiple_key_generation_attempts() {
    let mut processor = CryptographyProcessor::new();
    
    // Set algorithm to RSA
    processor.input.algorithm = CryptoAlgorithm::RSA;
    
    // Start first key generation
    processor.generate_random_key().expect("Failed to start first key generation");
    assert!(processor.is_key_generation_in_progress());
    
    // Try to start another - should not change state
    processor.generate_random_key().expect("Failed to handle second key generation attempt");
    assert!(processor.is_key_generation_in_progress());
    
    // Wait for completion
    let mut attempts = 0;
    let max_attempts = 100;
    
    while !processor.is_key_generation_complete() && attempts < max_attempts {
        processor.check_key_generation_progress();
        thread::sleep(Duration::from_millis(100));
        attempts += 1;
    }
    
    // Should be completed
    assert!(processor.is_key_generation_complete());
}

