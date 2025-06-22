mod cmyk;
mod format;
pub mod gradient;
mod hsl;
mod hsv;
mod palette;
pub mod palettes;
mod rgb;

pub use cmyk::Cmyk;
pub use hsl::Hsl;
pub use hsv::Hsv;
pub use rgb::Rgb;

use eframe::{
    egui::{Color32, Rgba},
    epaint::{Hsva, HsvaGamma},
};
use serde::{Deserialize, Serialize};

pub const CIE_E: f32 = 216. / 24389.;
pub const CIE_K: f32 = 24389. / 27.;
pub const U8_MAX: f32 = u8::MAX as f32;
pub const U8_MIN: f32 = u8::MIN as f32;

//################################################################################

const fn hex_val(ch: u8) -> u8 {
    match ch {
        b'0'..=b'9' => ch - 48,
        b'A'..=b'F' => ch - 55,
        b'a'..=b'f' => ch - 87,
        _ => 0,
    }
}

const fn hex_chars_to_u8(ch: (u8, u8)) -> u8 {
    let mut result = 0;
    result |= hex_val(ch.0);
    result <<= 4;
    result |= hex_val(ch.1);
    result
}

pub fn parse_hex(color: &str) -> Option<(u8, u8, u8)> {
    if color.len() != 6 {
        return None;
    }
    let mut bytes = color.as_bytes().chunks(2);

    Some((
        bytes.next().map(|arr| hex_chars_to_u8((arr[0], arr[1])))?,
        bytes.next().map(|arr| hex_chars_to_u8((arr[0], arr[1])))?,
        bytes.next().map(|arr| hex_chars_to_u8((arr[0], arr[1])))?,
    ))
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum ColorFormat {
    #[serde(rename = "hex")]
    Hex,
    #[serde(rename = "hex-uppercase")]
    HexUpercase,
    #[serde(rename = "css-rgb")]
    CssRgb,
    #[serde(rename = "css-hsl")]
    CssHsl { degree_symbol: bool },
}

impl ColorFormat {
    pub fn no_degree(self) -> Self {
        use ColorFormat::*;
        match self {
            CssHsl { .. } => CssHsl {
                degree_symbol: false,
            },
            fmt => fmt,
        }
    }
}

//################################################################################

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub enum Color {
    Rgb(Rgb),
    Cmyk(Cmyk),
    Hsv(Hsv),
    Hsl(Hsl),
    Color32(Color32),
}

impl Color {
    pub fn black() -> Self {
        Self::Rgb(Rgb::new(0., 0., 0.))
    }

    pub fn white() -> Self {
        Self::Rgb(Rgb::new(1., 1., 1.))
    }

    pub fn intensity(&self) -> f32 {
        let rgb = self.rgb();
        0.215 * rgb.r() + 0.7 * rgb.g() + 0.085 * rgb.b()
    }

    pub fn contrast(&self) -> Color {
        if self.intensity() > 0.5 {
            Self::black()
        } else {
            Self::white()
        }
    }

    pub fn as_hex(&self) -> String {
        let color = self.as_rgb_triplet_scaled();
        format!("#{:02x}{:02x}{:02x}", color.0, color.1, color.2)
    }

    pub fn as_css_rgb(&self) -> String {
        let color = self.as_rgb_triplet_scaled();
        format!("rgb({},{},{})", color.0, color.1, color.2)
    }

    pub fn as_css_rgb_padded(&self) -> String {
        let color = self.as_rgb_triplet_scaled();
        format!("rgb({:>3},{:>3},{:>3})", color.0, color.1, color.2)
    }

    pub fn as_css_hsl(&self, degree_symbol: bool) -> String {
        let color = self.hsl();
        format!(
            "hsl({}{},{}%,{}%)",
            color.h_scaled() as u16,
            if degree_symbol { "°" } else { "" },
            color.s_scaled() as u16,
            color.l_scaled() as u16
        )
    }

    pub fn as_css_hsl_padded(&self, degree_symbol: bool) -> String {
        let color = self.hsl();
        format!(
            "hsl({:>3}{},{:>3}%,{:>3}%)",
            color.h_scaled() as u16,
            if degree_symbol { "°" } else { "" },
            color.s_scaled() as u16,
            color.l_scaled() as u16
        )
    }

    pub fn display(&self, format: ColorFormat) -> String {
        match format {
            ColorFormat::Hex => self.as_hex(),
            ColorFormat::HexUpercase => self.as_hex().to_uppercase(),
            ColorFormat::CssRgb => self.as_css_rgb(),
            ColorFormat::CssHsl { degree_symbol } => self.as_css_hsl(degree_symbol),
        }
    }

    pub fn from_hex(hex: &str) -> Option<Self> {
        parse_hex(hex).map(|(r, g, b)| Rgb::new_scaled(r, g, b).into())
    }

    pub fn as_hue_offset(&self, offset: f32) -> Color {
        let mut hsv = self.hsv();
        hsv.offset_hue(offset);
        Self::Hsv(hsv)
    }

    pub fn as_saturation_offset(&self, offset: f32) -> Color {
        let mut hsv = self.hsv();
        hsv.offset_saturation(offset);
        Self::Hsv(hsv)
    }

    pub fn as_rgb_triplet_scaled(&self) -> (u8, u8, u8) {
        let color = self.rgb();
        (
            color.r_scaled().floor() as u8,
            color.g_scaled().floor() as u8,
            color.b_scaled().floor() as u8,
        )
    }

    pub fn as_rgb_triplet(&self) -> (f32, f32, f32) {
        let color = self.rgb();
        (color.r(), color.g(), color.b())
    }

    pub fn color32(&self) -> Color32 {
        self.into()
    }

    pub fn hsva(&self) -> Hsva {
        self.into()
    }

    pub fn rgba(&self) -> Rgba {
        self.into()
    }

    pub fn cmyk(&self) -> Cmyk {
        self.into()
    }

    pub fn hsl(&self) -> Hsl {
        self.into()
    }

    pub fn hsv(&self) -> Hsv {
        self.into()
    }

    pub fn rgb(&self) -> Rgb {
        self.into()
    }

    pub fn shades(&self, total: u8) -> Vec<Color> {
        if total == 0 {
            return vec![*self];
        }
        let mut step_total = total.saturating_sub(1) as f32;
        if step_total == 0. {
            step_total = 1.;
        }

        let rgb = self.rgb();
        let mut base_r = rgb.r_scaled() as u8;
        let mut base_g = rgb.g_scaled() as u8;
        let mut base_b = rgb.b_scaled() as u8;
        let step_r = (base_r as f32 / step_total).ceil() as u8;
        let step_g = (base_g as f32 / step_total).ceil() as u8;
        let step_b = (base_b as f32 / step_total).ceil() as u8;

        (0..total)
            .map(|_| {
                let c = Color32::from_rgb(base_r, base_g, base_b);
                base_r = base_r.saturating_sub(step_r);
                base_g = base_g.saturating_sub(step_g);
                base_b = base_b.saturating_sub(step_b);
                c.into()
            })
            .collect()
    }

    pub fn tints(&self, total: u8) -> Vec<Color> {
        if total == 0 {
            return vec![*self];
        }
        let mut step_total = total.saturating_sub(1) as f32;
        if step_total == 0. {
            step_total = 1.;
        }
        let rgb = self.rgb();
        let mut base_r = rgb.r_scaled() as u8;
        let mut base_g = rgb.g_scaled() as u8;
        let mut base_b = rgb.b_scaled() as u8;
        let step_r = ((U8_MAX - base_r as f32) / step_total).ceil() as u8;
        let step_g = ((U8_MAX - base_g as f32) / step_total).ceil() as u8;
        let step_b = ((U8_MAX - base_b as f32) / step_total).ceil() as u8;

        (0..total)
            .map(|_| {
                let c = Color32::from_rgb(base_r, base_g, base_b);
                base_r = base_r.saturating_add(step_r);
                base_g = base_g.saturating_add(step_g);
                base_b = base_b.saturating_add(step_b);
                c.into()
            })
            .collect()
    }

    pub fn hues(&self, total: u8, step: f32) -> Vec<Color> {
        let mut colors = Vec::new();
        let hsv = self.hsv();
        for i in (0..=total).rev() {
            let mut _h = hsv;
            _h.offset_hue(-1. * step * i as f32);
            colors.push(_h.into());
        }

        for i in 1..=total {
            let mut _h = hsv;
            _h.offset_hue(1. * step * i as f32);
            colors.push(_h.into());
        }

        colors
    }

    pub fn complementary(&self) -> Color {
        let white = Self::white();
        let black = Self::black();

        if self == &black {
            return white;
        } else if self == &white {
            return black;
        }

        self.as_hue_offset(6. / 12.)
    }

    pub fn triadic(&self) -> (Color, Color) {
        (self.as_hue_offset(4. / 12.), self.as_hue_offset(8. / 12.))
    }

    pub fn tetradic(&self) -> (Color, Color, Color) {
        (
            self.as_hue_offset(2. / 12.),
            self.complementary(),
            self.as_hue_offset(8. / 12.),
        )
    }

    pub fn analogous(&self) -> (Color, Color) {
        (self.as_hue_offset(-1. / 12.), self.as_hue_offset(1. / 12.))
    }

    pub fn split_complementary(&self) -> (Color, Color) {
        (self.as_hue_offset(5. / 12.), self.as_hue_offset(7. / 12.))
    }

    pub fn square(&self) -> (Color, Color, Color) {
        (
            self.as_hue_offset(3. / 12.),
            self.complementary(),
            self.as_hue_offset(9. / 12.),
        )
    }
}

//##################################################################################################

impl From<&Color> for Color32 {
    fn from(c: &Color) -> Self {
        (*c).into()
    }
}

impl From<&Color> for Rgba {
    fn from(c: &Color) -> Self {
        (*c).into()
    }
}

impl From<&Color> for Hsva {
    fn from(c: &Color) -> Self {
        (*c).into()
    }
}

impl From<Color> for Color32 {
    fn from(c: Color) -> Color32 {
        match c {
            Color::Rgb(c) => c.into(),
            Color::Cmyk(c) => c.into(),
            Color::Hsv(c) => c.into(),
            Color::Hsl(c) => c.into(),
            Color::Color32(c) => c,
        }
    }
}

impl From<Color32> for Color {
    fn from(c: Color32) -> Color {
        Color::Color32(c)
    }
}

impl From<Rgba> for Color {
    fn from(c: Rgba) -> Color {
        Color::Rgb(c.into())
    }
}

impl From<Hsva> for Color {
    fn from(c: Hsva) -> Color {
        Color::Hsv(c.into())
    }
}

macro_rules! to_epaint_color {
    ($c:ident) => {
        match $c {
            Color::Rgb(c) => c.into(),
            Color::Cmyk(c) => Rgb::from(c).into(),
            Color::Hsv(c) => Rgb::from(c).into(),
            Color::Hsl(c) => Rgb::from(c).into(),
            Color::Color32(c) => c.into(),
        }
    };
}

impl From<Color> for Rgba {
    fn from(c: Color) -> Rgba {
        to_epaint_color!(c)
    }
}

impl From<Color> for Hsva {
    fn from(c: Color) -> Hsva {
        to_epaint_color!(c)
    }
}

impl From<Color> for HsvaGamma {
    fn from(c: Color) -> HsvaGamma {
        to_epaint_color!(c)
    }
}

//##################################################################################################

impl From<Cmyk> for Color {
    fn from(c: Cmyk) -> Color {
        Color::Cmyk(c)
    }
}

impl From<Hsl> for Color {
    fn from(c: Hsl) -> Color {
        Color::Hsl(c)
    }
}

impl From<Hsv> for Color {
    fn from(c: Hsv) -> Color {
        Color::Hsv(c)
    }
}

impl From<Rgb> for Color {
    fn from(c: Rgb) -> Color {
        Color::Rgb(c)
    }
}

//##################################################################################################

#[cfg(test)]
mod tests {
    use super::parse_hex;
    #[test]
    fn parses_hex() {
        macro_rules! test_case {
            ($hex:literal, $r:expr, $g:expr, $b:expr) => {
                let parsed = parse_hex($hex).unwrap();
                assert_eq!($r, parsed.0);
                assert_eq!($g, parsed.1);
                assert_eq!($b, parsed.2);
            };
            ($hex:literal, None) => {
                let parsed = parse_hex($hex);
                assert!(parsed.is_none());
            };
        }

        test_case!("000000", 0, 0, 0);
        test_case!("ffffff", 255, 255, 255);
        test_case!("FFFFFF", 255, 255, 255);
        test_case!("abbaaf", 171, 186, 175);
        test_case!("12abff", 18, 171, 255);

        test_case!("", None);
        test_case!("12abf", None);
        test_case!("12abfff", None);
    }
}
