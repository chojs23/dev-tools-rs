#![allow(dead_code)]
#![allow(unused_imports)]

use anyhow::{Context, Result};
use eframe::Storage;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use crate::{core::color::*, ui::components::layout::HarmonyLayout};

pub const DEFAULT_PIXELS_PER_POINT: f32 = 1.0;

pub fn load_global(_storage: Option<&dyn eframe::Storage>) -> Option<Settings> {
    #[cfg(target_arch = "wasm32")]
    if let Some(storage) = _storage {
        if let Some(yaml) = storage.get_string(Settings::STORAGE_KEY) {
            if let Ok(settings) = Settings::from_yaml_str(&yaml) {
                return Some(settings);
            }
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    if let Some(config_dir) = Settings::dir("d_tools") {
        let path = config_dir.join(Settings::FILE_NAME);

        if let Ok(settings) = Settings::load(path) {
            return Some(settings);
        }
    }

    None
}

pub fn save_global(settings: &Settings, _storage: &mut dyn Storage) {
    #[cfg(target_arch = "wasm32")]
    if let Ok(yaml) = settings.as_yaml_str() {
        _storage.set_string(Settings::STORAGE_KEY, yaml);
    }
    #[cfg(not(target_arch = "wasm32"))]
    if let Some(dir) = Settings::dir("d_tools") {
        if !dir.exists() {
            let _ = std::fs::create_dir_all(&dir);
        }
        let _ = settings.save(dir.join(Settings::FILE_NAME));
    }
}

fn enabled() -> bool {
    true
}

fn is_false(it: &bool) -> bool {
    !*it
}

fn is_true(it: &bool) -> bool {
    *it
}

fn is_default_color_size(it: &f32) -> bool {
    *it == DEFAULT_COLOR_SIZE
}

const DEFAULT_COLOR_SIZE: f32 = 100.;

fn default_color_size() -> f32 {
    DEFAULT_COLOR_SIZE
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Settings {
    #[serde(default)]
    pub color_display_format: ColorDisplayFmtEnum,
    #[serde(default)]
    pub color_clipboard_format: Option<ColorDisplayFmtEnum>,
    // #[serde(default)]
    // pub palette_clipboard_format: PaletteFormat,
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub saved_color_formats: HashMap<String, String>,
    #[serde(default = "enabled")]
    #[serde(skip_serializing_if = "is_true")]
    pub is_dark_mode: bool,
    #[serde(default = "enabled")]
    #[serde(skip_serializing_if = "is_true")]
    pub cache_colors: bool,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_false")]
    pub auto_copy_picked_color: bool,
    #[serde(default = "default_pixels_per_point")]
    #[serde(skip_serializing_if = "is_default_pixels_per_point")]
    pub pixels_per_point: f32,
}

fn default_pixels_per_point() -> f32 {
    DEFAULT_PIXELS_PER_POINT
}

fn is_default_pixels_per_point(ppp: &f32) -> bool {
    *ppp == DEFAULT_PIXELS_PER_POINT
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            is_dark_mode: true,
            color_display_format: ColorDisplayFmtEnum::default(),
            color_clipboard_format: None,
            saved_color_formats: HashMap::default(),
            cache_colors: true,
            auto_copy_picked_color: false,
            pixels_per_point: DEFAULT_PIXELS_PER_POINT,
        }
    }
}

impl Settings {
    pub const STORAGE_KEY: &'static str = "d_tools.saved.settings";
    pub const FILE_NAME: &'static str = "settings.yaml";

    pub fn from_yaml_str(yaml: &str) -> Result<Self> {
        serde_yaml::from_str(yaml).context("failed to deserialize settings from YAML")
    }

    pub fn as_yaml_str(&self) -> Result<String> {
        serde_yaml::to_string(&self).context("failed to serialize settings as YAML")
    }

    /// Loads the settings from the configuration file located at `path`. The configuration file is
    /// expected to be a valid YAML file.
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let data = fs::read(path).context("failed to read configuration file")?;
        serde_yaml::from_slice(&data).context("failed to deserialize configuration")
    }

    /// Saves this settings as YAML file in the provided `path`.
    pub fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        let mut data = Vec::with_capacity(128);
        serde_yaml::to_writer(&mut data, &self).context("failed to serialize settings")?;
        fs::write(path, &data).context("failed to write settings to file")
    }

    /// Returns system directory where configuration should be placed joined by the `name` parameter.
    pub fn dir(name: impl AsRef<str>) -> Option<PathBuf> {
        let name = name.as_ref();
        if let Some(dir) = dirs::home_dir() {
            return Some(dir.join(name));
        }

        None
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ColorDisplayFmtEnum {
    #[serde(rename = "hex")]
    #[default]
    Hex,
    #[serde(rename = "hex-uppercase")]
    HexUppercase,
    #[serde(rename = "css-rgb")]
    CssRgb,
    #[serde(rename = "css-hsl")]
    CssHsl,
}

impl AsRef<str> for ColorDisplayFmtEnum {
    fn as_ref(&self) -> &str {
        use ColorDisplayFmtEnum::*;
        match &self {
            Hex => "hex",
            HexUppercase => "hex uppercase",
            CssRgb => "css rgb",
            CssHsl => "css hsl",
        }
    }
}

impl ColorDisplayFmtEnum {
    pub fn default_display_format() -> ColorFormat {
        ColorFormat::Hex
    }
}
