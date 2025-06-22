use chrono::{DateTime, Local, NaiveDateTime, TimeZone, Utc};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

static TIMESTAMP_CACHE: Lazy<Arc<RwLock<i64>>> = Lazy::new(|| {
    let cache = Arc::new(RwLock::new(Utc::now().timestamp()));
    let cache_clone = Arc::clone(&cache);

    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(100));
        let now = Utc::now().timestamp();
        if let Ok(mut timestamp) = cache_clone.write() {
            *timestamp = now;
        }
    });

    cache
});

static COMMON_FORMATS: [&str; 9] = [
    "%Y-%m-%d %H:%M:%S",
    "%Y-%m-%dT%H:%M:%S",
    "%Y-%m-%dT%H:%M:%SZ",
    "%Y-%m-%dT%H:%M:%S%.3fZ",
    "%Y-%m-%d",
    "%m/%d/%Y",
    "%d/%m/%Y",
    "%m-%d-%Y",
    "%d-%m-%Y",
];

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DateTimeFormat {
    Iso8601,
    Rfc2822,
    Rfc3339,
    Custom(String),
}

impl fmt::Display for DateTimeFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DateTimeFormat::Iso8601 => write!(f, "ISO 8601"),
            DateTimeFormat::Rfc2822 => write!(f, "RFC 2822"),
            DateTimeFormat::Rfc3339 => write!(f, "RFC 3339"),
            DateTimeFormat::Custom(format) => write!(f, "Custom: {}", format),
        }
    }
}

impl DateTimeFormat {
    pub fn all_formats() -> Vec<DateTimeFormat> {
        vec![
            DateTimeFormat::Iso8601,
            DateTimeFormat::Rfc2822,
            DateTimeFormat::Rfc3339,
            DateTimeFormat::Custom("%Y-%m-%d %H:%M:%S".to_string()),
        ]
    }

    pub fn format_string(&self) -> String {
        match self {
            DateTimeFormat::Iso8601 => "%Y-%m-%dT%H:%M:%S%.3fZ".to_string(),
            DateTimeFormat::Rfc2822 => "%a, %d %b %Y %H:%M:%S %z".to_string(),
            DateTimeFormat::Rfc3339 => "%Y-%m-%dT%H:%M:%S%.3f%:z".to_string(),
            DateTimeFormat::Custom(format) => format.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateTimeProcessor {
    pub timestamp_input: String,
    pub formatted_input: String,
    pub custom_format: String,
    pub selected_format: DateTimeFormat,
    pub timestamp_result: String,
    pub formatted_result: String,
    pub current_timestamp: i64,
    pub error_message: String,
}

impl Default for DateTimeProcessor {
    fn default() -> Self {
        let now = Utc::now().timestamp();
        Self {
            timestamp_input: now.to_string(),
            formatted_input: String::new(),
            custom_format: "%Y-%m-%d %H:%M:%S".to_string(),
            selected_format: DateTimeFormat::Iso8601,
            timestamp_result: String::new(),
            formatted_result: String::new(),
            current_timestamp: now,
            error_message: String::new(),
        }
    }
}

impl DateTimeProcessor {
    pub fn new() -> Self {
        Self::default()
    }

    /// Convert Unix timestamp to formatted date string
    pub fn timestamp_to_formatted(&mut self) {
        self.error_message.clear();

        match self.timestamp_input.parse::<i64>() {
            Ok(timestamp) => match self.create_datetime_from_timestamp(timestamp) {
                Ok(dt) => {
                    let format_str = self.selected_format.format_string();
                    match self.selected_format {
                        DateTimeFormat::Custom(_) => {
                            std::panic::set_hook(Box::new(|_| ()));
                            let formatted = std::panic::catch_unwind(|| {
                                dt.format(&self.custom_format).to_string()
                            });
                            self.formatted_result = formatted.unwrap_or_else(|_| {
                                self.error_message = "Failed to format current time".to_string();
                                String::new()
                            });
                        }
                        _ => {
                            self.formatted_result = dt.format(&format_str).to_string();
                        }
                    }
                }
                Err(e) => {
                    self.error_message = format!("Invalid timestamp: {}", e);
                    self.formatted_result.clear();
                }
            },
            Err(e) => {
                self.error_message = format!("Invalid timestamp format: {}", e);
                self.formatted_result.clear();
            }
        }
    }

    /// Convert formatted date string to Unix timestamp
    pub fn formatted_to_timestamp(&mut self) {
        self.error_message.clear();

        if self.formatted_input.trim().is_empty() {
            self.error_message = "Please enter a date/time string".to_string();
            self.timestamp_result.clear();
            return;
        }

        // Try parsing with the selected format first
        let format_str = match self.selected_format {
            DateTimeFormat::Custom(_) => &self.custom_format,
            _ => &self.selected_format.format_string(),
        };

        if let Ok(dt) = self.parse_datetime_with_format(&self.formatted_input, format_str) {
            self.timestamp_result = dt.timestamp().to_string();
            return;
        }

        for format in &COMMON_FORMATS {
            if let Ok(dt) = self.parse_datetime_with_format(&self.formatted_input, format) {
                self.timestamp_result = dt.timestamp().to_string();
                return;
            }
        }

        // Try parsing RFC formats
        if let Ok(dt) = DateTime::parse_from_rfc2822(&self.formatted_input) {
            self.timestamp_result = dt.timestamp().to_string();
            return;
        }

        if let Ok(dt) = DateTime::parse_from_rfc3339(&self.formatted_input) {
            self.timestamp_result = dt.timestamp().to_string();
            return;
        }

        self.error_message = "Unable to parse date/time. Please check the format.".to_string();
        self.timestamp_result.clear();
    }

    /// Get current timestamp from thread-safe cache
    pub fn get_current_timestamp(&mut self) {
        let now = self.get_cached_timestamp();
        self.current_timestamp = now;
        self.timestamp_input = now.to_string();
        self.timestamp_to_formatted();
    }

    /// Get current timestamp from thread-safe cache
    pub fn get_cached_timestamp(&self) -> i64 {
        TIMESTAMP_CACHE
            .read()
            .map(|ts| *ts)
            .unwrap_or_else(|_| Utc::now().timestamp())
    }

    pub fn update_current_timestamp(&mut self) {
        self.current_timestamp = self.get_cached_timestamp();
    }

    pub fn update_custom_format(&mut self, format: String) {
        self.custom_format = format;
        if matches!(self.selected_format, DateTimeFormat::Custom(_)) {
            self.selected_format = DateTimeFormat::Custom(self.custom_format.clone());
            if !self.timestamp_input.is_empty() {
                self.timestamp_to_formatted();
            }
        }
    }

    pub fn update_selected_format(&mut self, format: DateTimeFormat) {
        self.selected_format = format;
        if !self.timestamp_input.is_empty() {
            self.timestamp_to_formatted();
        }
    }

    fn create_datetime_from_timestamp(&self, timestamp: i64) -> Result<DateTime<Utc>, String> {
        // Handle both seconds and milliseconds timestamps
        let dt = if timestamp > 9999999999 {
            // Likely milliseconds
            match Utc.timestamp_opt(timestamp / 1000, ((timestamp % 1000) * 1_000_000) as u32) {
                chrono::LocalResult::Single(dt) => dt,
                _ => return Err("Invalid timestamp".to_string()),
            }
        } else {
            // Likely seconds
            match Utc.timestamp_opt(timestamp, 0) {
                chrono::LocalResult::Single(dt) => dt,
                _ => return Err("Invalid timestamp".to_string()),
            }
        };
        Ok(dt)
    }

    fn parse_datetime_with_format(
        &self,
        input: &str,
        format: &str,
    ) -> Result<DateTime<Utc>, String> {
        if let Ok(naive_dt) = NaiveDateTime::parse_from_str(input, format) {
            return Ok(Utc.from_utc_datetime(&naive_dt));
        }

        if let Ok(naive_dt) = NaiveDateTime::parse_from_str(input, format) {
            if let chrono::LocalResult::Single(local_dt) = naive_dt.and_local_timezone(Local) {
                return Ok(local_dt.with_timezone(&Utc));
            }
        }

        Err("Failed to parse datetime".to_string())
    }

    pub fn format_current_time(&mut self) -> String {
        let now = Utc::now();
        let format_str = match self.selected_format {
            DateTimeFormat::Custom(_) => &self.custom_format,
            _ => &self.selected_format.format_string(),
        };

        ////TODO: format can panic if the format is invalid
        std::panic::set_hook(Box::new(|_| ()));
        let formatted = std::panic::catch_unwind(|| now.format(format_str).to_string());
        formatted.unwrap_or_else(|_| {
            self.error_message = "Failed to format current time".to_string();
            String::new()
        })
    }

    pub fn get_relative_time(&self, timestamp: i64) -> String {
        match self.create_datetime_from_timestamp(timestamp) {
            Ok(dt) => {
                let now = Utc::now();
                let duration = now.signed_duration_since(dt);

                if duration.num_seconds().abs() < 60 {
                    "just now".to_string()
                } else if duration.num_minutes().abs() < 60 {
                    let minutes = duration.num_minutes();
                    if minutes > 0 {
                        format!(
                            "{} minute{} ago",
                            minutes,
                            if minutes == 1 { "" } else { "s" }
                        )
                    } else {
                        format!(
                            "in {} minute{}",
                            -minutes,
                            if minutes == -1 { "" } else { "s" }
                        )
                    }
                } else if duration.num_hours().abs() < 24 {
                    let hours = duration.num_hours();
                    if hours > 0 {
                        format!("{} hour{} ago", hours, if hours == 1 { "" } else { "s" })
                    } else {
                        format!("in {} hour{}", -hours, if hours == -1 { "" } else { "s" })
                    }
                } else {
                    let days = duration.num_days();
                    if days > 0 {
                        format!("{} day{} ago", days, if days == 1 { "" } else { "s" })
                    } else {
                        format!("in {} day{}", -days, if days == -1 { "" } else { "s" })
                    }
                }
            }
            Err(_) => "Invalid timestamp".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timestamp_to_formatted() {
        let mut processor = DateTimeProcessor::new();
        processor.timestamp_input = "1640995200".to_string(); // 2022-01-01 00:00:00 UTC
        processor.selected_format = DateTimeFormat::Iso8601;
        processor.timestamp_to_formatted();

        assert!(!processor.formatted_result.is_empty());
        assert!(processor.error_message.is_empty());
    }

    #[test]
    fn test_formatted_to_timestamp() {
        let mut processor = DateTimeProcessor::new();
        processor.formatted_input = "2022-01-01 00:00:00".to_string();
        processor.formatted_to_timestamp();

        assert!(!processor.timestamp_result.is_empty());
        assert!(processor.error_message.is_empty());
    }

    #[test]
    fn test_relative_time() {
        let processor = DateTimeProcessor::new();
        let now = Utc::now().timestamp();
        let one_hour_ago = now - 3600;
        let relative = processor.get_relative_time(one_hour_ago);

        assert!(relative.contains("hour") && relative.contains("ago"));
    }
}
