mod app;
mod context;
mod core;
mod platform;
mod settings;
mod types;
mod ui;
mod utils;

pub use app::App as DevToolsApp;

use anyhow::{Context, Result};

#[cfg(not(target_arch = "wasm32"))]
fn save_to_clipboard(text: String) -> Result<()> {
    let mut clipboard = arboard::Clipboard::new()?;
    clipboard
        .set_text(text)
        .context("failed to save to clipboard")
}

#[cfg(not(target_arch = "wasm32"))]
fn get_timestamp() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(1)
}

fn elapsed(timestamp: u64) -> u64 {
    get_timestamp() - timestamp
}
