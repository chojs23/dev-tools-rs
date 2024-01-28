use anyhow::Result;

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

#[derive(Debug, Clone)]
pub struct JwtEncoderDecoder {
    pub encoded: String,
    pub decoded: String,

    pub algorithm: Algorithm,
    pub secret: Option<String>,
    pub public_key: Option<String>,
    pub private_key: Option<String>,
}

impl Default for JwtEncoderDecoder {
    fn default() -> Self {
        Self {
            encoded: String::new(),
            decoded: String::new(),
            algorithm: Algorithm::default(),
            secret: None,
            public_key: None,
            private_key: None,
        }
    }
}

impl JwtEncoderDecoder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn encode(&mut self) -> Result<()> {
        let token = match self.algorithm {
            Algorithm::HS256 => jsonwebtoken::encode(
                &jsonwebtoken::Header::new(self.algorithm.clone().into()),
                &self.decoded,
                &jsonwebtoken::EncodingKey::from_secret(
                    self.secret.as_ref().unwrap_or(&String::new()).as_bytes(),
                ),
            )?,
            Algorithm::RS256 => jsonwebtoken::encode(
                &jsonwebtoken::Header::new(self.algorithm.clone().into()),
                &self.decoded,
                &jsonwebtoken::EncodingKey::from_rsa_pem(
                    self.private_key
                        .as_ref()
                        .unwrap_or(&String::new())
                        .as_bytes(),
                )?,
            )?,
            _ => panic!("Unsupported algorithm"),
        };

        self.encoded = token;
        Ok(())
    }

    pub fn decode(&mut self) -> Result<()> {
        let token = match self.algorithm {
            Algorithm::HS256 => jsonwebtoken::decode::<String>(
                &self.encoded,
                &jsonwebtoken::DecodingKey::from_secret(
                    self.secret.as_ref().unwrap_or(&String::new()).as_bytes(),
                ),
                &jsonwebtoken::Validation::new(self.algorithm.clone().into()),
            )?,
            Algorithm::RS256 => jsonwebtoken::decode::<String>(
                &self.encoded,
                &jsonwebtoken::DecodingKey::from_rsa_pem(
                    self.public_key
                        .as_ref()
                        .unwrap_or(&String::new())
                        .as_bytes(),
                )?,
                &jsonwebtoken::Validation::new(self.algorithm.clone().into()),
            )?,
            _ => panic!("Unsupported algorithm"),
        };

        self.decoded = token.claims;
        Ok(())
    }
}
