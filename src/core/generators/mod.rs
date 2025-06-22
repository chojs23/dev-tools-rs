use rand::{rng, Rng};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum GeneratorType {
    Uuid,
    Ulid,
    NanoId,
    Password,
}

impl Default for GeneratorType {
    fn default() -> Self {
        Self::Uuid
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GeneratorProcessor {
    pub generator_type: GeneratorType,
    pub output: String,
    pub generated_count: usize,
    // Password generator options
    pub password_length: usize,
    pub include_uppercase: bool,
    pub include_lowercase: bool,
    pub include_numbers: bool,
    pub include_symbols: bool,
    pub exclude_ambiguous: bool,
    // Nano ID options
    pub nano_id_length: usize,
    pub nano_id_alphabet: String,
    pub use_custom_alphabet: bool,
}

impl Default for GeneratorProcessor {
    fn default() -> Self {
        Self {
            generator_type: GeneratorType::default(),
            output: String::new(),
            generated_count: 1,
            password_length: 12,
            include_uppercase: true,
            include_lowercase: true,
            include_numbers: true,
            include_symbols: false,
            exclude_ambiguous: false,
            nano_id_length: 21,
            nano_id_alphabet: String::from(
                "_-0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ",
            ),
            use_custom_alphabet: false,
        }
    }
}

impl GeneratorProcessor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn generate(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut results = Vec::new();

        for _ in 0..self.generated_count {
            let result = match self.generator_type {
                GeneratorType::Uuid => self.generate_uuid(),
                GeneratorType::Ulid => self.generate_ulid(),
                GeneratorType::NanoId => self.generate_nano_id()?,
                GeneratorType::Password => self.generate_password()?,
            };
            results.push(result);
        }

        self.output = results.join("\n");
        Ok(())
    }

    fn generate_uuid(&self) -> String {
        Uuid::new_v4().to_string()
    }

    fn generate_ulid(&self) -> String {
        ulid::Ulid::new().to_string()
    }

    fn generate_nano_id(&self) -> Result<String, Box<dyn std::error::Error>> {
        let length = if self.nano_id_length == 0 {
            Self::default().nano_id_length
        } else {
            self.nano_id_length
        };
        if self.use_custom_alphabet && !self.nano_id_alphabet.is_empty() {
            let alphabet: Vec<char> = self.nano_id_alphabet.chars().collect();
            Ok(nanoid::nanoid!(length, &alphabet))
        } else {
            Ok(nanoid::nanoid!(length))
        }
    }

    fn generate_password(&self) -> Result<String, Box<dyn std::error::Error>> {
        if self.password_length == 0 {
            return Err("Password length must be greater than 0".into());
        }

        let mut charset = String::new();

        if self.include_lowercase {
            charset.push_str("abcdefghijklmnopqrstuvwxyz");
        }
        if self.include_uppercase {
            charset.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        }
        if self.include_numbers {
            charset.push_str("0123456789");
        }
        if self.include_symbols {
            charset.push_str("!@#$%^&*()_+-=[]{}|;:,.<>?");
        }

        if self.exclude_ambiguous {
            // Remove ambiguous characters
            let ambiguous_chars = "0O1lI";
            charset = charset
                .chars()
                .filter(|c| !ambiguous_chars.contains(*c))
                .collect();
        }

        if charset.is_empty() {
            return Err("At least one character type must be selected".into());
        }

        let charset_chars: Vec<char> = charset.chars().collect();
        let mut rng = rng();

        let password: String = (0..self.password_length)
            .map(|_| charset_chars[rng.random_range(0..charset_chars.len())])
            .collect();

        Ok(password)
    }

    pub fn clear(&mut self) {
        self.output.clear();
    }

    pub fn copy_to_clipboard(&self) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            use arboard::Clipboard;
            let mut clipboard = Clipboard::new()?;
            clipboard.set_text(&self.output)?;
        }
        Ok(())
    }
}
