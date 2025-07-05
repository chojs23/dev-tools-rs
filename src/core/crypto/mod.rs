pub mod asymmetric;
pub mod symmetric;

use anyhow::{anyhow, Result};
use asymmetric::{
    ecdsa::{ecdsa_sign, ecdsa_verify},
    generate_des_iv, generate_des_key, generate_triple_des_key,
    rsa::{
        generate_ecdsa_keypair, generate_rsa_keypair, rsa_decrypt, rsa_encrypt, rsa_sign,
        rsa_verify,
    },
};
use base64::{self, Engine};
use hex;
use serde::{Deserialize, Serialize};
use std::fmt;
use symmetric::{
    aes::AesKeySize,
    des::{des_decrypt, des_encrypt},
    generate_aes_iv, generate_aes_key,
    tdes::{triple_des_decrypt, triple_des_encrypt},
};

use crate::core::crypto::symmetric::aes::{aes_decrypt, aes_encrypt};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[allow(clippy::upper_case_acronyms)]
pub enum CryptoAlgorithm {
    AES,
    DES,
    TripleDES,
    RSA,
    ECDSA,
}

impl fmt::Display for CryptoAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CryptoAlgorithm::AES => write!(f, "AES"),
            CryptoAlgorithm::DES => write!(f, "DES"),
            CryptoAlgorithm::TripleDES => write!(f, "Triple DES"),
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
            CryptoAlgorithm::RSA,
            CryptoAlgorithm::ECDSA,
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
    pub key_size: Option<AesKeySize>, // Only for AES
    pub iv: Option<String>,           // Initialization Vector for CBC mode
    pub public_key: Option<String>,   // For asymmetric algorithms
    pub private_key: Option<String>,  // For asymmetric algorithms
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
            key_size: Some(AesKeySize::Aes128),
            iv: None,
            public_key: None,
            private_key: None,
            signature: None,
            encoding: OutputEncoding::default(),
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
        let key_size = self.input.key_size.unwrap_or(AesKeySize::Aes128);
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
                rsa_encrypt(&self.input.input_text, public_key)
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
                rsa_sign(&self.input.input_text, private_key)
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

    fn process_ecdsa(&self) -> Result<String> {
        match self.input.operation {
            CryptoOperation::Sign => {
                let private_key = self
                    .input
                    .private_key
                    .as_ref()
                    .ok_or_else(|| anyhow!("Private key required for ECDSA signing"))?;
                ecdsa_sign(&self.input.input_text, private_key)
            }
            CryptoOperation::Verify => {
                let public_key = self
                    .input
                    .public_key
                    .as_ref()
                    .ok_or_else(|| anyhow!("Public key required for ECDSA verification"))?;
                let signature = self
                    .input
                    .signature
                    .as_ref()
                    .ok_or_else(|| anyhow!("Signature required for ECDSA verification"))?;
                let is_valid = ecdsa_verify(&self.input.input_text, signature, public_key)?;
                Ok(format!("Signature valid: {is_valid}"))
            }
            _ => Err(anyhow!(
                "ECDSA only supports signing and verification operations"
            )),
        }
    }

    pub fn clear_output(&mut self) {
        self.output.clear();
        self.error = None;
    }

    pub fn generate_random_key(&mut self) -> Result<()> {
        let key = match self.input.algorithm {
            CryptoAlgorithm::AES => generate_aes_key(
                self.input
                    .key_size
                    .ok_or(anyhow!("Key size is required for AES"))?,
            ),
            CryptoAlgorithm::DES => generate_des_key(),
            CryptoAlgorithm::TripleDES => generate_triple_des_key(),
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
