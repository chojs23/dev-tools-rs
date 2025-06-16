use anyhow::{Error, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegexMatch {
    pub start: usize,
    pub end: usize,
    pub text: String,
    pub groups: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegexMatchResult {
    pub pattern: String,
    pub text: String,
    pub matches: Vec<RegexMatch>,
    pub is_valid: bool,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegexProcessor {
    pub pattern: String,
    pub text: String,
    pub case_insensitive: bool,
    pub multiline: bool,
    pub dot_matches_newline: bool,
    pub result: Option<RegexMatchResult>,
}

impl Default for RegexProcessor {
    fn default() -> Self {
        Self {
            pattern: "mid(?<postfix>[a-zA-Z]+)".to_string(),
            text: "aurora midsummer midnight earth".to_string(),
            case_insensitive: false,
            multiline: false,
            dot_matches_newline: false,
            result: None,
        }
    }
}

impl RegexProcessor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn process(&mut self) -> Result<()> {
        if self.pattern.is_empty() {
            self.result = Some(RegexMatchResult {
                pattern: self.pattern.clone(),
                text: self.text.clone(),
                matches: Vec::new(),
                is_valid: true,
                error_message: None,
            });
            return Ok(());
        }

        let regex_result = self.build_regex();

        match regex_result {
            Ok(regex) => {
                let matches = self.find_matches(&regex);
                self.result = Some(RegexMatchResult {
                    pattern: self.pattern.clone(),
                    text: self.text.clone(),
                    matches,
                    is_valid: true,
                    error_message: None,
                });
                Ok(())
            }
            Err(e) => {
                self.result = Some(RegexMatchResult {
                    pattern: self.pattern.clone(),
                    text: self.text.clone(),
                    matches: Vec::new(),
                    is_valid: false,
                    error_message: Some(e.to_string()),
                });
                Err(e)
            }
        }
    }

    fn build_regex(&self) -> Result<Regex> {
        let mut builder = regex::RegexBuilder::new(&self.pattern);
        builder.case_insensitive(self.case_insensitive);
        builder.multi_line(self.multiline);
        builder.dot_matches_new_line(self.dot_matches_newline);

        builder.build().map_err(|e| Error::msg(e.to_string()))
    }

    fn find_matches(&self, regex: &Regex) -> Vec<RegexMatch> {
        let mut matches = Vec::new();

        for mat in regex.find_iter(&self.text) {
            let mut groups = Vec::new();

            // Find capture groups for this match
            if let Some(captures) = regex.captures(&self.text[mat.start()..]) {
                for (i, group) in captures.iter().enumerate() {
                    if i == 0 {
                        continue; // Skip the full match
                    }
                    groups.push(group.map_or_else(|| String::new(), |m| m.as_str().to_string()));
                }
            }

            matches.push(RegexMatch {
                start: mat.start(),
                end: mat.end(),
                text: mat.as_str().to_string(),
                groups,
            });
        }

        matches
    }

    pub fn clear(&mut self) {
        self.pattern.clear();
        self.text.clear();
        self.result = None;
    }
}
