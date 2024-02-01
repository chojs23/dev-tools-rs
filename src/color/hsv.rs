#![allow(clippy::many_single_char_names)]
use crate::{
    color::{rgb::Rgb, CIEColor, Cmyk, Color, Hsl, Xyz},
    math,
};
use eframe::{
    egui::{Color32, Rgba},
    epaint::Hsva,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub struct Hsv {
    h: f32,
    s: f32,
    v: f32,
}

impl Hsv {
    /// Takes in values in the range of 0.0 ..= 1.0 and returns a HSV color.
    pub fn new(hue: f32, saturation: f32, value: f32) -> Self {
        let hue = if hue.is_nan() { 0. } else { hue };
        let saturation = if saturation.is_nan() { 0. } else { saturation };
        let value = if value.is_nan() { 0. } else { value };
        Self {
            h: hue,
            s: saturation,
            v: value,
        }
    }

    #[inline(always)]
    /// Returns Hue in the range of 0.0 ..= 1.0
    pub fn h(&self) -> f32 {
        self.h
    }

    #[inline(always)]
    /// Returns Saturation in the range of 0.0 ..= 1.0
    pub fn s(&self) -> f32 {
        self.s
    }

    #[inline(always)]
    /// Returns Value in the range of 0.0 ..= 1.0
    pub fn v(&self) -> f32 {
        self.v
    }

    /// Returns Hue in the range of 0.0 ..= 360.0
    pub fn h_scaled(&self) -> f32 {
        self.h * 360.
    }

    /// Returns Saturation in the range of 0.0 ..= 100.0
    pub fn s_scaled(&self) -> f32 {
        self.s * 100.
    }

    /// Returns Value in the range of 0.0 ..= 100.0
    pub fn v_scaled(&self) -> f32 {
        self.v * 100.
    }

    pub fn offset_hue(&mut self, offset: f32) {
        self.h = math::wrap_f32(self.h + offset);
    }

    pub fn offset_saturation(&mut self, offset: f32) {
        self.s = math::wrap_f32(self.s + offset);
    }
}

//####################################################################################################

impl From<Hsv> for Color32 {
    fn from(color: Hsv) -> Self {
        Rgb::from(color).into()
    }
}

impl From<Color32> for Hsv {
    fn from(color: Color32) -> Self {
        Rgb::from(color).into()
    }
}

impl From<Hsv> for Hsva {
    fn from(hsv: Hsv) -> Self {
        Hsva {
            h: hsv.h(),
            s: hsv.s(),
            v: hsv.v(),
            a: 1.,
        }
    }
}

impl From<Hsva> for Hsv {
    fn from(hsv: Hsva) -> Self {
        Self::new(hsv.h, hsv.s, hsv.v)
    }
}

impl From<Hsv> for Rgba {
    fn from(color: Hsv) -> Self {
        Rgb::from(color).into()
    }
}

impl From<Rgba> for Hsv {
    fn from(color: Rgba) -> Self {
        Rgb::from(color).into()
    }
}

//####################################################################################################

impl From<Color> for Hsv {
    fn from(c: Color) -> Hsv {
        match c {
            Color::Rgb(c) => c.into(),
            Color::Cmyk(c) => Rgb::from(c).into(),
            Color::Hsv(c) => c,
            Color::Hsl(c) => c.into(),
            Color::Xyz(c, ws) => c.to_rgb(ws).into(),
            Color::xyY(c, ws) => Xyz::from(c).to_rgb(ws).into(),
            Color::Luv(c, ws) => Xyz::from(c).to_rgb(ws).into(),
            Color::LchUV(c, ws) => Xyz::from(c).to_rgb(ws).into(),
            Color::Lab(c, ws, illuminant) => c.to_xyz(illuminant).to_rgb(ws).into(),
            Color::LchAB(c, ws, illuminant) => c.to_xyz(illuminant).to_rgb(ws).into(),
            Color::Color32(c) => Rgb::from(c).into(),
        }
    }
}

impl From<&Color> for Hsv {
    fn from(c: &Color) -> Self {
        (*c).into()
    }
}

impl From<Cmyk> for Hsv {
    fn from(color: Cmyk) -> Self {
        Rgb::from(color).into()
    }
}

impl From<Hsl> for Hsv {
    fn from(color: Hsl) -> Self {
        let h = color.h();
        let mut ss = color.s();
        let l = color.l() * 2.;

        if l <= 1. {
            ss *= l;
        } else {
            ss *= 2. - l;
        }

        let v = (l + ss) / 2.;
        let s = (2. * ss) / (l + ss);

        Hsv::new(h, s, v)
    }
}

impl From<Rgb> for Hsv {
    fn from(rgb: Rgb) -> Self {
        let r = rgb.r();
        let g = rgb.g();
        let b = rgb.b();
        let max = r.max(g.max(b));
        let min = r.min(g.min(b));
        let delta = max - min;

        let h = if delta == 0. {
            0.
        } else if math::eq_f32(max, r) {
            (g - b) / (delta * 6.)
        } else if math::eq_f32(max, g) {
            1. / 3. + (b - r) / (delta * 6.)
        } else {
            2. / 3. + (r - g) / (delta * 6.)
        };

        let h = if h < 0. { math::wrap_f32(h) } else { h }; // wrap

        let v = max;
        let s = if v == 0. { 0. } else { 1. - min / max };

        Hsv::new(h, s, v)
    }
}

#[cfg(test)]
mod tests {
    use super::{Hsl, Hsv, Rgb};
    #[test]
    fn rgb_to_hsv() {
        macro_rules! test_case {
            (Rgb: $r:expr, $g:expr, $b:expr ;Hsv: $h:expr, $s:expr, $v:expr) => {
                let expected = Hsv::new($h, $s, $v);
                let rgb = Rgb::new($r, $g, $b);
                let got = Hsv::from(rgb);
                assert_eq!(got, expected);
            };
        }

        test_case!(Rgb: 0., 0., 0.; Hsv: 0., 0., 0.);
        test_case!(Rgb: 1., 0., 0.; Hsv: 0., 1., 1.);
        test_case!(Rgb: 1., 1., 0.; Hsv: 1./6., 1., 1.);
        test_case!(Rgb: 0.5, 1., 0.; Hsv: 1./4., 1., 1.);
        test_case!(Rgb: 0., 1., 0.; Hsv: 1./3., 1., 1.);
        test_case!(Rgb: 0., 1., 1.; Hsv: 1./2., 1., 1.);
        test_case!(Rgb: 0., 0., 1.; Hsv: 2./3., 1., 1.);
        test_case!(Rgb: 0.5, 0., 1.; Hsv: 3./4., 1., 1.);
        test_case!(Rgb: 1., 0., 1.; Hsv: 5./6., 1., 1.);
        test_case!(Rgb: 1., 0., 0.; Hsv: 0., 1., 1.);
    }
    #[test]
    fn hsl_to_hsv() {
        macro_rules! test_case {
            (Hsl: $hh:expr, $ss:expr, $l:expr ;Hsv: $h:expr, $s:expr, $v:expr) => {
                let expected = Hsv::new($h, $s, $v);
                let hsl = Hsl::new($hh, $ss, $l);
                let got = Hsv::from(hsl);
                assert_eq!(got, expected);
            };
        }

        test_case!(Hsl: 0., 0., 0.; Hsv: 0., 0., 0.);
        test_case!(Hsl: 0.5, 0., 0.; Hsv: 0.5, 0., 0.);
        test_case!(Hsl: 1., 0., 0.; Hsv: 1., 0., 0.);
        test_case!(Hsl: 0., 0.5, 0.; Hsv: 0., 0., 0.);
        test_case!(Hsl: 0., 1., 0.; Hsv: 0., 0., 0.);
        test_case!(Hsl: 0., 0., 0.5; Hsv: 0., 0., 0.5);
        test_case!(Hsl: 0., 0., 1.; Hsv: 0., 0., 1.);
        test_case!(Hsl: 0., 0.5, 0.5; Hsv: 0., 2./3., 3./4.);
        test_case!(Hsl: 0., 1., 1.; Hsv: 0., 0., 1.);
        test_case!(Hsl: 1./4., 1./4., 1./4.; Hsv: 1./4., 2./5., 0.3125);
        test_case!(Hsl: 1./3., 1./3., 1./3.; Hsv: 1./3., 1./2., 0.44444448);
        test_case!(Hsl: 1./2., 1./2., 1./2.; Hsv: 1./2., 2./3., 3./4.);
        test_case!(Hsl: 2./3., 2./3., 2./3.; Hsv: 2./3., 0.49999997, 0.8888889);
        test_case!(Hsl: 3./4., 3./4., 3./4.; Hsv: 3./4., 2./5., 0.9375);
        test_case!(Hsl: 1., 1., 1.; Hsv: 1., 0., 1.);
        test_case!(Hsl: 1./4., 0.14285715, 0.21875; Hsv: 1./4., 1./4., 1./4.);
        test_case!(Hsl: 1./3., 1./5., 0.2777778; Hsv: 1./3., 1./3., 1./3.);
        test_case!(Hsl: 1./2., 1./3., 3./8.; Hsv: 1./2., 1./2., 1./2.);
        test_case!(Hsl: 2./3., 1./2., 0.44444442; Hsv: 2./3., 2./3., 0.6666666);
        test_case!(Hsl: 3./4., 3./5., 0.46875; Hsv: 3./4., 3./4., 3./4.);
        test_case!(Hsl: 1., 1., 0.5; Hsv: 1., 1., 1.);
    }
}
