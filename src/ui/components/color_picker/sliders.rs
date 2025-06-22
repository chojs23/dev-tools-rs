use crate::core::color::Color;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ColorSliders {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub c: f32,
    pub m: f32,
    pub y: f32,
    pub k: f32,
    pub hue: f32,
    pub sat: f32,
    pub val: f32,
    pub hsl_h: f32,
    pub hsl_s: f32,
    pub hsl_l: f32,
    pub luv_l: f32,
    pub luv_u: f32,
    pub luv_v: f32,
    pub lch_uv_l: f32,
    pub lch_uv_c: f32,
    pub lch_uv_h: f32,
    pub lab_l: f32,
    pub lab_a: f32,
    pub lab_b: f32,
    pub lch_ab_l: f32,
    pub lch_ab_c: f32,
    pub lch_ab_h: f32,
}

impl Default for ColorSliders {
    fn default() -> Self {
        Self {
            r: 0.,
            g: 0.,
            b: 0.,
            c: 0.,
            m: 0.,
            y: 0.,
            k: 100.,
            hue: 0.,
            sat: 0.,
            val: 0.,
            hsl_h: 0.,
            hsl_s: 0.,
            hsl_l: 0.,
            luv_l: 0.,
            luv_u: 0.,
            luv_v: 0.,
            lch_uv_l: 0.,
            lch_uv_c: 0.,
            lch_uv_h: 180.,
            lab_l: 0.,
            lab_a: 0.,
            lab_b: 0.,
            lch_ab_l: 0.,
            lch_ab_c: 0.,
            lch_ab_h: 0.,
        }
    }
}

impl ColorSliders {
    pub fn set_color(&mut self, color: Color) {
        let rgb = color.rgb();
        self.r = rgb.r_scaled();
        self.g = rgb.g_scaled();
        self.b = rgb.b_scaled();
        let cmyk = color.cmyk();
        self.c = cmyk.c_scaled();
        self.m = cmyk.m_scaled();
        self.y = cmyk.y_scaled();
        self.k = cmyk.k_scaled();
        let hsv = color.hsv();
        self.hue = hsv.h_scaled();
        self.sat = hsv.s_scaled();
        self.val = hsv.v_scaled();
        let hsl = color.hsl();
        self.hsl_h = hsl.h_scaled();
        self.hsl_s = hsl.s_scaled();
        self.hsl_l = hsl.l_scaled();
    }

    pub fn restore(&mut self, other: Self) {
        self.r = other.r;
        self.g = other.g;
        self.b = other.b;
        self.c = other.c;
        self.m = other.m;
        self.y = other.y;
        self.k = other.k;
        self.hue = other.hue;
        self.sat = other.sat;
        self.val = other.val;
        self.hsl_h = other.hsl_h;
        self.hsl_s = other.hsl_s;
        self.hsl_l = other.hsl_l;
        self.luv_l = other.luv_l;
        self.luv_u = other.luv_u;
        self.luv_v = other.luv_v;
        self.lch_uv_l = other.lch_uv_l;
        self.lch_uv_c = other.lch_uv_c;
        self.lch_uv_h = other.lch_uv_h;
        self.lab_l = other.lab_l;
        self.lab_a = other.lab_a;
        self.lab_b = other.lab_b;
        self.lch_ab_l = other.lch_ab_l;
        self.lch_ab_c = other.lch_ab_c;
        self.lch_ab_h = other.lch_ab_h;
    }
}
