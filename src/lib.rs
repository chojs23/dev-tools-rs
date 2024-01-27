mod app;
mod context;
mod error;
mod menu;
mod screen_size;
mod settings;
mod ui;

pub use app::App as DevToolsApp;

#[cfg(not(target_arch = "wasm32"))]
fn get_timestamp() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(1)
}
