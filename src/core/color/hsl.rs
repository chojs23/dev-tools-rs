use super::{hsv::Hsv, rgb::Rgb, CIEColor, Cmyk, Color, Xyz};
use eframe::{
    egui::{Color32, Rgba},
    epaint::Hsva,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub struct Hsl {
    h: f32,
    s: f32,
    l: f32,
}

impl Hsl {
    /// Takes in values in the range 0.0 ..= 1.0 and returns an HSL color
    pub fn new(hue: f32, saturation: f32, light: f32) -> Self {
        let hue = if hue.is_nan() { 0. } else { hue };
        let saturation = if saturation.is_nan() { 0. } else { saturation };
        let light = if light.is_nan() { 0. } else { light };
        Self {
            h: hue,
            s: saturation,
            l: light,
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
    /// Returns Light in the range of 0.0 ..= 1.0
    pub fn l(&self) -> f32 {
        self.l
    }

    /// Returns Hue in the range of 0.0 ..= 360.0
    pub fn h_scaled(&self) -> f32 {
        self.h * 360.
    }

    /// Returns Saturation in the range of 0.0 ..= 100.0
    pub fn s_scaled(&self) -> f32 {
        self.s * 100.
    }

    /// Returns Light in the range of 0.0 ..= 100.0
    pub fn l_scaled(&self) -> f32 {
        self.l * 100.
    }
}

//####################################################################################################

impl From<Hsl> for Color32 {
    fn from(color: Hsl) -> Self {
        Hsv::from(color).into()
    }
}

impl From<Color32> for Hsl {
    fn from(color: Color32) -> Self {
        Hsv::from(color).into()
    }
}

impl From<Hsl> for Hsva {
    fn from(color: Hsl) -> Self {
        Hsv::from(color).into()
    }
}

impl From<Hsva> for Hsl {
    fn from(color: Hsva) -> Self {
        Hsv::from(color).into()
    }
}

impl From<Hsl> for Rgba {
    fn from(color: Hsl) -> Self {
        Hsv::from(color).into()
    }
}

impl From<Rgba> for Hsl {
    fn from(color: Rgba) -> Self {
        Hsv::from(color).into()
    }
}

//####################################################################################################

impl From<Color> for Hsl {
    fn from(c: Color) -> Hsl {
        match c {
            Color::Rgb(c) => c.into(),
            Color::Cmyk(c) => Rgb::from(c).into(),
            Color::Hsv(c) => c.into(),
            Color::Hsl(c) => c,
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

impl From<&Color> for Hsl {
    fn from(c: &Color) -> Self {
        (*c).into()
    }
}

impl From<Cmyk> for Hsl {
    fn from(color: Cmyk) -> Self {
        Hsv::from(color).into()
    }
}

#[allow(clippy::many_single_char_names)]
impl From<Hsv> for Hsl {
    fn from(color: Hsv) -> Self {
        let h = color.h();
        let s = color.s();
        let v = color.v();

        let mut l = (2. - s) * v;
        let mut ss = s * v;
        if l <= 1. {
            ss /= l;
        } else {
            ss /= 2. - l;
        }
        l /= 2.;

        Hsl::new(h, ss, l)
    }
}

impl From<Rgb> for Hsl {
    fn from(rgb: Rgb) -> Hsl {
        Hsv::from(rgb).into()
    }
}

//####################################################################################################

#[cfg(test)]
mod tests {
    use super::{Hsl, Hsv};
    #[test]
    fn hsv_to_hsl() {
        macro_rules! test_case {
            (Hsv: $hh:expr, $ss:expr, $v:expr ;Hsl: $h:expr, $s:expr, $l:expr) => {
                let expected = Hsl::new($h, $s, $l);
                let hsv = Hsv::new($hh, $ss, $v);
                let got = Hsl::from(hsv);
                assert_eq!(got, expected);
            };
        }

        test_case!(Hsv: 0., 0., 0.; Hsl: 0., 0., 0.);
        test_case!(Hsv: 0.5, 0., 0.; Hsl: 0.5, 0., 0.);
        test_case!(Hsv: 1., 0., 0.; Hsl: 1., 0., 0.);
        test_case!(Hsv: 0., 0.5, 0.; Hsl: 0., 0., 0.);
        test_case!(Hsv: 0., 1., 0.; Hsl: 0., 0., 0.);
        test_case!(Hsv: 0., 0., 0.5; Hsl: 0., 0., 0.5);
        test_case!(Hsv: 0., 0., 1.; Hsl: 0., 0., 1.);
        test_case!(Hsv: 0., 0.5, 0.5; Hsl: 0., 1./3., 0.375);
        test_case!(Hsv: 0., 1., 1.; Hsl: 0., 1., 0.5);
        test_case!(Hsv: 1./4., 1./4., 1./4.; Hsl: 1./4., 0.14285715, 0.21875);
        test_case!(Hsv: 1./3., 1./3., 1./3.; Hsl: 1./3., 1./5., 0.2777778);
        test_case!(Hsv: 1./2., 1./2., 1./2.; Hsl: 1./2., 1./3., 3./8.);
        test_case!(Hsv: 2./3., 2./3., 2./3.; Hsl: 2./3., 0.50000006, 0.44444442);
        test_case!(Hsv: 3./4., 3./4., 3./4.; Hsl: 3./4., 3./5., 0.46875);
        test_case!(Hsv: 1., 1., 1.; Hsl: 1., 1., 0.5);
        test_case!(Hsv: 1./4., 2./5., 0.3125; Hsl: 1./4., 1./4., 1./4.);
        test_case!(Hsv: 1./3., 1./2., 0.44444448; Hsl: 1./3., 0.3333333, 0.33333337);
        test_case!(Hsv: 1./2., 2./3., 3./4.; Hsl: 1./2., 0.50000006, 0.49999997);
        test_case!(Hsv: 2./3., 0.49999997, 0.8888889; Hsl: 2./3., 2./3., 2./3.);
        test_case!(Hsv: 3./4., 2./5., 0.9375; Hsl: 3./4., 3./4., 3./4.);
        test_case!(Hsv: 1., 0., 1.; Hsl: 1., 0., 1.);
    }
}
