use eframe::{
    egui::{self, CursorIcon},
    CreationContext, Storage,
};
use serde::{Deserialize, Serialize};

use crate::{
    app::{CentralPanelTab, DARK_VISUALS, LIGHT_VISUALS},
    core::{
        color::{palettes::Palettes, Color, ColorFormat},
        crypto::CryptographyProcessor,
        datetime::DateTimeProcessor,
        encoding::EncodingProcessor,
        generators::GeneratorProcessor,
        jwt::JwtEncoderDecoder,
        regex::RegexProcessor,
    },
    settings::{ColorDisplayFmtEnum, Settings},
    types::error::append_global_error,
    ui::components::color_picker::ColorPicker,
    utils::{
        render::{TextureAllocator, TextureManager},
        screen_size::ScreenSize,
    },
    APP_NAME,
};

#[derive(Clone, Debug)]
pub struct AppCtx {
    pub settings: Settings,

    pub screen_size: ScreenSize,
    pub cursor_icon: CursorIcon,

    pub sidepanel: SidePanelData,

    pub jwt: JwtEncoderDecoder,
    pub encoding: EncodingProcessor,
    pub regex: RegexProcessor,
    pub generator: GeneratorProcessor,
    pub datetime: DateTimeProcessor,
    pub crypto: CryptographyProcessor,
    pub picker: ColorPicker,
    pub palettes: Palettes,

    pub cursor_pick_color: Color,
    pub current_selected_color: Color,

    pub central_panel_tab: CentralPanelTab,

    pub zoom_window_dragged: bool,
    pub color_picking_enabled: bool,
    pub color_picking_history: Vec<Color>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SidePanelData {
    pub show: bool,
    pub edit_palette_name: bool,
    pub trigger_edit_focus: bool,
    pub box_width: f32,
    pub response_size: egui::Vec2,
}

impl Default for AppCtx {
    fn default() -> Self {
        Self {
            settings: Settings::default(),
            cursor_icon: CursorIcon::default(),
            screen_size: ScreenSize::Desktop(0., 0.),
            sidepanel: SidePanelData {
                show: false,
                edit_palette_name: false,
                trigger_edit_focus: false,
                box_width: 0.,
                response_size: (0., 0.).into(),
            },
            jwt: JwtEncoderDecoder::default(),
            encoding: EncodingProcessor::default(),
            regex: RegexProcessor::default(),
            generator: GeneratorProcessor::default(),
            datetime: DateTimeProcessor::default(),
            crypto: CryptographyProcessor::default(),
            picker: ColorPicker::default(),
            palettes: Palettes::default(),
            cursor_pick_color: Color::black(),
            current_selected_color: Color::black(),
            central_panel_tab: CentralPanelTab::Jwt,
            zoom_window_dragged: false,
            color_picking_enabled: false,
            color_picking_history: Vec::new(),
        }
    }
}

impl AppCtx {
    pub const KEY: &'static str = "app-global-ctx";

    /// Initialize a new context
    pub fn new(context: &CreationContext) -> Self {
        Self {
            settings: crate::settings::load_global(context.storage).unwrap_or_default(),
            color_picking_enabled: false,
            ..Default::default()
        }
    }

    /// Current color display format
    pub fn display_format(&self) -> ColorFormat {
        match self.settings.color_display_format {
            ColorDisplayFmtEnum::Hex => ColorFormat::Hex,
            ColorDisplayFmtEnum::HexUppercase => ColorFormat::HexUpercase,
            ColorDisplayFmtEnum::CssRgb => ColorFormat::CssRgb,
            ColorDisplayFmtEnum::CssHsl => ColorFormat::CssHsl {
                degree_symbol: true,
            },
        }
    }

    /// Format a color as a string using display color format from settings
    pub fn display_color(&self, color: &Color) -> String {
        color.display(self.display_format())
    }

    /// Format a color as a string using clipboard color format from settings
    pub fn clipboard_color(&self, color: &Color) -> String {
        let format = match self
            .settings
            .color_clipboard_format
            .as_ref()
            .unwrap_or(&self.settings.color_display_format)
        {
            ColorDisplayFmtEnum::Hex => ColorFormat::Hex,
            ColorDisplayFmtEnum::HexUppercase => ColorFormat::HexUpercase,
            ColorDisplayFmtEnum::CssRgb => ColorFormat::CssRgb,
            ColorDisplayFmtEnum::CssHsl => ColorFormat::CssHsl {
                degree_symbol: false,
            },
        };
        color.display(format)
    }

    /// Load palettes from appropriate location based on the target arch
    pub fn load_palettes(&mut self, _storage: Option<&dyn Storage>) {
        if self.settings.cache_colors {
            #[cfg(target_arch = "wasm32")]
            if let Some(storage) = _storage {
                match Palettes::load_from_storage(storage) {
                    Ok(palettes) => self.palettes = palettes,
                    Err(e) => append_global_error(format!("failed to load palettes, {e:?}")),
                }
            }

            #[cfg(not(target_arch = "wasm32"))]
            if let Some(path) = Palettes::dir(APP_NAME) {
                println!("Loading palettes from: {}", path.display());
                match Palettes::load_or_default(path.join(Palettes::FILE_NAME)) {
                    Ok(palettes) => self.palettes = palettes,
                    Err(e) => append_global_error(format!("failed to load palettes, {e:?}")),
                }
            }
        }
    }

    /// Save palettes to appropriate location based on the target arch
    pub fn save_palettes(&self, _storage: &mut dyn Storage) {
        #[cfg(target_arch = "wasm32")]
        if self.settings.cache_colors {
            if let Err(e) = self.palettes.save_to_storage(_storage) {
                append_global_error(format!("failed to save palettes, {e:?}"));
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(dir) = Palettes::dir(APP_NAME) {
            if !dir.exists() {
                let _ = std::fs::create_dir_all(&dir);
            }
            if let Err(e) = self.palettes.save(dir.join(Palettes::FILE_NAME)) {
                append_global_error(format!("failed to save palettes, {e:?}"));
            }
        }
    }

    /// Adds a color to the currently selected palette
    pub fn add_color(&mut self, color: Color) {
        if !self.palettes.current_mut().palette.add(color) {
            let color_str = self.display_color(&color);
            append_global_error(format!("Color {} already saved!", color_str));
        } else {
            self.sidepanel.show = true;
        }
    }

    pub fn add_cur_color(&mut self) {
        self.add_color(self.picker.current_color)
    }

    /// Replaces cursor icon with `icon`
    pub fn toggle_mouse(&mut self, icon: CursorIcon) {
        self.cursor_icon = if icon == self.cursor_icon {
            CursorIcon::default()
        } else {
            icon
        }
    }

    pub fn check_settings_change(&mut self) {
        //TODO:
    }
}

pub struct FrameCtx<'frame> {
    pub app: &'frame mut AppCtx,
    pub egui: &'frame egui::Context,
    pub tex_manager: &'frame mut TextureManager,
    pub frame: Option<&'frame mut eframe::Frame>,
}

impl<'frame> FrameCtx<'frame> {
    pub fn tex_allocator(&self) -> TextureAllocator {
        Some(self.egui.tex_manager())
    }

    pub fn is_dark_mode(&self) -> bool {
        self.app.settings.is_dark_mode
    }

    pub fn set_dark_theme(&mut self) {
        self.app.settings.is_dark_mode = true;
        self.egui.set_visuals(DARK_VISUALS.clone());
    }

    pub fn set_light_theme(&mut self) {
        self.app.settings.is_dark_mode = false;
        self.egui.set_visuals(LIGHT_VISUALS.clone());
    }

    pub fn set_theme(&mut self) {
        if self.is_dark_mode() {
            self.set_light_theme();
        } else {
            self.set_dark_theme();
        }
    }

    pub fn set_styles(&mut self, screen_size: ScreenSize) {
        self.app.screen_size = screen_size;

        let slider_size = match screen_size {
            ScreenSize::Phone(w, _) => w * 0.5,
            ScreenSize::Desktop(w, _) if w > 1500. => w * 0.2,
            ScreenSize::Tablet(w, _) | ScreenSize::Laptop(w, _) | ScreenSize::Desktop(w, _) => {
                w * 0.35
            }
        };

        let mut style = (*self.egui.style()).clone();
        style.spacing.slider_width = slider_size / 2.;
        self.egui.set_style(style);
    }

    // #[cfg(not(target_arch = "wasm32"))]
    // pub fn set_window_size(&mut self, size: egui::Vec2) {
    //     if let Some(frame) = self.frame.as_mut() {}
    // }
}
