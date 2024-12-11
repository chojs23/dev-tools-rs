pub mod colorbox;
pub mod layout;
pub mod slider_1d;
pub mod slider_2d;

use eframe::{
    egui::{
        style::{Selection, Widgets},
        Visuals,
    },
    epaint::{Color32, Shadow, Stroke},
};

pub const SPACE: f32 = 7.;
pub const DOUBLE_SPACE: f32 = SPACE * 2.;
pub const HALF_SPACE: f32 = SPACE / 2.;

pub mod icon {
    #![allow(dead_code)]
    pub static ADD: &str = "\u{2795}";
    pub static COPY: &str = "\u{1F3F7}";
    pub static ZOOM_PICKER: &str = "\u{1F489}";
    pub static SETTINGS: &str = "\u{2699}";
    pub static EXPAND: &str = "\u{2B0C}";
    pub static EXPORT: &str = "\u{1F5B9}";
    pub static CLEAR: &str = "\u{1F5D1}";
    pub static DELETE: &str = "\u{1F5D9}";
    pub static PLAY: &str = "\u{25B6}";
    pub static DARK_MODE: &str = "\u{1F319}";
    pub static LIGHT_MODE: &str = "\u{2600}";
    pub static HELP: &str = "\u{FF1F}";
    pub static EDIT: &str = "\u{270F}";
    pub static APPLY: &str = "\u{2714}";
}

#[allow(dead_code)]
pub mod colors {
    use eframe::egui;
    use egui::{Color32, Rgba};
    use once_cell::sync::Lazy;

    pub static D_BG_00: Lazy<Color32> = Lazy::new(|| Color32::from_rgb(0x11, 0x16, 0x1b));
    pub static D_BG_0: Lazy<Color32> = Lazy::new(|| Color32::from_rgb(0x16, 0x1c, 0x23));
    pub static D_BG_1: Lazy<Color32> = Lazy::new(|| Color32::from_rgb(0x23, 0x2d, 0x38));
    pub static D_BG_2: Lazy<Color32> = Lazy::new(|| Color32::from_rgb(0x31, 0x3f, 0x4e));
    pub static D_BG_3: Lazy<Color32> = Lazy::new(|| Color32::from_rgb(0x41, 0x53, 0x67));
    pub static D_FG_0: Lazy<Color32> = Lazy::new(|| Color32::from_rgb(0xe5, 0xde, 0xd6));
    pub static D_BG_00_TRANSPARENT: Lazy<Color32> =
        Lazy::new(|| Rgba::from(*D_BG_00).multiply(0.96).into());
    pub static D_BG_0_TRANSPARENT: Lazy<Color32> =
        Lazy::new(|| Rgba::from(*D_BG_0).multiply(0.96).into());
    pub static D_BG_1_TRANSPARENT: Lazy<Color32> =
        Lazy::new(|| Rgba::from(*D_BG_1).multiply(0.96).into());
    pub static D_BG_2_TRANSPARENT: Lazy<Color32> =
        Lazy::new(|| Rgba::from(*D_BG_2).multiply(0.96).into());
    pub static D_BG_3_TRANSPARENT: Lazy<Color32> =
        Lazy::new(|| Rgba::from(*D_BG_3).multiply(0.96).into());
    pub static L_BG_0: Lazy<Color32> = Lazy::new(|| Color32::from_rgb(0xbf, 0xbf, 0xbf));
    pub static L_BG_1: Lazy<Color32> = Lazy::new(|| Color32::from_rgb(0xd4, 0xd3, 0xd4));
    pub static L_BG_2: Lazy<Color32> = Lazy::new(|| Color32::from_rgb(0xd9, 0xd9, 0xd9));
    pub static L_BG_3: Lazy<Color32> = Lazy::new(|| Color32::from_rgb(0xea, 0xea, 0xea));
    pub static L_BG_4: Lazy<Color32> = Lazy::new(|| Color32::from_rgb(0xf9, 0xf9, 0xf9));
    pub static L_BG_5: Lazy<Color32> = Lazy::new(|| Color32::from_rgb(0xff, 0xff, 0xff));
    pub static L_BG_0_TRANSPARENT: Lazy<Color32> =
        Lazy::new(|| Rgba::from(*L_BG_0).multiply(0.86).into());
    pub static L_BG_1_TRANSPARENT: Lazy<Color32> =
        Lazy::new(|| Rgba::from(*L_BG_1).multiply(0.86).into());
    pub static L_BG_2_TRANSPARENT: Lazy<Color32> =
        Lazy::new(|| Rgba::from(*L_BG_2).multiply(0.86).into());
    pub static L_BG_3_TRANSPARENT: Lazy<Color32> =
        Lazy::new(|| Rgba::from(*L_BG_3).multiply(0.86).into());
    pub static L_BG_4_TRANSPARENT: Lazy<Color32> =
        Lazy::new(|| Rgba::from(*L_BG_4).multiply(0.86).into());
    pub static L_BG_5_TRANSPARENT: Lazy<Color32> =
        Lazy::new(|| Rgba::from(*L_BG_5).multiply(0.86).into());
    pub static L_FG_0: Lazy<Color32> = Lazy::new(|| *D_BG_0);
}

use colors::*;

use crate::color::{Color, ColorFormat, Illuminant, RgbWorkingSpace};

pub fn light_visuals() -> Visuals {
    let mut widgets = Widgets::light();
    widgets.noninteractive.bg_fill = *L_BG_4_TRANSPARENT;
    widgets.inactive.bg_fill = *L_BG_3_TRANSPARENT;
    widgets.inactive.bg_stroke = Stroke::new(0.5, *D_BG_3);
    widgets.inactive.fg_stroke = Stroke::new(0.5, *D_BG_3);
    widgets.hovered.bg_fill = *L_BG_4_TRANSPARENT;
    widgets.hovered.bg_stroke = Stroke::new(1., *D_BG_1);
    widgets.hovered.fg_stroke = Stroke::new(1., *D_BG_1);
    widgets.active.bg_fill = *L_BG_5_TRANSPARENT;
    widgets.active.fg_stroke = Stroke::new(1.5, *D_BG_0);
    widgets.active.bg_stroke = Stroke::new(1.5, *D_BG_0);

    Visuals {
        dark_mode: false,
        override_text_color: Some(*L_FG_0),
        extreme_bg_color: Color32::WHITE,
        selection: Selection {
            bg_fill: *L_BG_5,
            stroke: Stroke::new(0.7, *D_BG_0),
        },
        popup_shadow: Shadow::default(),
        widgets,
        ..Default::default()
    }
}

pub fn dark_visuals() -> Visuals {
    let mut widgets = Widgets::dark();
    widgets.noninteractive.bg_fill = *D_BG_2_TRANSPARENT;
    widgets.inactive.bg_fill = *D_BG_1_TRANSPARENT;
    widgets.hovered.bg_fill = *D_BG_2_TRANSPARENT;
    widgets.active.bg_fill = *D_BG_3_TRANSPARENT;

    Visuals {
        dark_mode: true,
        override_text_color: Some(*D_FG_0),
        selection: Selection {
            bg_fill: *D_BG_3_TRANSPARENT,
            stroke: Stroke::new(0.7, *D_FG_0),
        },
        popup_shadow: Shadow::default(),
        widgets,
        ..Default::default()
    }
}

pub fn color_tooltip(
    color: &Color,
    display_format: ColorFormat,
    ws: RgbWorkingSpace,
    illuminant: Illuminant,
    text: Option<&str>,
) -> String {
    format!(
        "{}\n\n{}",
        color.display(display_format, ws, illuminant),
        text.unwrap_or_default()
    )
}
