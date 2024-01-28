use eframe::{egui, epaint::TextureManager, CreationContext};
use serde::{Deserialize, Serialize};

use crate::{
    app::{CentralPanelTab, DARK_VISUALS, LIGHT_VISUALS},
    screen_size::ScreenSize,
    settings::{self, Settings},
};

#[derive(Clone, Debug)]
pub struct AppCtx {
    pub settings: Settings,
    pub screen_size: ScreenSize,
    pub sidepanel: SidePanelData,
    // pub jwt: JwtEncoderDecoder,
    pub central_panel_tab: CentralPanelTab,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SidePanelData {
    /// Is the side panel visible
    pub show: bool,
    /// If true palette name is currently being edited
    pub edit_palette_name: bool,
    /// When triggering palette name edit this is used to
    /// switch focus to the textedit
    pub trigger_edit_focus: bool,
    /// Width of the button toolbar on the sidebar
    pub box_width: f32,
    /// Size of the whole Sidebar response
    pub response_size: egui::Vec2,
}

impl Default for AppCtx {
    fn default() -> Self {
        Self {
            settings: Settings::default(),
            screen_size: ScreenSize::Desktop(0., 0.),
            sidepanel: SidePanelData {
                show: false,
                edit_palette_name: false,
                trigger_edit_focus: false,
                box_width: 0.,
                response_size: (0., 0.).into(),
            },
            central_panel_tab: CentralPanelTab::Jwt,
        }
    }
}

impl AppCtx {
    pub const KEY: &'static str = "app-global-ctx";

    /// Initialize a new context
    pub fn new(context: &CreationContext) -> Self {
        Self {
            settings: settings::load_global(context.storage).unwrap_or_default(),
            screen_size: ScreenSize::Desktop(0., 0.),
            sidepanel: SidePanelData {
                show: false,
                edit_palette_name: false,
                trigger_edit_focus: false,
                box_width: 0.,
                response_size: (0., 0.).into(),
            },
            central_panel_tab: CentralPanelTab::Jwt,
        }
    }

    pub fn check_settings_change(&mut self) {
        // if self.settings.chromatic_adaptation_method
        //     != self.picker.sliders.chromatic_adaptation_method
        // {
        //     self.picker.sliders.chromatic_adaptation_method =
        //         self.settings.chromatic_adaptation_method;
        // }
        // if self.settings.rgb_working_space != self.picker.sliders.rgb_working_space {
        //     self.picker.new_workspace = Some(self.settings.rgb_working_space);
        //     if self.settings.illuminant != self.picker.sliders.illuminant {
        //         self.picker.new_illuminant = Some(self.settings.illuminant);
        //     }
        // }
    }
}

pub struct FrameCtx<'frame> {
    pub app: &'frame mut AppCtx,
    pub egui: &'frame egui::Context,
    pub tex_manager: &'frame mut TextureManager,
    pub frame: Option<&'frame mut eframe::Frame>,
}

impl<'frame> FrameCtx<'frame> {
    // pub fn tex_allocator(&self) -> TextureAllocator {
    //     Some(self.egui.tex_manager())
    // }

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
    //     if let Some(frame) = self.frame.as_mut() {
    //         frame.set_window_size(size);
    //     }
    // }
}
