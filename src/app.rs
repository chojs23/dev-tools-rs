use std::sync::{Arc, RwLock};

use eframe::{
    egui::{self, Margin, Theme, Ui, Visuals},
    CreationContext,
};
use once_cell::sync::{Lazy, OnceCell};

use crate::{
    context::{AppCtx, FrameCtx},
    types::error::{append_global_error, ERROR_STACK},
    ui::{
        components::{colors::*, *},
        panels::{
            color_picker_panel::ColorPickerPanel, datetime_panel::DateTimePanel,
            encoding_panel::EncodingPanel, error_display::ErrorDisplay,
            generators_panel::GeneratorsPanel, jwt_panel::JwtPanel, regex_panel::RegexPanel,
            top_panel::TopPanel,
        },
        traits::{UiComponent, UiPanel, UiWindow},
        windows::settings::SettingsWindow,
    },
    utils::{render::TextureManager, screen_size::ScreenSize},
};

pub static ADD_DESCRIPTION: &str = "Add this color to saved colors";

pub static LIGHT_VISUALS: Lazy<Visuals> = Lazy::new(light_visuals);
pub static DARK_VISUALS: Lazy<Visuals> = Lazy::new(dark_visuals);
pub static TEXTURE_MANAGER: Lazy<RwLock<TextureManager>> =
    Lazy::new(|| RwLock::new(TextureManager::default()));
pub static CONTEXT: OnceCell<RwLock<AppCtx>> = OnceCell::new();

pub static ERROR_DISPLAY_DURATION: u64 = 10;

pub const CURRENT_COLOR_BOX_SIZE: f32 = 40.0;

#[derive(Clone, Debug)]
pub enum CentralPanelTab {
    Jwt,
    Encoding,
    Regex,
    ColorPicker,
    Generators,
    DateTime,
}

#[derive(Default)]
pub struct Windows {
    pub settings: SettingsWindow,
    // pub help: HelpWindow,
}

pub struct App {
    pub windows: Windows,
    pub top_panel: TopPanel,
    pub jwt_panel: JwtPanel,
    pub encoding_panel: EncodingPanel,
    pub regex_panel: RegexPanel,
    pub color_picker_panel: ColorPickerPanel,
    pub generators_panel: GeneratorsPanel,
    pub datetime_panel: DateTimePanel,
    pub error_display: ErrorDisplay,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if let Some(mut app_ctx) = CONTEXT.get().and_then(|ctx| ctx.write().ok()) {
            let res = TEXTURE_MANAGER.try_write();
            if let Err(e) = &res {
                append_global_error(e);
                return;
            }
            let mut tex_manager = res.unwrap();

            let mut ctx = FrameCtx {
                app: &mut app_ctx,
                egui: ctx,
                tex_manager: &mut tex_manager,
                frame: Some(frame),
            };

            let screen_size = ScreenSize::from(ctx.egui.available_rect());
            if ctx.app.screen_size != screen_size {
                ctx.set_styles(screen_size);
            }

            ctx.egui
                .set_pixels_per_point(ctx.app.settings.pixels_per_point);
            ctx.app.check_settings_change();

            self.top_panel(&mut ctx);

            self.central_panel(&mut ctx);

            self.display_windows(&mut ctx);

            // Global detection for color picking
            if ctx.app.color_picking_enabled {
                self.check_backtick_key_pressed(&mut ctx);
            }

            if let Ok(mut stack) = ERROR_STACK.try_lock() {
                while let Some(error) = stack.errors.pop_front() {
                    self.error_display.add_error(error);
                }
            }

            // Limit frame when out of focus
            #[cfg(not(target_arch = "wasm32"))]
            if !ctx.egui.is_pointer_over_area() {
                if ctx.app.zoom_window_dragged {
                    return;
                }

                std::thread::sleep(std::time::Duration::from_millis(100));
            }

            ctx.app.current_selected_color = ctx.app.picker.current_color;
        }
    }
}

impl App {
    pub fn init(context: &CreationContext) -> Box<dyn eframe::App + 'static> {
        let mut app_ctx = AppCtx::new(context);

        let app = Box::new(Self {
            windows: Windows::default(),
            top_panel: TopPanel::new(),
            jwt_panel: JwtPanel::new(),
            encoding_panel: EncodingPanel::new(),
            regex_panel: RegexPanel::new(),
            color_picker_panel: ColorPickerPanel::new(),
            generators_panel: GeneratorsPanel::new(),
            datetime_panel: DateTimePanel::new(),
            error_display: ErrorDisplay::new(),
        });

        if let Ok(mut tex_manager) = TEXTURE_MANAGER.write() {
            let mut ctx = FrameCtx {
                app: &mut app_ctx,
                egui: &context.egui_ctx,
                tex_manager: &mut tex_manager,
                frame: None,
            };

            match context.egui_ctx.system_theme() {
                Some(Theme::Dark) => {
                    ctx.set_dark_theme();
                }
                Some(Theme::Light) => {
                    ctx.set_light_theme();
                }
                _ => {
                    ctx.set_dark_theme();
                }
            }

            ctx.app.load_palettes(context.storage);
        }

        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "Iosevka".to_string(),
            Arc::new(egui::FontData::from_static(include_bytes!(
                "../assets/fonts/Iosevka/IosevkaNerdFont-Regular.ttf"
            ))),
        );
        fonts
            .families
            .get_mut(&egui::FontFamily::Monospace)
            .unwrap()
            .insert(0, "Iosevka".to_owned());

        context.egui_ctx.set_fonts(fonts);

        CONTEXT.try_insert(RwLock::new(app_ctx)).unwrap();

        app
    }

    fn top_ui(&mut self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        ui.horizontal(|ui| {
            self.top_panel.render_tab_buttons(ctx, ui);
            ui.add_space(DOUBLE_SPACE);
            if self.top_panel.render_right_side_buttons(ctx, ui) {
                self.windows.settings.toggle();
            }
        });
    }

    fn display_windows(&mut self, ctx: &mut FrameCtx<'_>) {
        self.windows.settings.display(ctx);
    }

    fn top_panel(&mut self, ctx: &mut FrameCtx<'_>) {
        let frame = egui::Frame {
            fill: if ctx.egui.style().visuals.dark_mode {
                *D_BG_00
            } else {
                *L_BG_0
            },
            inner_margin: Margin::symmetric(15, 10),
            ..Default::default()
        };
        egui::TopBottomPanel::top("top panel")
            .frame(frame)
            .show(ctx.egui, |ui| {
                self.top_ui(ctx, ui);
            });
    }

    fn central_panel(&mut self, ctx: &mut FrameCtx<'_>) {
        let _frame = egui::Frame {
            fill: if ctx.egui.style().visuals.dark_mode {
                *D_BG_0
            } else {
                *L_BG_2
            },

            inner_margin: Margin {
                left: 10,
                top: 5,
                right: 0,
                bottom: 0,
            },
            ..Default::default()
        };

        egui::CentralPanel::default()
            .frame(_frame)
            .show(ctx.egui, |ui| match ctx.app.central_panel_tab {
                CentralPanelTab::Jwt => self.jwt_ui(ctx, ui),
                CentralPanelTab::ColorPicker => self.color_picker_ui(ctx, ui),
                CentralPanelTab::Encoding => self.encoding_panel_ui(ctx, ui),
                CentralPanelTab::Regex => self.regex_panel_ui(ctx, ui),
                CentralPanelTab::Generators => self.generators_panel_ui(ctx, ui),
                CentralPanelTab::DateTime => self.datetime_panel_ui(ctx, ui),
            });
    }

    fn jwt_ui(&mut self, ctx: &mut FrameCtx<'_>, ui: &mut egui::Ui) {
        self.error_display.render(ctx, ui);
        self.jwt_panel.display(ctx, ui);
    }

    fn color_picker_ui(&mut self, ctx: &mut FrameCtx<'_>, ui: &mut egui::Ui) {
        self.error_display.render(ctx, ui);
        self.color_picker_panel.display(ctx, ui);
    }

    fn encoding_panel_ui(&mut self, ctx: &mut FrameCtx<'_>, ui: &mut egui::Ui) {
        self.error_display.render(ctx, ui);
        self.encoding_panel.display(ctx, ui);
    }

    fn regex_panel_ui(&mut self, ctx: &mut FrameCtx<'_>, ui: &mut egui::Ui) {
        self.error_display.render(ctx, ui);
        self.regex_panel.display(ctx, ui);
    }

    fn generators_panel_ui(&mut self, ctx: &mut FrameCtx<'_>, ui: &mut egui::Ui) {
        self.error_display.render(ctx, ui);
        self.generators_panel.display(ctx, ui);
    }

    fn datetime_panel_ui(&mut self, ctx: &mut FrameCtx<'_>, ui: &mut egui::Ui) {
        self.error_display.render(ctx, ui);
        self.datetime_panel.display(ctx, ui);
    }

    fn check_backtick_key_pressed(&mut self, ctx: &mut FrameCtx<'_>) {
        //TODO: Capture mouse clicked event

        // Check for backtick/grave key (`) to select color
        if ctx.egui.input(|i| i.key_pressed(egui::Key::Backtick)) {
            let picked_color = ctx.app.cursor_pick_color;
            ctx.app.picker.current_color = picked_color;

            // Add to picking history (avoid duplicates of the same color)
            if ctx.app.color_picking_history.is_empty()
                || ctx.app.color_picking_history.last() != Some(&picked_color)
            {
                ctx.app.color_picking_history.push(picked_color);

                // Keep history limited to last 20 colors
                if ctx.app.color_picking_history.len() > 20 {
                    ctx.app.color_picking_history.remove(0);
                }
            }

            ctx.app.color_picking_enabled = false;
        }

        // Check for Escape key to cancel picking
        if ctx.egui.input(|i| i.key_pressed(egui::Key::Escape)) {
            ctx.app.color_picking_enabled = false;
        }
    }
}
