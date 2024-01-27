use crate::get_timestamp;
use once_cell::sync::Lazy;
use std::collections::VecDeque;
#[cfg(not(target_arch = "wasm32"))]
use std::sync::Mutex;

#[derive(Debug)]
pub struct DisplayError {
    message: String,
    timestamp: u64,
}

impl DisplayError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            timestamp: get_timestamp(),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }
}

#[derive(Default)]
pub struct ErrorStack {
    pub errors: VecDeque<DisplayError>,
}

impl ErrorStack {
    pub fn push(&mut self, message: impl Into<String>) {
        self.errors.push_back(DisplayError::new(message))
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub static ERROR_STACK: Lazy<Mutex<ErrorStack>> = Lazy::new(|| Mutex::new(ErrorStack::default()));
#[cfg(target_arch = "wasm32")]
pub static mut ERROR_STACK: Lazy<ErrorStack> = Lazy::new(|| ErrorStack::default());

pub fn append_global_error(error: impl std::fmt::Display) {
    #[cfg(not(target_arch = "wasm32"))]
    if let Ok(mut stack) = ERROR_STACK.try_lock() {
        stack.push(error.to_string());
    }
    #[cfg(target_arch = "wasm32")]
    unsafe {
        ERROR_STACK.push(error.to_string());
    }
}
