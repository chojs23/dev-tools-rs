mod ui;
mod ui_trait;
mod windows;

use std::sync::RwLock;

use eframe::{
    egui::{self, CursorIcon, Margin, Ui, Visuals},
    CreationContext, Theme,
};
use once_cell::sync::{Lazy, OnceCell};

use ui::{ColorPickerPanel, ErrorDisplay, JwtPanel, TopPanel};
use ui_trait::{UiComponent, UiPanel};
use windows::SettingsWindow;

static ADD_DESCRIPTION: &str = "Add this color to saved colors";

use crate::{
    app::colors::*,
    context::{AppCtx, FrameCtx},
    error::ERROR_STACK,
    render::TextureManager,
    screen_size::ScreenSize,
    ui::*,
};

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
    ColorPicker,
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
    pub color_picker_panel: ColorPickerPanel,
    pub error_display: ErrorDisplay,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if let Some(mut app_ctx) = CONTEXT.get().and_then(|ctx| ctx.write().ok()) {
            let res = TEXTURE_MANAGER.try_write();
            if let Err(e) = &res {
                crate::error::append_global_error(e);
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

            if let Ok(mut stack) = ERROR_STACK.try_lock() {
                while let Some(error) = stack.errors.pop_front() {
                    self.error_display.add_error(error);
                }
            }

            #[cfg(not(target_arch = "wasm32"))]
            if !ctx.egui.is_pointer_over_area() {
                ctx.egui.request_repaint();

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
            color_picker_panel: ColorPickerPanel::new(),
            error_display: ErrorDisplay::new(),
        });

        let prefer_dark = context
            .integration_info
            .system_theme
            .map(|t| matches!(t, Theme::Dark))
            .unwrap_or(true);

        if let Ok(mut tex_manager) = TEXTURE_MANAGER.write() {
            let mut ctx = FrameCtx {
                app: &mut app_ctx,
                egui: &context.egui_ctx,
                tex_manager: &mut tex_manager,
                frame: None,
            };

            // ctx.app.load_palettes(context.storage);

            if prefer_dark {
                ctx.set_dark_theme();
            } else {
                ctx.set_light_theme();
            }
        }

        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "Firacode".to_string(),
            egui::FontData::from_static(include_bytes!(
                "../../assets/fonts/FiraCode/FiraCode-Regular.ttf"
            )),
        );
        fonts
            .families
            .get_mut(&egui::FontFamily::Monospace)
            .unwrap()
            .insert(0, "Firacode".to_owned());

        context.egui_ctx.set_fonts(fonts);

        CONTEXT.try_insert(RwLock::new(app_ctx)).unwrap();

        app
    }

    fn top_ui(&mut self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        ui.horizontal(|ui| {
            self.top_panel.render_tab_buttons(ctx, ui);
            ui.add_space(DOUBLE_SPACE);
            if self.top_panel.render_right_side_buttons(ctx, ui) {
                self.windows.settings.show = true;
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
            inner_margin: Margin::symmetric(15., 10.),
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
                left: 10.,
                top: 5.,
                right: 0.,
                bottom: 0.,
            },
            ..Default::default()
        };

        egui::CentralPanel::default()
            .frame(_frame)
            .show(ctx.egui, |ui| match ctx.app.central_panel_tab {
                CentralPanelTab::Jwt => self.jwt_ui(ctx, ui),
                CentralPanelTab::ColorPicker => self.color_picker_ui(ctx, ui),
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
}
