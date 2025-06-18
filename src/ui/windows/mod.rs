use eframe::{
    egui::{Color32, Frame, Margin, Ui},
    epaint::{Shadow, Stroke},
};

use super::components::colors::{
    D_BG_00, D_BG_1_TRANSPARENT, D_BG_2_TRANSPARENT, L_BG_2, L_BG_2_TRANSPARENT, L_BG_3_TRANSPARENT,
};

pub mod settings;

pub const WINDOW_X_OFFSET: f32 = 10.;
pub const WINDOW_Y_OFFSET: f32 = 30.;

pub fn default_frame(is_dark_mode: bool) -> Frame {
    Frame {
        fill: if is_dark_mode {
            *D_BG_1_TRANSPARENT
        } else {
            *L_BG_3_TRANSPARENT
        },
        inner_margin: Margin::symmetric(15, 15),
        outer_margin: Margin {
            left: WINDOW_X_OFFSET as i8,
            top: WINDOW_Y_OFFSET as i8,
            right: 0,
            bottom: 0,
        },
        shadow: if is_dark_mode {
            Shadow {
                offset: [0, 0],
                blur: 10,
                spread: 0,
                color: Color32::from_black_alpha(96),
            }
        } else {
            Shadow {
                offset: [0, 0],
                blur: 10,
                spread: 0,
                color: Color32::from_black_alpha(20),
            }
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
