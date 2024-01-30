use anyhow::{bail, Result};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum Algorithm {
    /// HMAC using SHA-256
    #[default]
    HS256,
    /// HMAC using SHA-384
    HS384,
    /// HMAC using SHA-512
    HS512,

    /// RSASSA-PKCS1-v1_5 using SHA-256
    RS256,
    /// RSASSA-PKCS1-v1_5 using SHA-384
    RS384,
    /// RSASSA-PKCS1-v1_5 using SHA-512
    RS512,
}

impl From<Algorithm> for jsonwebtoken::Algorithm {
    fn from(algorithm: Algorithm) -> Self {
        match algorithm {
            Algorithm::HS256 => jsonwebtoken::Algorithm::HS256,
            Algorithm::HS384 => jsonwebtoken::Algorithm::HS384,
            Algorithm::HS512 => jsonwebtoken::Algorithm::HS512,
            Algorithm::RS256 => jsonwebtoken::Algorithm::RS256,
            Algorithm::RS384 => jsonwebtoken::Algorithm::RS384,
            Algorithm::RS512 => jsonwebtoken::Algorithm::RS512,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct JwtEncoderDecoder {
    pub encoded: String,
    pub decoded: String,
    pub algorithm: Algorithm,
    pub secret: Option<String>,
    pub public_key: Option<String>,
    pub private_key: Option<String>,
}

impl JwtEncoderDecoder {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn encode(&mut self) -> Result<()> {
        let token = match self.algorithm {
            Algorithm::HS256 | Algorithm::HS384 | Algorithm::HS512 => self.encode_by_hmac()?,
            Algorithm::RS256 | Algorithm::RS384 | Algorithm::RS512 => self.encode_by_rsa()?,
        };

        self.encoded = token;
        Ok(())
    }

    pub fn decode(&mut self) -> Result<()> {
        let claims = match self.algorithm {
            Algorithm::HS256 | Algorithm::HS384 | Algorithm::HS512 => self.decode_by_hmac()?,
            Algorithm::RS256 | Algorithm::RS384 | Algorithm::RS512 => self.decode_by_rsa()?,
        };

        self.decoded = claims;
        Ok(())
    }

    fn encode_by_hmac(&mut self) -> Result<String> {
        if self.secret.is_none() {
            bail!("Secret is required");
        }

        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::new(self.algorithm.clone().into()),
            &self.decoded,
            &jsonwebtoken::EncodingKey::from_secret(
                self.secret.as_ref().unwrap_or(&String::new()).as_bytes(),
            ),
        )?;

        Ok(token)
    }

    fn encode_by_rsa(&mut self) -> Result<String> {
        if self.private_key.is_none() {
            bail!("Private key is required");
        }

        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::new(self.algorithm.clone().into()),
            &self.decoded,
            &jsonwebtoken::EncodingKey::from_rsa_pem(
                self.private_key
                    .as_ref()
                    .unwrap_or(&String::new())
                    .as_bytes(),
            )?,
        )?;

        Ok(token)
    }

    fn decode_by_hmac(&mut self) -> Result<String> {
        if self.secret.is_none() {
            bail!("Secret is required");
        }

        let token_data = jsonwebtoken::decode::<String>(
            &self.encoded,
            &jsonwebtoken::DecodingKey::from_secret(
                self.secret.as_ref().unwrap_or(&String::new()).as_bytes(),
            ),
            &jsonwebtoken::Validation::new(self.algorithm.clone().into()),
        )?;

        Ok(token_data.claims)
    }

    fn decode_by_rsa(&mut self) -> Result<String> {
        if self.public_key.is_none() {
            bail!("Public key is required");
        }

        let token_data = jsonwebtoken::decode::<String>(
            &self.encoded,
            &jsonwebtoken::DecodingKey::from_rsa_pem(
                self.public_key
                    .as_ref()
                    .unwrap_or(&String::new())
                    .as_bytes(),
            )?,
            &jsonwebtoken::Validation::new(self.algorithm.clone().into()),
        )?;

        Ok(token_data.claims)
    }
}
