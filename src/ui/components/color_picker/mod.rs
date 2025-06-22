mod sliders;

use eframe::{
    egui::{CollapsingHeader, DragValue, Grid, Ui},
    epaint::Hsva,
};
use serde::{Deserialize, Serialize};
use sliders::ColorSliders;
use std::mem;

use super::{slider_1d, slider_2d};
use crate::{
    core::color::{Cmyk, Color, Hsl, Hsv, Rgb, U8_MAX, U8_MIN},
    utils::math,
};

macro_rules! slider {
    ($it:ident, $ui:ident, $field:ident, $label:literal, $range:expr, $($tt:tt)+) => {
            let resp = slider_1d::color(&mut $ui, &mut $it.sliders.$field, $range, $($tt)+).on_hover_text($label);
            if resp.changed() {
                $it.check_for_change();
            }
            $ui.label(format!("{}: ", $label));
            $ui.add(DragValue::new(&mut $it.sliders.$field));
    };
    (int $it:ident, $ui:ident, $field:ident, $label:literal, $range:expr, $($tt:tt)+) => {
            let resp = slider_1d::color(&mut $ui, &mut $it.sliders.$field, $range, $($tt)+).on_hover_text($label);
            if resp.changed() {
                $it.check_for_change();
            }
            $ui.label(format!("{}: ", $label));
            let mut it = $it.sliders.$field as u32;
            let it_copy = it;
            $ui.add(DragValue::new(&mut it));
            if it != it_copy {
                $it.sliders.$field = it as f32;
            }
    };
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ColorPicker {
    pub current_color: Color,
    pub hex_color: String,
    pub sliders: ColorSliders,
    pub saved_sliders: Option<ColorSliders>,
}

impl Default for ColorPicker {
    fn default() -> Self {
        Self {
            current_color: Color::black(),
            hex_color: "".to_string(),
            sliders: ColorSliders::default(),
            saved_sliders: None,
        }
    }
}

impl ColorPicker {
    pub fn set_cur_color(&mut self, color: impl Into<Color>) {
        let color = color.into();
        self.sliders.set_color(color);
        self.current_color = color;
    }

    fn restore_sliders_if_saved(&mut self) {
        if let Some(saved) = mem::take(&mut self.saved_sliders) {
            self.sliders.restore(saved);
        }
    }

    fn save_sliders_if_unsaved(&mut self) {
        if self.saved_sliders.is_none() {
            self.saved_sliders = Some(self.sliders.clone());
        }
    }

    fn rgb_changed(&mut self) -> bool {
        let rgb = self.current_color.rgb();
        let r = self.sliders.r;
        let g = self.sliders.g;
        let b = self.sliders.b;
        if !math::eq_f32(r, rgb.r_scaled())
            || !math::eq_f32(g, rgb.g_scaled())
            || !math::eq_f32(b, rgb.b_scaled())
        {
            self.saved_sliders = None;
            self.set_cur_color(Rgb::new(r / U8_MAX, g / U8_MAX, b / U8_MAX));
            true
        } else {
            false
        }
    }

    fn cmyk_changed(&mut self) -> bool {
        let cmyk = Cmyk::from(self.current_color);
        if !math::eq_f32(self.sliders.c, cmyk.c_scaled())
            || !math::eq_f32(self.sliders.m, cmyk.m_scaled())
            || !math::eq_f32(self.sliders.y, cmyk.y_scaled())
            || !math::eq_f32(self.sliders.k, cmyk.k_scaled())
        {
            if math::eq_f32(self.sliders.k, 100.) {
                self.save_sliders_if_unsaved();
            } else if self.sliders.k < 100. {
                self.restore_sliders_if_saved();
            }
            self.set_cur_color(Cmyk::new(
                self.sliders.c / 100.,
                self.sliders.m / 100.,
                self.sliders.y / 100.,
                self.sliders.k / 100.,
            ));
            true
        } else {
            false
        }
    }

    fn hsv_changed(&mut self) -> bool {
        let hsv = Hsv::from(self.current_color);
        if !math::eq_f32(self.sliders.hue, hsv.h_scaled())
            || !math::eq_f32(self.sliders.sat, hsv.s_scaled())
            || !math::eq_f32(self.sliders.val, hsv.v_scaled())
        {
            if self.sliders.val == 0. {
                self.save_sliders_if_unsaved();
            } else if self.sliders.val > 0. {
                self.restore_sliders_if_saved();
            }
            self.set_cur_color(Hsva::new(
                self.sliders.hue / 360.,
                self.sliders.sat / 100.,
                self.sliders.val / 100.,
                1.,
            ));
            true
        } else {
            false
        }
    }

    fn hsl_changed(&mut self) -> bool {
        let hsl = Hsl::from(self.current_color);
        if !math::eq_f32(self.sliders.hsl_h, hsl.h_scaled())
            || !math::eq_f32(self.sliders.hsl_s, hsl.s_scaled())
            || !math::eq_f32(self.sliders.hsl_l, hsl.l_scaled())
        {
            self.set_cur_color(Hsl::new(
                self.sliders.hsl_h / 360.,
                self.sliders.hsl_s / 100.,
                self.sliders.hsl_l / 100.,
            ));
            true
        } else {
            false
        }
    }

    fn color_changed(&mut self) -> bool {
        if self.rgb_changed() {
            return true;
        }
        if self.cmyk_changed() {
            return true;
        }
        if self.hsv_changed() {
            return true;
        }
        if self.hsl_changed() {
            return true;
        }
        self.rgb_changed()
    }

    pub fn check_for_change(&mut self) {
        self.color_changed();
    }

    pub fn rgb_sliders(&mut self, ui: &mut Ui) {
        let opaque = self.current_color.rgb();
        CollapsingHeader::new("RGB")
            .default_open(false)
            .show(ui, |ui| {
                Grid::new("RGB sliders")
                    .spacing((8., 8.))
                    .show(ui, |mut ui| {
                        slider!(int self, ui, r, "red", U8_MIN..=U8_MAX, |mut r| {
                            r /= U8_MAX;
                            Rgb::new(r, opaque.g(), opaque.b()).into()
                        });
                        ui.end_row();
                        slider!(int self, ui, g, "green", U8_MIN..=U8_MAX, |mut g| {
                            g /= U8_MAX;
                            Rgb::new(opaque.r(), g, opaque.b()).into()
                        });
                        ui.end_row();
                        slider!(int self, ui, b, "blue", U8_MIN..=U8_MAX, |mut b| {
                            b /= U8_MAX;
                            Rgb::new(opaque.r(), opaque.g(), b).into()
                        });
                        ui.end_row();
                    });
            });
    }

    pub fn cmyk_sliders(&mut self, ui: &mut Ui) {
        let opaque = self.current_color.cmyk();
        CollapsingHeader::new("CMYK")
            .default_open(false)
            .show(ui, |ui| {
                Grid::new("CMYK sliders")
                    .spacing((8., 8.))
                    .show(ui, |mut ui| {
                        slider!(self, ui, c, "cyan", 0. ..=100., |mut c| {
                            c /= 100.;
                            Cmyk::new(c, opaque.m(), opaque.y(), opaque.k()).into()
                        });
                        ui.end_row();
                        slider!(self, ui, m, "magenta", 0. ..=100., |mut m| {
                            m /= 100.;
                            Cmyk::new(opaque.c(), m, opaque.y(), opaque.k()).into()
                        });
                        ui.end_row();
                        slider!(self, ui, y, "yellow", 0. ..=100., |mut y| {
                            y /= 100.;
                            Cmyk::new(opaque.c(), opaque.m(), y, opaque.k()).into()
                        });
                        ui.end_row();
                        slider!(self, ui, k, "key", 0. ..=100., |mut k| {
                            k /= 100.;
                            Cmyk::new(opaque.c(), opaque.m(), opaque.y(), k).into()
                        });
                        ui.end_row();
                    });
            });
    }

    pub fn hsv_sliders(&mut self, ui: &mut Ui) {
        let opaque = self.current_color.hsv();
        CollapsingHeader::new("HSV")
            .default_open(false)
            .show(ui, |ui| {
                Grid::new("HSV sliders")
                    .spacing((8., 8.))
                    .show(ui, |mut ui| {
                        slider!(self, ui, hue, "hue", 0. ..=360., |mut h| {
                            h /= 360.;
                            Hsv::new(h, opaque.s(), opaque.v()).into()
                        });
                        ui.end_row();
                        slider!(self, ui, sat, "saturation", 0. ..=100., |mut s| {
                            s /= 100.;
                            Hsv::new(opaque.h(), s, opaque.v()).into()
                        });
                        ui.end_row();
                        slider!(self, ui, val, "value", 0. ..=100., |mut v| {
                            v /= 100.;
                            Hsv::new(opaque.h(), opaque.s(), v).into()
                        });
                        ui.end_row();
                    });
                slider_2d::color(
                    ui,
                    &mut self.sliders.sat,
                    &mut self.sliders.val,
                    0.0..=100.,
                    0.0..=100.,
                    |mut s, mut v| {
                        s /= 100.;
                        v /= 100.;
                        Hsv::new(opaque.h(), s, v).into()
                    },
                )
            });
    }

    pub fn hsl_sliders(&mut self, ui: &mut Ui) {
        let opaque = self.current_color.hsl();
        CollapsingHeader::new("HSL")
            .default_open(false)
            .show(ui, |ui| {
                Grid::new("HSL sliders")
                    .spacing((8., 8.))
                    .show(ui, |mut ui| {
                        slider!(self, ui, hsl_h, "hue", 0. ..=360., |mut h| {
                            h /= 360.;
                            Hsl::new(h, opaque.s(), opaque.l()).into()
                        });
                        ui.end_row();
                        slider!(self, ui, hsl_s, "saturation", 0. ..=100., |mut s| {
                            s /= 100.;
                            Hsl::new(opaque.h(), s, opaque.l()).into()
                        });
                        ui.end_row();
                        slider!(self, ui, hsl_l, "light", 0. ..=100., |mut l| {
                            l /= 100.;
                            Hsl::new(opaque.h(), opaque.s(), l).into()
                        });
                        ui.end_row();
                    });
            });
    }
}
