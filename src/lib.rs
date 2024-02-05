mod app;
mod color;
mod color_picker;
mod context;
mod display_picker;
mod error;
mod jwt;
mod math;
mod render;
mod screen_size;
mod settings;
mod ui;
mod zoom_picker;

use anyhow::{Context, Result};

pub use app::App as DevToolsApp;

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
