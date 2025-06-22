use eframe::egui::{Button, CursorIcon, Layout, Rgba, Ui};

use crate::{
    app::CentralPanelTab,
    context::FrameCtx,
    ui::{
        components::{icon, DOUBLE_SPACE},
        traits::UiPanel,
    },
};

#[derive(Clone)]
pub struct TopPanel;

impl UiPanel for TopPanel {
    fn display(&mut self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        ui.horizontal(|ui| {
            self.render_tab_buttons(ctx, ui);
            ui.add_space(DOUBLE_SPACE);
            let _settings_clicked = self.render_right_side_buttons(ctx, ui);
            // Settings handling will be done in the App struct
        });
    }
}

impl Default for TopPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl TopPanel {
    pub fn new() -> Self {
        Self
    }

    pub fn render_tab_buttons(&self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        macro_rules! add_button_if {
            ($text:expr, $condition:expr, $block:tt) => {
                add_button_if!($text, $condition, $block, $block);
            };
            ($text:expr, $condition:expr, $block_a:tt, $block_b:tt) => {
                if $condition {
                    if ui
                        .button($text)
                        .on_hover_cursor(CursorIcon::PointingHand)
                        .clicked()
                    $block_a;
                } else {
                    let btn = Button::new($text).fill(Rgba::from_black_alpha(0.));
                    if ui
                        .add(btn)
                        .on_hover_cursor(CursorIcon::PointingHand)
                        .clicked()
                    $block_b;
                }
            };
        }

        add_button_if!(
            "JWT",
            matches!(ctx.app.central_panel_tab, CentralPanelTab::Jwt),
            {
                ctx.app.central_panel_tab = CentralPanelTab::Jwt;
            }
        );
        add_button_if!(
            "ColorPicker",
            matches!(ctx.app.central_panel_tab, CentralPanelTab::ColorPicker),
            {
                ctx.app.central_panel_tab = CentralPanelTab::ColorPicker;
                ctx.app.sidepanel.show = false;
            }
        );
        add_button_if!(
            "Encoding",
            matches!(ctx.app.central_panel_tab, CentralPanelTab::Encoding),
            {
                ctx.app.central_panel_tab = CentralPanelTab::Encoding;
                ctx.app.sidepanel.show = false;
            }
        );
        add_button_if!(
            "Regex",
            matches!(ctx.app.central_panel_tab, CentralPanelTab::Regex),
            {
                ctx.app.central_panel_tab = CentralPanelTab::Regex;
                ctx.app.sidepanel.show = false;
            }
        );
        add_button_if!(
            "Generators",
            matches!(ctx.app.central_panel_tab, CentralPanelTab::Generators),
            {
                ctx.app.central_panel_tab = CentralPanelTab::Generators;
                ctx.app.sidepanel.show = false;
            }
        );
        add_button_if!(
            "DateTime",
            matches!(ctx.app.central_panel_tab, CentralPanelTab::DateTime),
            {
                ctx.app.central_panel_tab = CentralPanelTab::DateTime;
                ctx.app.sidepanel.show = false;
            }
        );
    }

    pub fn render_right_side_buttons(&self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) -> bool {
        let mut settings_clicked = false;
        ui.with_layout(Layout::right_to_left(eframe::emath::Align::Center), |ui| {
            if ui
                .button(icon::EXPAND)
                .on_hover_text("Show/hide side panel")
                .on_hover_cursor(CursorIcon::ResizeHorizontal)
                .clicked()
            {
                ctx.app.sidepanel.show = !ctx.app.sidepanel.show;
            }
            if ui
                .button(icon::SETTINGS)
                .on_hover_text("Settings")
                .on_hover_cursor(CursorIcon::PointingHand)
                .clicked()
            {
                settings_clicked = true;
            }
            self.render_theme_switch(ctx, ui);
        });
        settings_clicked
    }

    fn render_theme_switch(&self, ctx: &mut FrameCtx, ui: &mut Ui) {
        let btn = if ctx.is_dark_mode() {
            icon::LIGHT_MODE
        } else {
            icon::DARK_MODE
        };
        if ui
            .button(btn)
            .on_hover_text("Switch ui color theme")
            .on_hover_cursor(CursorIcon::PointingHand)
            .clicked()
        {
            ctx.set_theme();
        }
    }
}
