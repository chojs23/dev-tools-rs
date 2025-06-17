use crate::core::color::{ChromaticAdaptationMethod, Color, Illuminant, RgbWorkingSpace};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ColorSliders {
    pub rgb_working_space: RgbWorkingSpace,
    pub illuminant: Illuminant,
    pub chromatic_adaptation_method: ChromaticAdaptationMethod,
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
        let ws = RgbWorkingSpace::default();
        Self {
            rgb_working_space: ws,
            illuminant: ws.reference_illuminant(),
            chromatic_adaptation_method: ChromaticAdaptationMethod::default(),
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
        let luv = color.luv(self.rgb_working_space);
        self.luv_l = luv.l();
        self.luv_u = luv.u();
        self.luv_v = luv.v();
        let lch_uv = color.lch_uv(self.rgb_working_space);
        self.lch_uv_l = lch_uv.l();
        self.lch_uv_c = lch_uv.c();
        self.lch_uv_h = lch_uv.h();
        let lab = color.lab(
            self.rgb_working_space,
            self.illuminant,
            self.chromatic_adaptation_method,
        );
        self.lab_l = lab.l();
        self.lab_a = lab.a();
        self.lab_b = lab.b();
        let lch_ab = color.lch_ab(
            self.rgb_working_space,
            self.illuminant,
            self.chromatic_adaptation_method,
        );
        self.lch_ab_l = lch_ab.l();
        self.lch_ab_c = lch_ab.c();
        self.lch_ab_h = lch_ab.h();
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
