use std::fmt;
use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow};

pub mod symmetric;
pub mod asymmetric;

use symmetric::*;
use asymmetric::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CryptoAlgorithm {
    AES,
    DES,
    TripleDES,
    RC4,
    RSA,
    ECDSA,
}

impl fmt::Display for CryptoAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CryptoAlgorithm::AES => write!(f, "AES"),
            CryptoAlgorithm::DES => write!(f, "DES"),
            CryptoAlgorithm::TripleDES => write!(f, "Triple DES"),
            CryptoAlgorithm::RC4 => write!(f, "RC4"),
            CryptoAlgorithm::RSA => write!(f, "RSA"),
            CryptoAlgorithm::ECDSA => write!(f, "ECDSA"),
        }
    }
}

impl CryptoAlgorithm {
    pub fn variants() -> &'static [CryptoAlgorithm] {
        &[
            CryptoAlgorithm::AES,
            CryptoAlgorithm::DES,
            CryptoAlgorithm::TripleDES,
            CryptoAlgorithm::RC4,
            CryptoAlgorithm::RSA,
            CryptoAlgorithm::ECDSA,
        ]
    }

    pub fn is_symmetric(&self) -> bool {
        matches!(self, 
            CryptoAlgorithm::AES | 
            CryptoAlgorithm::DES | 
            CryptoAlgorithm::TripleDES | 
            CryptoAlgorithm::RC4
        )
    }

    pub fn is_asymmetric(&self) -> bool {
        !self.is_symmetric()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoInput {
    pub algorithm: CryptoAlgorithm,
    pub operation: CryptoOperation,
    pub mode: Option<CipherMode>,
    pub input_text: String,
    pub key: String,
    pub iv: Option<String>, // Initialization Vector for CBC mode
    pub public_key: Option<String>, // For asymmetric algorithms
    pub private_key: Option<String>, // For asymmetric algorithms
    pub signature: Option<String>, // For verification
}

impl Default for CryptoInput {
    fn default() -> Self {
        Self {
            algorithm: CryptoAlgorithm::AES,
            operation: CryptoOperation::Encrypt,
            mode: Some(CipherMode::CBC),
            input_text: String::new(),
            key: String::new(),
            iv: None,
            public_key: None,
            private_key: None,
            signature: None,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CryptographyProcessor {
    pub input: CryptoInput,
    pub output: String,
    pub error: Option<String>,
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
            CryptoAlgorithm::RC4 => self.process_rc4(),
            CryptoAlgorithm::RSA => self.process_rsa(),
            CryptoAlgorithm::ECDSA => self.process_ecdsa(),
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
        match self.input.operation {
            CryptoOperation::Encrypt => aes_encrypt(&self.input.input_text, &self.input.key, mode, self.input.iv.as_deref()),
            CryptoOperation::Decrypt => aes_decrypt(&self.input.input_text, &self.input.key, mode, self.input.iv.as_deref()),
            _ => Err(anyhow!("Invalid operation for AES")),
        }
    }

    fn process_des(&self) -> Result<String> {
        let mode = self.input.mode.unwrap_or(CipherMode::CBC);
        match self.input.operation {
            CryptoOperation::Encrypt => des_encrypt(&self.input.input_text, &self.input.key, mode, self.input.iv.as_deref()),
            CryptoOperation::Decrypt => des_decrypt(&self.input.input_text, &self.input.key, mode, self.input.iv.as_deref()),
            _ => Err(anyhow!("Invalid operation for DES")),
        }
    }

    fn process_triple_des(&self) -> Result<String> {
        let mode = self.input.mode.unwrap_or(CipherMode::CBC);
        match self.input.operation {
            CryptoOperation::Encrypt => triple_des_encrypt(&self.input.input_text, &self.input.key, mode, self.input.iv.as_deref()),
            CryptoOperation::Decrypt => triple_des_decrypt(&self.input.input_text, &self.input.key, mode, self.input.iv.as_deref()),
            _ => Err(anyhow!("Invalid operation for Triple DES")),
        }
    }

    fn process_rc4(&self) -> Result<String> {
        match self.input.operation {
            CryptoOperation::Encrypt => rc4_encrypt(&self.input.input_text, &self.input.key),
            CryptoOperation::Decrypt => rc4_decrypt(&self.input.input_text, &self.input.key),
            _ => Err(anyhow!("Invalid operation for RC4")),
        }
    }

    fn process_rsa(&self) -> Result<String> {
        match self.input.operation {
            CryptoOperation::Encrypt => {
                let public_key = self.input.public_key.as_ref()
                    .ok_or_else(|| anyhow!("Public key required for RSA encryption"))?;
                rsa_encrypt(&self.input.input_text, public_key)
            }
            CryptoOperation::Decrypt => {
                let private_key = self.input.private_key.as_ref()
                    .ok_or_else(|| anyhow!("Private key required for RSA decryption"))?;
                rsa_decrypt(&self.input.input_text, private_key)
            }
            CryptoOperation::Sign => {
                let private_key = self.input.private_key.as_ref()
                    .ok_or_else(|| anyhow!("Private key required for RSA signing"))?;
                rsa_sign(&self.input.input_text, private_key)
            }
            CryptoOperation::Verify => {
                let public_key = self.input.public_key.as_ref()
                    .ok_or_else(|| anyhow!("Public key required for RSA verification"))?;
                let signature = self.input.signature.as_ref()
                    .ok_or_else(|| anyhow!("Signature required for RSA verification"))?;
                let is_valid = rsa_verify(&self.input.input_text, signature, public_key)?;
                Ok(format!("Signature valid: {}", is_valid))
            }
        }
    }

    fn process_ecdsa(&self) -> Result<String> {
        match self.input.operation {
            CryptoOperation::Sign => {
                let private_key = self.input.private_key.as_ref()
                    .ok_or_else(|| anyhow!("Private key required for ECDSA signing"))?;
                ecdsa_sign(&self.input.input_text, private_key)
            }
            CryptoOperation::Verify => {
                let public_key = self.input.public_key.as_ref()
                    .ok_or_else(|| anyhow!("Public key required for ECDSA verification"))?;
                let signature = self.input.signature.as_ref()
                    .ok_or_else(|| anyhow!("Signature required for ECDSA verification"))?;
                let is_valid = ecdsa_verify(&self.input.input_text, signature, public_key)?;
                Ok(format!("Signature valid: {}", is_valid))
            }
            _ => Err(anyhow!("ECDSA only supports signing and verification operations")),
        }
    }

    pub fn clear_output(&mut self) {
        self.output.clear();
        self.error = None;
    }

    pub fn generate_random_key(&mut self) -> Result<()> {
        let key = match self.input.algorithm {
            CryptoAlgorithm::AES => generate_aes_key()?,
            CryptoAlgorithm::DES => generate_des_key()?,
            CryptoAlgorithm::TripleDES => generate_triple_des_key()?,
            CryptoAlgorithm::RC4 => generate_rc4_key()?,
            CryptoAlgorithm::RSA => {
                let (public_key, private_key) = generate_rsa_keypair()?;
                self.input.public_key = Some(public_key);
                self.input.private_key = Some(private_key.clone());
                return Ok(());
            }
            CryptoAlgorithm::ECDSA => {
                let (public_key, private_key) = generate_ecdsa_keypair()?;
                self.input.public_key = Some(public_key);
                self.input.private_key = Some(private_key.clone());
                return Ok(());
            }
        };
        
        self.input.key = key;
        Ok(())
    }

    pub fn generate_random_iv(&mut self) -> Result<()> {
        if self.input.mode == Some(CipherMode::CBC) {
            let iv = match self.input.algorithm {
                CryptoAlgorithm::AES => generate_aes_iv()?,
                CryptoAlgorithm::DES => generate_des_iv()?,
                CryptoAlgorithm::TripleDES => generate_des_iv()?,
                _ => return Err(anyhow!("IV not required for this algorithm")),
            };
            self.input.iv = Some(iv);
        }
        Ok(())
    }
}

