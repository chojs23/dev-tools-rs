mod sliders;

use crate::{
    color::{
        CIEColor, Cmyk, Color, Hsl, Hsv, Illuminant, Lab, LchAB, LchUV, Luv, Rgb, RgbWorkingSpace,
        Xyz, U8_MAX, U8_MIN,
    },
    math,
    ui::{slider_1d, slider_2d},
};
use sliders::ColorSliders;

use eframe::{
    egui::{CollapsingHeader, DragValue, Grid, Ui},
    epaint::Hsva,
};
use serde::{Deserialize, Serialize};
use std::mem;

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
    pub new_workspace: Option<RgbWorkingSpace>,
    pub new_illuminant: Option<Illuminant>,
}

impl Default for ColorPicker {
    fn default() -> Self {
        Self {
            current_color: Color::black(),
            hex_color: "".to_string(),
            sliders: ColorSliders::default(),
            saved_sliders: None,
            new_workspace: None,
            new_illuminant: None,
        }
    }
}

impl ColorPicker {
    pub fn set_cur_color(&mut self, color: impl Into<Color>) {
        let color = color.into();
        self.sliders.set_color(color);
        self.current_color = color;
    }

    pub fn set_cie_color(&mut self, color: impl CIEColor) {
        let color = color.to_rgb(self.sliders.rgb_working_space).into();
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

    fn luv_changed(&mut self) -> bool {
        let luv = Luv::from(self.current_color.xyz(self.sliders.rgb_working_space));
        if !math::eq_f32(self.sliders.luv_l, luv.l())
            || !math::eq_f32(self.sliders.luv_u, luv.u())
            || !math::eq_f32(self.sliders.luv_v, luv.v())
        {
            self.set_cie_color(Xyz::from(Luv::new(
                self.sliders.luv_l,
                self.sliders.luv_u,
                self.sliders.luv_v,
            )));
            true
        } else {
            false
        }
    }

    fn lch_uv_changed(&mut self) -> bool {
        let lch = LchUV::from(self.current_color.xyz(self.sliders.rgb_working_space));
        if !math::eq_f32(self.sliders.lch_uv_l, lch.l())
            || !math::eq_f32(self.sliders.lch_uv_c, lch.c())
            || !math::eq_f32(self.sliders.lch_uv_h, lch.h())
        {
            self.set_cie_color(Xyz::from(LchUV::new(
                self.sliders.lch_uv_l,
                self.sliders.lch_uv_c,
                self.sliders.lch_uv_h,
            )));
            true
        } else {
            false
        }
    }

    fn lab_changed(&mut self) -> bool {
        let lab = self.current_color.lab(
            self.sliders.rgb_working_space,
            self.sliders.illuminant,
            self.sliders.chromatic_adaptation_method,
        );
        if !math::eq_f32(self.sliders.lab_l, lab.l())
            || !math::eq_f32(self.sliders.lab_a, lab.a())
            || !math::eq_f32(self.sliders.lab_b, lab.b())
        {
            let xyz = Lab::new(self.sliders.lab_l, self.sliders.lab_a, self.sliders.lab_b)
                .to_xyz(self.sliders.illuminant);
            self.set_cie_color(xyz);
            true
        } else {
            false
        }
    }

    fn lch_ab_changed(&mut self) -> bool {
        let lch = self.current_color.lch_ab(
            self.sliders.rgb_working_space,
            self.sliders.illuminant,
            self.sliders.chromatic_adaptation_method,
        );
        if !math::eq_f32(self.sliders.lch_ab_l, lch.l())
            || !math::eq_f32(self.sliders.lch_ab_c, lch.c())
            || !math::eq_f32(self.sliders.lch_ab_h, lch.h())
        {
            self.set_cie_color(
                LchAB::new(
                    self.sliders.lch_ab_l,
                    self.sliders.lch_ab_c,
                    self.sliders.lch_ab_h,
                )
                .to_xyz(self.sliders.illuminant),
            );
            true
        } else {
            false
        }
    }

    fn workspace_changed(&mut self) -> bool {
        if let Some(ws) = mem::take(&mut self.new_workspace) {
            self.sliders.rgb_working_space = ws;
            self.set_cur_color(Rgb::new(
                self.sliders.r / U8_MAX,
                self.sliders.g / U8_MAX,
                self.sliders.b / U8_MAX,
            ));
            return true;
        }
        false
    }

    fn illuminant_changed(&mut self) -> bool {
        if let Some(illuminant) = mem::take(&mut self.new_illuminant) {
            self.sliders.illuminant = illuminant;
            self.set_cur_color(Rgb::new(
                self.sliders.r / U8_MAX,
                self.sliders.g / U8_MAX,
                self.sliders.b / U8_MAX,
            ));
            return true;
        }
        false
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
        if self.luv_changed() {
            return true;
        }
        if self.lch_uv_changed() {
            return true;
        }
        if self.lab_changed() {
            return true;
        }
        self.lch_ab_changed()
    }

    pub fn check_for_change(&mut self) {
        if self.workspace_changed() {
            return;
        }
        if self.illuminant_changed() {
            return;
        }
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

    pub fn luv_sliders(&mut self, ui: &mut Ui) {
        let ws = self.sliders.rgb_working_space;
        let opaque = self.current_color.luv(ws);
        CollapsingHeader::new("Luv")
            .default_open(false)
            .show(ui, |ui| {
                Grid::new("Luv sliders")
                    .spacing((8., 8.))
                    .show(ui, |mut ui| {
                        slider!(self, ui, luv_l, "light", 0. ..=100., |l| {
                            Xyz::from(Luv::new(l, opaque.u(), opaque.v()))
                                .to_rgb(ws)
                                .into()
                        });
                        ui.end_row();
                        slider!(self, ui, luv_u, "u", -134. ..=220., |u| {
                            Xyz::from(Luv::new(opaque.l(), u, opaque.v()))
                                .to_rgb(ws)
                                .into()
                        });
                        ui.end_row();
                        slider!(self, ui, luv_v, "v", -140. ..=122., |v| {
                            Xyz::from(Luv::new(opaque.l(), opaque.u(), v))
                                .to_rgb(ws)
                                .into()
                        });
                        ui.end_row();
                    });
            });
    }

    pub fn lch_uv_sliders(&mut self, ui: &mut Ui) {
        let ws = self.sliders.rgb_working_space;
        let opaque = self.current_color.lch_uv(ws);
        CollapsingHeader::new("LCH(uv)")
            .default_open(false)
            .show(ui, |ui| {
                Grid::new("LCH(uv) sliders")
                    .spacing((8., 8.))
                    .show(ui, |mut ui| {
                        slider!(self, ui, lch_uv_l, "light", 0. ..=100., |l| {
                            Xyz::from(LchUV::new(l, opaque.c(), opaque.h()))
                                .to_rgb(ws)
                                .into()
                        });
                        ui.end_row();
                        slider!(self, ui, lch_uv_c, "c", 0. ..=270., |c| {
                            Xyz::from(LchUV::new(opaque.l(), c, opaque.h()))
                                .to_rgb(ws)
                                .into()
                        });
                        ui.end_row();
                        slider!(self, ui, lch_uv_h, "h", 0. ..=360., |h| {
                            Xyz::from(LchUV::new(opaque.l(), opaque.c(), h))
                                .to_rgb(ws)
                                .into()
                        });
                        ui.end_row();
                    });
            });
    }

    pub fn lab_sliders(&mut self, ui: &mut Ui) {
        let ws = self.sliders.rgb_working_space;
        let ref_white = self.sliders.illuminant;
        let opaque =
            self.current_color
                .lab(ws, ref_white, self.sliders.chromatic_adaptation_method);

        CollapsingHeader::new("Lab")
            .default_open(false)
            .show(ui, |ui| {
                Grid::new("Lab sliders")
                    .spacing((8., 8.))
                    .show(ui, |mut ui| {
                        slider!(self, ui, lab_l, "light", 0. ..=100., |l| {
                            Lab::new(l, opaque.a(), opaque.b())
                                .to_xyz(ref_white)
                                .to_rgb(ws)
                                .into()
                        });
                        ui.end_row();
                        slider!(self, ui, lab_a, "a", -128. ..=127., |a| {
                            Lab::new(opaque.l(), a, opaque.b())
                                .to_xyz(ref_white)
                                .to_rgb(ws)
                                .into()
                        });
                        ui.end_row();
                        slider!(self, ui, lab_b, "b", -128. ..=127., |b| {
                            Lab::new(opaque.l(), opaque.a(), b)
                                .to_xyz(ref_white)
                                .to_rgb(ws)
                                .into()
                        });
                        ui.end_row();
                    });
            });
    }

    pub fn lch_ab_sliders(&mut self, ui: &mut Ui) {
        let ws = self.sliders.rgb_working_space;
        let ref_white = self.sliders.illuminant;
        let opaque = self.current_color.lch_ab(
            ws,
            self.sliders.illuminant,
            self.sliders.chromatic_adaptation_method,
        );
        CollapsingHeader::new("LCH(ab)")
            .default_open(false)
            .show(ui, |ui| {
                Grid::new("LCH(ab) sliders")
                    .spacing((8., 8.))
                    .show(ui, |mut ui| {
                        slider!(self, ui, lch_ab_l, "light", 0. ..=100., |l| {
                            LchAB::new(l, opaque.c(), opaque.h())
                                .to_xyz(ref_white)
                                .to_rgb(ws)
                                .into()
                        });
                        ui.end_row();
                        slider!(self, ui, lch_ab_c, "c", 0. ..=270., |c| {
                            LchAB::new(opaque.l(), c, opaque.h())
                                .to_xyz(ref_white)
                                .to_rgb(ws)
                                .into()
                        });
                        ui.end_row();
                        slider!(self, ui, lch_ab_h, "h", 0. ..=360., |h| {
                            LchAB::new(opaque.l(), opaque.c(), h)
                                .to_xyz(ref_white)
                                .to_rgb(ws)
                                .into()
                        });
                        ui.end_row();
                    });
            });
    }
}
