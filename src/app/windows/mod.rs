use eframe::{
    egui::{Frame, Margin, Ui},
    epaint::{Rounding, Shadow, Stroke},
};

use crate::ui::colors::*;

mod settings;

pub use settings::SettingsWindow;

pub const WINDOW_X_OFFSET: f32 = 10.;
pub const WINDOW_Y_OFFSET: f32 = 30.;

pub fn default_frame(is_dark_mode: bool) -> Frame {
    Frame {
        fill: if is_dark_mode {
            *D_BG_1_TRANSPARENT
        } else {
            *L_BG_3_TRANSPARENT
        },
        inner_margin: Margin::symmetric(15., 15.),
        rounding: Rounding::same(5.),
        shadow: if is_dark_mode {
            Shadow::default()
        } else {
            Shadow::default()
        },
        stroke: if is_dark_mode {
            Stroke::new(2., *D_BG_00)
        } else {
            Stroke::new(2., *L_BG_2)
        },
        ..Default::default()
    }
}

pub fn apply_default_style(ui: &mut Ui, is_dark_mode: bool) {
    let widgets = &mut ui.style_mut().visuals.widgets;
    if is_dark_mode {
        widgets.inactive.bg_fill = *D_BG_2_TRANSPARENT;
    } else {
        widgets.inactive.bg_fill = *L_BG_2_TRANSPARENT;
    }
}
