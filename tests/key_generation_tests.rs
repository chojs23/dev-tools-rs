use dev_tools_rs::core::crypto::{
    asymmetric::{
        generate_des_iv, generate_des_key, generate_triple_des_key, rsa::generate_rsa_keypair,
    },
    symmetric::{aes::AesKeySize, generate_aes_iv, generate_aes_key},
};

#[cfg(test)]
mod tests {
    use super::*;

    // AES Key Generation Tests
    #[test]
    fn test_generate_aes_key_128() {
        let key = generate_aes_key(AesKeySize::Aes128);
        assert_eq!(key.len(), 16); // 128 bits = 16 hex chars
        assert!(key.chars().all(|c| c.is_ascii_hexdigit()));
        assert!(key
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()));
    }

    #[test]
    fn test_generate_aes_key_192() {
        let key = generate_aes_key(AesKeySize::Aes192);
        assert_eq!(key.len(), 24); // 192 bits = 24 hex chars
        assert!(key.chars().all(|c| c.is_ascii_hexdigit()));
        assert!(key
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()));
    }

    #[test]
    fn test_generate_aes_key_256() {
        let key = generate_aes_key(AesKeySize::Aes256);
        assert_eq!(key.len(), 32); // 256 bits = 32 hex chars
        assert!(key.chars().all(|c| c.is_ascii_hexdigit()));
        assert!(key
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()));
    }

    #[test]
    fn test_generate_different_aes_keys() {
        let key1 = generate_aes_key(AesKeySize::Aes256);
        let key2 = generate_aes_key(AesKeySize::Aes256);
        assert_ne!(key1, key2); // Generated keys should be different
    }

    #[test]
    fn test_generate_multiple_aes_keys_different_sizes() {
        let key128 = generate_aes_key(AesKeySize::Aes128);
        let key192 = generate_aes_key(AesKeySize::Aes192);
        let key256 = generate_aes_key(AesKeySize::Aes256);

        assert_eq!(key128.len(), 16);
        assert_eq!(key192.len(), 24);
        assert_eq!(key256.len(), 32);

        // All should be different
        assert_ne!(key128, key192[..16]);
        assert_ne!(key128, key256[..16]);
        assert_ne!(key192[..16], key256[..16]);
    }

    // AES IV Generation Tests
    #[test]
    fn test_generate_aes_iv() {
        let iv = generate_aes_iv();
        assert_eq!(iv.len(), 16); // 128 bits = 16 hex chars
        assert!(iv.chars().all(|c| c.is_ascii_hexdigit()));
        assert!(iv
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()));
    }

    #[test]
    fn test_generate_different_aes_ivs() {
        let iv1 = generate_aes_iv();
        let iv2 = generate_aes_iv();
        assert_ne!(iv1, iv2); // Generated IVs should be different
    }

    #[test]
    fn test_generate_multiple_aes_ivs() {
        let mut ivs = Vec::new();
        for _ in 0..10 {
            ivs.push(generate_aes_iv());
        }

        // Check all are different
        for i in 0..ivs.len() {
            for j in i + 1..ivs.len() {
                assert_ne!(ivs[i], ivs[j]);
            }
        }

        // Check all have correct format
        for iv in ivs {
            assert_eq!(iv.len(), 16);
            assert!(iv.chars().all(|c| c.is_ascii_hexdigit()));
        }
    }

    // DES Key Generation Tests
    #[test]
    fn test_generate_des_key() {
        let key = generate_des_key();
        assert_eq!(key.len(), 8); // 64 bits = 8 hex chars
        assert!(key.chars().all(|c| c.is_ascii_hexdigit()));
        assert!(key
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()));
    }

    #[test]
    fn test_generate_different_des_keys() {
        let key1 = generate_des_key();
        let key2 = generate_des_key();
        assert_ne!(key1, key2); // Generated keys should be different
    }

    #[test]
    fn test_generate_multiple_des_keys() {
        let mut keys = Vec::new();
        for _ in 0..10 {
            keys.push(generate_des_key());
        }

        // Check all are different
        for i in 0..keys.len() {
            for j in i + 1..keys.len() {
                assert_ne!(keys[i], keys[j]);
            }
        }

        // Check all have correct format
        for key in keys {
            assert_eq!(key.len(), 8);
            assert!(key.chars().all(|c| c.is_ascii_hexdigit()));
        }
    }

    // Triple DES Key Generation Tests
    #[test]
    fn test_generate_triple_des_key() {
        let key = generate_triple_des_key();
        assert_eq!(key.len(), 24); // 192 bits = 24 hex chars
        assert!(key.chars().all(|c| c.is_ascii_hexdigit()));
        assert!(key
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()));
    }

    #[test]
    fn test_generate_different_triple_des_keys() {
        let key1 = generate_triple_des_key();
        let key2 = generate_triple_des_key();
        assert_ne!(key1, key2); // Generated keys should be different
    }

    #[test]
    fn test_generate_multiple_triple_des_keys() {
        let mut keys = Vec::new();
        for _ in 0..10 {
            keys.push(generate_triple_des_key());
        }

        // Check all are different
        for i in 0..keys.len() {
            for j in i + 1..keys.len() {
                assert_ne!(keys[i], keys[j]);
            }
        }

        // Check all have correct format
        for key in keys {
            assert_eq!(key.len(), 24);
            assert!(key.chars().all(|c| c.is_ascii_hexdigit()));
        }
    }

    // DES IV Generation Tests
    #[test]
    fn test_generate_des_iv() {
        let iv = generate_des_iv();
        assert_eq!(iv.len(), 8); // 64 bits = 8 hex chars
        assert!(iv.chars().all(|c| c.is_ascii_hexdigit()));
        assert!(iv
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()));
    }

    #[test]
    fn test_generate_different_des_ivs() {
        let iv1 = generate_des_iv();
        let iv2 = generate_des_iv();
        assert_ne!(iv1, iv2); // Generated IVs should be different
    }

    #[test]
    fn test_generate_multiple_des_ivs() {
        let mut ivs = Vec::new();
        for _ in 0..10 {
            ivs.push(generate_des_iv());
        }

        // Check all are different
        for i in 0..ivs.len() {
            for j in i + 1..ivs.len() {
                assert_ne!(ivs[i], ivs[j]);
            }
        }

        // Check all have correct format
        for iv in ivs {
            assert_eq!(iv.len(), 8);
            assert!(iv.chars().all(|c| c.is_ascii_hexdigit()));
        }
    }

    // RSA Key Generation Tests
    #[test]
    fn test_generate_rsa_keypair() {
        let result = generate_rsa_keypair(2048);
        assert!(result.is_ok());

        let (public_key, private_key) = result.unwrap();
        assert!(!public_key.is_empty());
        assert!(!private_key.is_empty());
        assert!(public_key.contains("BEGIN PUBLIC KEY"));
        assert!(public_key.contains("END PUBLIC KEY"));
        assert!(private_key.contains("BEGIN PRIVATE KEY"));
        assert!(private_key.contains("END PRIVATE KEY"));
    }

    #[test]
    fn test_generate_different_rsa_keypairs() {
        let result1 = generate_rsa_keypair(2048);
        let result2 = generate_rsa_keypair(2048);

        assert!(result1.is_ok());
        assert!(result2.is_ok());

        let (public_key1, private_key1) = result1.unwrap();
        let (public_key2, private_key2) = result2.unwrap();

        assert_ne!(public_key1, public_key2);
        assert_ne!(private_key1, private_key2);
    }

    #[test]
    fn test_generate_multiple_rsa_keypairs() {
        let mut keypairs = Vec::new();
        for _ in 0..3 {
            // Generate fewer RSA keypairs as they're slower
            let result = generate_rsa_keypair(2048);
            assert!(result.is_ok());
            keypairs.push(result.unwrap());
        }

        // Check all are different
        for i in 0..keypairs.len() {
            for j in i + 1..keypairs.len() {
                assert_ne!(keypairs[i].0, keypairs[j].0); // Different public keys
                assert_ne!(keypairs[i].1, keypairs[j].1); // Different private keys
            }
        }

        // Check all have correct format
        for (public_key, private_key) in keypairs {
            assert!(public_key.contains("BEGIN PUBLIC KEY"));
            assert!(public_key.contains("END PUBLIC KEY"));
            assert!(private_key.contains("BEGIN PRIVATE KEY"));
            assert!(private_key.contains("END PRIVATE KEY"));
        }
    }

    // Key Format Tests
    #[test]
    fn test_key_format_consistency() {
        // Test that keys are consistently formatted as lowercase hex
        let aes_key = generate_aes_key(AesKeySize::Aes256);
        let des_key = generate_des_key();
        let triple_des_key = generate_triple_des_key();
        let aes_iv = generate_aes_iv();
        let des_iv = generate_des_iv();

        // All should be lowercase hex
        assert!(aes_key
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()));
        assert!(des_key
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()));
        assert!(triple_des_key
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()));
        assert!(aes_iv
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()));
        assert!(des_iv
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()));
    }

    #[test]
    fn test_key_entropy() {
        // Test that generated keys have good entropy (not all zeros or all same character)
        let aes_key = generate_aes_key(AesKeySize::Aes256);
        let des_key = generate_des_key();
        let triple_des_key = generate_triple_des_key();

        // Check that keys are not all the same character
        assert!(!aes_key
            .chars()
            .all(|c| c == aes_key.chars().next().unwrap()));
        assert!(!des_key
            .chars()
            .all(|c| c == des_key.chars().next().unwrap()));
        assert!(!triple_des_key
            .chars()
            .all(|c| c == triple_des_key.chars().next().unwrap()));

        // Check that keys are not all zeros
        assert_ne!(aes_key, "0".repeat(32));
        assert_ne!(des_key, "0".repeat(8));
        assert_ne!(triple_des_key, "0".repeat(24));
    }

    #[test]
    fn test_iv_entropy() {
        // Test that generated IVs have good entropy
        let aes_iv = generate_aes_iv();
        let des_iv = generate_des_iv();

        // Check that IVs are not all the same character
        assert!(!aes_iv.chars().all(|c| c == aes_iv.chars().next().unwrap()));
        assert!(!des_iv.chars().all(|c| c == des_iv.chars().next().unwrap()));

        // Check that IVs are not all zeros
        assert_ne!(aes_iv, "0".repeat(16));
        assert_ne!(des_iv, "0".repeat(8));
    }

    #[test]
    fn test_key_generation_performance() {
        // Test that key generation is reasonably fast
        use std::time::Instant;

        let start = Instant::now();
        for _ in 0..100 {
            let _ = generate_aes_key(AesKeySize::Aes256);
            let _ = generate_des_key();
            let _ = generate_triple_des_key();
            let _ = generate_aes_iv();
            let _ = generate_des_iv();
        }
        let duration = start.elapsed();

        // Should complete in reasonable time (less than 1 second for 100 iterations)
        assert!(duration.as_secs() < 1);
    }
}
