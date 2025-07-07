pub mod asymmetric;
pub mod symmetric;

use anyhow::{anyhow, Result};
use asymmetric::{
    generate_des_iv, generate_des_key, generate_triple_des_key,
    rsa::{generate_rsa_keypair, rsa_decrypt, rsa_encrypt, rsa_sign, rsa_verify},
};
use base64::{self, Engine};
use hex;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Receiver;
use symmetric::{
    aes::AesKeySize,
    des::{des_decrypt, des_encrypt},
    generate_aes_iv, generate_aes_key,
    tdes::{triple_des_decrypt, triple_des_encrypt},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RsaKeySize {
    Rsa512,
    Rsa1024,
    Rsa2048,
    Rsa3072,
    Rsa4096,
}

impl fmt::Display for RsaKeySize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RsaKeySize::Rsa512 => write!(f, "512 bits"),
            RsaKeySize::Rsa1024 => write!(f, "1024 bits"),
            RsaKeySize::Rsa2048 => write!(f, "2048 bits"),
            RsaKeySize::Rsa3072 => write!(f, "3072 bits"),
            RsaKeySize::Rsa4096 => write!(f, "4096 bits"),
        }
    }
}

impl RsaKeySize {
    pub fn variants() -> &'static [RsaKeySize] {
        &[
            RsaKeySize::Rsa512,
            RsaKeySize::Rsa1024,
            RsaKeySize::Rsa2048,
            RsaKeySize::Rsa3072,
            RsaKeySize::Rsa4096,
        ]
    }

    pub fn to_bits(&self) -> usize {
        match self {
            RsaKeySize::Rsa512 => 512,
            RsaKeySize::Rsa1024 => 1024,
            RsaKeySize::Rsa2048 => 2048,
            RsaKeySize::Rsa3072 => 3072,
            RsaKeySize::Rsa4096 => 4096,
        }
    }
}

impl Default for RsaKeySize {
    fn default() -> Self {
        RsaKeySize::Rsa2048
    }
}

use crate::core::crypto::symmetric::aes::{aes_decrypt, aes_encrypt};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[allow(clippy::upper_case_acronyms)]
pub enum CryptoAlgorithm {
    AES,
    DES,
    TripleDES,
    RSA,
}

impl fmt::Display for CryptoAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CryptoAlgorithm::AES => write!(f, "AES"),
            CryptoAlgorithm::DES => write!(f, "DES"),
            CryptoAlgorithm::TripleDES => write!(f, "Triple DES"),
            CryptoAlgorithm::RSA => write!(f, "RSA"),
        }
    }
}

impl CryptoAlgorithm {
    pub fn variants() -> &'static [CryptoAlgorithm] {
        &[
            CryptoAlgorithm::AES,
            CryptoAlgorithm::DES,
            CryptoAlgorithm::TripleDES,
            CryptoAlgorithm::RSA,
        ]
    }

    pub fn is_symmetric(&self) -> bool {
        matches!(
            self,
            CryptoAlgorithm::AES | CryptoAlgorithm::DES | CryptoAlgorithm::TripleDES
        )
    }

    pub fn is_asymmetric(&self) -> bool {
        !self.is_symmetric()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[allow(clippy::upper_case_acronyms)]
pub enum CipherMode {
    ECB,
    CBC,
}

impl fmt::Display for CipherMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CipherMode::ECB => write!(f, "ECB"),
            CipherMode::CBC => write!(f, "CBC"),
        }
    }
}

impl CipherMode {
    pub fn variants() -> &'static [CipherMode] {
        &[CipherMode::ECB, CipherMode::CBC]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CryptoOperation {
    Encrypt,
    Decrypt,
    Sign,
    Verify,
}

impl fmt::Display for CryptoOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CryptoOperation::Encrypt => write!(f, "Encrypt"),
            CryptoOperation::Decrypt => write!(f, "Decrypt"),
            CryptoOperation::Sign => write!(f, "Sign"),
            CryptoOperation::Verify => write!(f, "Verify"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OutputEncoding {
    Hex,
    Base64,
}

impl fmt::Display for OutputEncoding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OutputEncoding::Hex => write!(f, "Hex"),
            OutputEncoding::Base64 => write!(f, "Base64"),
        }
    }
}

impl OutputEncoding {
    pub fn variants() -> &'static [OutputEncoding] {
        &[OutputEncoding::Hex, OutputEncoding::Base64]
    }
}

impl Default for OutputEncoding {
    fn default() -> Self {
        OutputEncoding::Hex
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoInput {
    pub algorithm: CryptoAlgorithm,
    pub operation: CryptoOperation,
    pub mode: Option<CipherMode>,
    pub input_text: String,
    pub key: String,
    pub aes_key_size: Option<AesKeySize>, // Key size for AES (128, 192, or 256 bits)
    pub iv: Option<String>,               // Initialization Vector for CBC mode
    pub public_key: Option<String>,       // For asymmetric algorithms
    pub private_key: Option<String>,      // For asymmetric algorithms
    pub rsa_key_size: Option<RsaKeySize>, // Key size for RSA (512, 1024, 2048, 3072, 4096 bits)
    pub signature: Option<String>,
    pub encoding: OutputEncoding,
}

impl Default for CryptoInput {
    fn default() -> Self {
        Self {
            algorithm: CryptoAlgorithm::AES,
            operation: CryptoOperation::Encrypt,
            mode: Some(CipherMode::CBC),
            input_text: String::new(),
            key: String::new(),
            aes_key_size: Some(AesKeySize::Aes128),
            iv: None,
            public_key: None,
            private_key: None,
            rsa_key_size: Some(RsaKeySize::Rsa2048),
            signature: None,
            encoding: OutputEncoding::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyGenerationState {
    Idle,
    Generating,
    Completed,
    Failed(String),
}

impl Default for KeyGenerationState {
    fn default() -> Self {
        KeyGenerationState::Idle
    }
}

#[derive(Debug, Clone)]
pub struct KeyGenerationResult {
    pub public_key: String,
    pub private_key: String,
}

#[derive(Debug)]
pub struct CryptographyProcessor {
    pub input: CryptoInput,
    pub output: String,
    pub error: Option<String>,
    pub key_generation_state: KeyGenerationState,
    pub key_generation_receiver: Option<Arc<Mutex<Receiver<Result<KeyGenerationResult, String>>>>>,
}

impl Default for CryptographyProcessor {
    fn default() -> Self {
        Self {
            input: CryptoInput::default(),
            output: String::new(),
            error: None,
            key_generation_state: KeyGenerationState::default(),
            key_generation_receiver: None,
        }
    }
}

impl Clone for CryptographyProcessor {
    fn clone(&self) -> Self {
        Self {
            input: self.input.clone(),
            output: self.output.clone(),
            error: self.error.clone(),
            key_generation_state: self.key_generation_state.clone(),
            key_generation_receiver: None, // Can't clone receiver
        }
    }
}

impl CryptographyProcessor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn process(&mut self) -> Result<String> {
        self.error = None;

        let result = match self.input.algorithm {
            CryptoAlgorithm::AES => self.process_aes(),
            CryptoAlgorithm::DES => self.process_des(),
            CryptoAlgorithm::TripleDES => self.process_triple_des(),
            CryptoAlgorithm::RSA => self.process_rsa(),
        };

        match result {
            Ok(output) => {
                self.output = output.clone();
                Ok(output)
            }
            Err(e) => {
                let error_msg = format!("Cryptography error: {}", e);
                self.error = Some(error_msg.clone());
                self.output.clear();
                Err(anyhow!(error_msg))
            }
        }
    }

    fn process_aes(&self) -> Result<String> {
        let mode = self.input.mode.unwrap_or(CipherMode::CBC);
        let key_size = self.input.aes_key_size.unwrap_or(AesKeySize::Aes128);
        match self.input.operation {
            CryptoOperation::Encrypt => {
                let encrypted_bytes = aes_encrypt(
                    &self.input.input_text,
                    &self.input.key,
                    key_size,
                    mode,
                    self.input.iv.as_deref(),
                )?;

                // Handle encoding in CryptographyProcessor
                match self.input.encoding {
                    OutputEncoding::Hex => Ok(hex::encode(encrypted_bytes)),
                    OutputEncoding::Base64 => {
                        Ok(base64::engine::general_purpose::STANDARD.encode(encrypted_bytes))
                    }
                }
            }
            CryptoOperation::Decrypt => {
                let decrypted_bytes = aes_decrypt(
                    &self.input.input_text,
                    &self.input.key,
                    key_size,
                    mode,
                    self.input.iv.as_deref(),
                )?;

                String::from_utf8(decrypted_bytes)
                    .map_err(|e| anyhow!("Invalid UTF-8 in decrypted text: {}", e))
            }
            _ => Err(anyhow!("Invalid operation for AES")),
        }
    }

    fn process_des(&self) -> Result<String> {
        let mode = self.input.mode.unwrap_or(CipherMode::CBC);
        match self.input.operation {
            CryptoOperation::Encrypt => {
                let encrypted_bytes = des_encrypt(
                    &self.input.input_text,
                    &self.input.key,
                    mode,
                    self.input.iv.as_deref(),
                )?;

                match self.input.encoding {
                    OutputEncoding::Hex => Ok(hex::encode(encrypted_bytes)),
                    OutputEncoding::Base64 => {
                        Ok(base64::engine::general_purpose::STANDARD.encode(encrypted_bytes))
                    }
                }
            }
            CryptoOperation::Decrypt => des_decrypt(
                &self.input.input_text,
                &self.input.key,
                mode,
                self.input.iv.as_deref(),
            ),
            _ => Err(anyhow!("Invalid operation for DES")),
        }
    }

    fn process_triple_des(&self) -> Result<String> {
        let mode = self.input.mode.unwrap_or(CipherMode::CBC);
        match self.input.operation {
            CryptoOperation::Encrypt => {
                let encrypted_bytes = triple_des_encrypt(
                    &self.input.input_text,
                    &self.input.key,
                    mode,
                    self.input.iv.as_deref(),
                )?;

                match self.input.encoding {
                    OutputEncoding::Hex => Ok(hex::encode(encrypted_bytes)),
                    OutputEncoding::Base64 => {
                        Ok(base64::engine::general_purpose::STANDARD.encode(encrypted_bytes))
                    }
                }
            }
            CryptoOperation::Decrypt => triple_des_decrypt(
                &self.input.input_text,
                &self.input.key,
                mode,
                self.input.iv.as_deref(),
            ),
            _ => Err(anyhow!("Invalid operation for Triple DES")),
        }
    }

    fn process_rsa(&self) -> Result<String> {
        match self.input.operation {
            CryptoOperation::Encrypt => {
                let public_key = self
                    .input
                    .public_key
                    .as_ref()
                    .ok_or_else(|| anyhow!("Public key required for RSA encryption"))?;
                let encrypted_bytes = rsa_encrypt(&self.input.input_text, public_key)?;

                match self.input.encoding {
                    OutputEncoding::Hex => Ok(hex::encode(encrypted_bytes)),
                    OutputEncoding::Base64 => {
                        Ok(base64::engine::general_purpose::STANDARD.encode(encrypted_bytes))
                    }
                }
            }
            CryptoOperation::Decrypt => {
                let private_key = self
                    .input
                    .private_key
                    .as_ref()
                    .ok_or_else(|| anyhow!("Private key required for RSA decryption"))?;
                rsa_decrypt(&self.input.input_text, private_key)
            }
            CryptoOperation::Sign => {
                let private_key = self
                    .input
                    .private_key
                    .as_ref()
                    .ok_or_else(|| anyhow!("Private key required for RSA signing"))?;
                let signed_bytes = rsa_sign(&self.input.input_text, private_key)?;

                match self.input.encoding {
                    OutputEncoding::Hex => Ok(hex::encode(signed_bytes)),
                    OutputEncoding::Base64 => {
                        Ok(base64::engine::general_purpose::STANDARD.encode(signed_bytes))
                    }
                }
            }
            CryptoOperation::Verify => {
                let public_key = self
                    .input
                    .public_key
                    .as_ref()
                    .ok_or_else(|| anyhow!("Public key required for RSA verification"))?;
                let signature = self
                    .input
                    .signature
                    .as_ref()
                    .ok_or_else(|| anyhow!("Signature required for RSA verification"))?;
                let is_valid = rsa_verify(&self.input.input_text, signature, public_key)?;
                Ok(format!("Signature valid: {is_valid}"))
            }
        }
    }
    pub fn clear_output(&mut self) {
        self.output.clear();
        self.error = None;
    }

    pub fn generate_random_key(&mut self) -> Result<()> {
        match self.input.algorithm {
            CryptoAlgorithm::AES => {
                let key = generate_aes_key(
                    self.input
                        .aes_key_size
                        .ok_or(anyhow!("Key size is required for AES"))?,
                );
                self.input.key = key;
                Ok(())
            }
            CryptoAlgorithm::DES => {
                let key = generate_des_key();
                self.input.key = key;
                Ok(())
            }
            CryptoAlgorithm::TripleDES => {
                let key = generate_triple_des_key();
                self.input.key = key;
                Ok(())
            }
            CryptoAlgorithm::RSA => {
                self.start_async_key_generation()
            }
        }
    }

    pub fn start_async_key_generation(&mut self) -> Result<()> {
        // If already generating, don't start another thread
        if self.key_generation_state == KeyGenerationState::Generating {
            return Ok(());
        }

        let key_size = self.input.rsa_key_size.unwrap_or(RsaKeySize::Rsa2048);
        let (sender, receiver) = std::sync::mpsc::channel();
        
        self.key_generation_state = KeyGenerationState::Generating;
        self.key_generation_receiver = Some(Arc::new(Mutex::new(receiver)));

        // Spawn thread for key generation
        std::thread::spawn(move || {
            let result = generate_rsa_keypair(key_size.to_bits())
                .map(|(public_key, private_key)| KeyGenerationResult {
                    public_key,
                    private_key,
                })
                .map_err(|e| e.to_string());

            let _ = sender.send(result);
        });

        Ok(())
    }

    pub fn check_key_generation_progress(&mut self) {
        let should_clear_receiver = if let Some(receiver_arc) = &self.key_generation_receiver {
            if let Ok(receiver) = receiver_arc.try_lock() {
                if let Ok(result) = receiver.try_recv() {
                    match result {
                        Ok(keys) => {
                            self.input.public_key = Some(keys.public_key);
                            self.input.private_key = Some(keys.private_key);
                            self.key_generation_state = KeyGenerationState::Completed;
                        }
                        Err(error) => {
                            self.key_generation_state = KeyGenerationState::Failed(error);
                        }
                    }
                    true
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        };
        
        if should_clear_receiver {
            self.key_generation_receiver = None;
        }
    }

    pub fn is_key_generation_complete(&self) -> bool {
        matches!(
            self.key_generation_state,
            KeyGenerationState::Completed | KeyGenerationState::Failed(_)
        )
    }

    pub fn is_key_generation_in_progress(&self) -> bool {
        matches!(self.key_generation_state, KeyGenerationState::Generating)
    }

    pub fn reset_key_generation_state(&mut self) {
        self.key_generation_state = KeyGenerationState::Idle;
        self.key_generation_receiver = None;
    }

    pub fn generate_random_iv(&mut self) -> Result<()> {
        if self.input.mode == Some(CipherMode::CBC) {
            let iv = match self.input.algorithm {
                CryptoAlgorithm::AES => generate_aes_iv(),
                CryptoAlgorithm::DES => generate_des_iv(),
                CryptoAlgorithm::TripleDES => generate_des_iv(),
                _ => return Err(anyhow!("IV not required for this algorithm")),
            };
            self.input.iv = Some(iv);
        }
        Ok(())
    }
}
