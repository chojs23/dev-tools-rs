use anyhow::Result;
use base32::Alphabet;
use base64::{engine::general_purpose, Engine};
use url::form_urlencoded;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum EncodingType {
    #[default]
    Base64,
    Base64Url,
    Base64Mime,
    Base32,
    UrlEncoding,
}

#[derive(Debug, Clone, Default)]
pub struct EncodingProcessor {
    pub decoded_text: String,
    pub encoded_text: String,
    pub encoding_type: EncodingType,
    pub handle_line_breaks: bool,
    pub live_conversion: bool,
}

impl EncodingProcessor {
    pub fn new() -> Self {
        Self {
            decoded_text: String::new(),
            encoded_text: String::new(),
            encoding_type: EncodingType::Base64,
            handle_line_breaks: false,
            live_conversion: false,
        }
    }

    pub fn clear(&mut self) {
        self.decoded_text.clear();
        self.encoded_text.clear();
        self.encoding_type = EncodingType::Base64;
        self.handle_line_breaks = false;
    }

    pub fn encode(&mut self) -> Result<()> {
        let input = if self.handle_line_breaks {
            self.decoded_text.replace('\n', "\\n").replace('\r', "\\r")
        } else {
            self.decoded_text.clone()
        };

        self.encoded_text = match self.encoding_type {
            EncodingType::Base64 => general_purpose::STANDARD.encode(&input),
            EncodingType::Base64Url => general_purpose::URL_SAFE.encode(&input),
            EncodingType::Base64Mime => {
                let encoded = general_purpose::STANDARD.encode(&input);
                // MIME base64 adds line breaks every 76 characters
                encoded
                    .chars()
                    .collect::<Vec<char>>()
                    .chunks(76)
                    .map(|chunk| chunk.iter().collect::<String>())
                    .collect::<Vec<String>>()
                    .join("\r\n")
            }
            EncodingType::Base32 => {
                base32::encode(Alphabet::Rfc4648 { padding: true }, input.as_bytes())
            }
            EncodingType::UrlEncoding => form_urlencoded::Serializer::new(String::new())
                .append_pair("", &input)
                .finish()
                .trim_start_matches('=')
                .to_string(),
        };

        Ok(())
    }

    pub fn decode(&mut self) -> Result<()> {
        let decoded_bytes = match self.encoding_type {
            EncodingType::Base64 => general_purpose::STANDARD.decode(&self.encoded_text)?,
            EncodingType::Base64Url => general_purpose::URL_SAFE.decode(&self.encoded_text)?,
            EncodingType::Base64Mime => {
                let clean_input = self.encoded_text.replace(['\r', '\n', ' '], "");
                general_purpose::STANDARD.decode(&clean_input)?
            }
            EncodingType::Base32 => {
                base32::decode(Alphabet::Rfc4648 { padding: true }, &self.encoded_text)
                    .ok_or_else(|| anyhow::anyhow!("Invalid Base32 input"))?
            }
            EncodingType::UrlEncoding => {
                let decoded =
                    form_urlencoded::parse(format!("dummy={}", self.encoded_text).as_bytes())
                        .find(|(key, _)| key == "dummy")
                        .map(|(_, value)| value.into_owned())
                        .ok_or_else(|| anyhow::anyhow!("Invalid URL encoding"))?;
                decoded.into_bytes()
            }
        };

        let mut decoded_string = String::from_utf8(decoded_bytes)
            .map_err(|_| anyhow::anyhow!("Decoded data is not valid UTF-8"))?;

        if self.handle_line_breaks {
            decoded_string = decoded_string.replace("\\n", "\n").replace("\\r", "\r");
        }

        self.decoded_text = decoded_string;
        Ok(())
    }
}
