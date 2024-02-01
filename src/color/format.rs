use crate::color::{
    xyY, CIEColor, Cmyk, Color, Hsl, Hsv, Illuminant, Lab, LchAB, LchUV, Luv, Palette,
    RgbWorkingSpace, Xyz,
};

use anyhow::{Error, Result};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{char, digit1, space0},
    combinator::{map, map_res, opt},
    error::{ErrorKind, FromExternalError, ParseError},
    multi::many0,
    sequence::{delimited, preceded, tuple},
    Err, IResult, Parser,
};
use serde::{Deserialize, Serialize};
use std::{fmt::Write, num::ParseIntError};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CustomPaletteFormat {
    pub prefix: String,
    pub entry_format: String,
    pub suffix: String,
}

impl CustomPaletteFormat {
    pub fn format_palette(
        &self,
        palette: &Palette,
        ws: RgbWorkingSpace,
        illuminant: Illuminant,
    ) -> Result<String> {
        let mut s = self.prefix.clone();
        let entry_format = CustomColorFormat::parse(&self.entry_format)?;
        for entry in palette.iter() {
            s.push_str(&entry_format.format_color(entry, ws, illuminant)?);
        }
        s.push_str(&self.suffix);
        Ok(s)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CustomColorFormat<'a>(Vec<FormatToken<'a>>);

impl<'a> CustomColorFormat<'a> {
    pub fn parse(text: &'a str) -> Result<CustomColorFormat<'a>> {
        match parse_color_format(text).map(|(_, fmt)| fmt) {
            Ok(fmt) => Ok(fmt),
            Err(e) => Err(Error::msg(format!("failed to parse color format - {}", e))),
        }
    }

    pub fn format_color(
        &self,
        color: &Color,
        ws: RgbWorkingSpace,
        illuminant: Illuminant,
    ) -> Result<String> {
        use ColorSymbol::*;

        let rgb = color.rgb();
        let cmyk = Cmyk::from(rgb);
        let hsl = Hsl::from(rgb);
        let hsv = Hsv::from(rgb);
        let xyz = Xyz::from_rgb(rgb, ws);
        let xyy = xyY::from(xyz);
        let lab = Lab::from_xyz(xyz, illuminant);
        let luv = Luv::from(xyz);
        let lch_ab = LchAB::from(lab);
        let lch_uv = LchUV::from(luv);

        let mut s = String::new();

        for token in &self.0 {
            match &token {
                FormatToken::Text(text) => s.push_str(text),
                FormatToken::Color(ColorField {
                    symbol,
                    digit_format,
                }) => match symbol {
                    Red | Green | Blue | Cyan | Magenta | Yellow | Key | Cyan100 | Magenta100
                    | Yellow100 | Key100 | HSLHue | HSLSaturation | HSLLight | HSVHue
                    | HSVSaturation | HSVValue | LabL | LabA | LabB | LCHabL | LCHabC | LCHabH
                    | LuvL | LuvU | LuvV | LCHuvL | LCHuvC | LCHuvH | xyYx | xyYy | xyYY | XYZx
                    | XYZy | XYZz | HSLHue360 | HSLSaturation100 | HSLLight100 | HSVHue360
                    | HSVSaturation100 | HSVValue100 => {
                        let num = match symbol {
                            Red => rgb.r(),
                            Green => rgb.g(),
                            Blue => rgb.b(),

                            Cyan => cmyk.c(),
                            Magenta => cmyk.m(),
                            Yellow => cmyk.y(),
                            Key => cmyk.k(),

                            Cyan100 => cmyk.c_scaled(),
                            Magenta100 => cmyk.m_scaled(),
                            Yellow100 => cmyk.y_scaled(),
                            Key100 => cmyk.k_scaled(),

                            HSLHue => hsl.h(),
                            HSLSaturation => hsl.s(),
                            HSLLight => hsl.l(),

                            HSLHue360 => hsl.h_scaled(),
                            HSLSaturation100 => hsl.s_scaled(),
                            HSLLight100 => hsl.l_scaled(),

                            HSVHue => hsv.h(),
                            HSVSaturation => hsv.s(),
                            HSVValue => hsv.v(),

                            HSVHue360 => hsv.h_scaled(),
                            HSVSaturation100 => hsv.s_scaled(),
                            HSVValue100 => hsv.v_scaled(),

                            LabL => lab.l(),
                            LabA => lab.a(),
                            LabB => lab.b(),

                            LCHabL => lch_ab.l(),
                            LCHabC => lch_ab.c(),
                            LCHabH => lch_ab.h(),

                            LuvL => luv.l(),
                            LuvU => luv.u(),
                            LuvV => luv.v(),

                            LCHuvL => lch_uv.l(),
                            LCHuvC => lch_uv.c(),
                            LCHuvH => lch_uv.h(),

                            xyYx => xyy.x(),
                            xyYy => xyy.y(),
                            xyYY => xyy.yy(),

                            XYZx => xyz.x(),
                            XYZy => xyz.y(),
                            XYZz => xyz.z(),
                            _ => unreachable!(),
                        };

                        match digit_format {
                            Some(DigitFormat::Decimal) => {
                                write!(&mut s, "{}", num.abs() as u32)?;
                            }
                            Some(DigitFormat::Hex) => {
                                write!(&mut s, "{:x}", num.abs() as u32)?;
                            }
                            Some(DigitFormat::UppercaseHex) => {
                                write!(&mut s, "{:X}", num.abs() as u32)?;
                            }
                            Some(DigitFormat::Octal) => {
                                write!(&mut s, "{:o}", num.abs() as u32)?;
                            }
                            Some(DigitFormat::Float { precision }) => {
                                write!(&mut s, "{:.*}", *precision as usize, num)?;
                            }
                            None => {
                                write!(&mut s, "{:.1}", num)?;
                            }
                        }
                    }
                    Red255 | Green255 | Blue255 => {
                        let num = match symbol {
                            Red255 => rgb.r_scaled(),
                            Green255 => rgb.g_scaled(),
                            Blue255 => rgb.b_scaled(),
                            _ => unreachable!(),
                        } as u32;

                        match digit_format.unwrap_or_default() {
                            DigitFormat::Decimal => write!(&mut s, "{}", num)?,
                            DigitFormat::Hex => write!(&mut s, "{:x}", num)?,
                            DigitFormat::UppercaseHex => write!(&mut s, "{:X}", num)?,
                            DigitFormat::Octal => write!(&mut s, "{:o}", num)?,
                            DigitFormat::Float { precision: _ } => write!(&mut s, "{}", num)?,
                        }
                    }
                },
            }
        }

        Ok(s)
    }
}

impl<'a> From<Vec<FormatToken<'a>>> for CustomColorFormat<'a> {
    fn from(vec: Vec<FormatToken<'a>>) -> Self {
        Self(vec)
    }
}

#[derive(Debug, PartialEq)]
enum ColorParseError<I> {
    InputEmpty,
    InvalidPrecision,
    Nom(I, ErrorKind),
}

impl<I> ParseError<I> for ColorParseError<I> {
    fn from_error_kind(input: I, kind: ErrorKind) -> Self {
        ColorParseError::Nom(input, kind)
    }

    fn append(_: I, _: ErrorKind, other: Self) -> Self {
        other
    }
}

impl FromExternalError<&str, ParseIntError> for ColorParseError<&str> {
    fn from_external_error(_: &str, _: ErrorKind, _: ParseIntError) -> Self {
        Self::InvalidPrecision
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum FormatToken<'a> {
    Color(ColorField),
    Text(&'a str),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorField {
    symbol: ColorSymbol,
    digit_format: Option<DigitFormat>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum DigitFormat {
    Hex,
    UppercaseHex,
    Octal,
    Decimal,
    Float { precision: u8 },
}

impl Default for DigitFormat {
    fn default() -> Self {
        Self::Decimal
    }
}

#[rustfmt::skip]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq)]
enum ColorSymbol {
    Red,
    Green,
    Blue,

    Red255,
    Green255,
    Blue255,

    Cyan,
    Magenta,
    Yellow,
    Key,

    Cyan100,
    Magenta100,
    Yellow100,
    Key100,

    HSLHue,
    HSLSaturation,
    HSLLight,

    HSLHue360,
    HSLSaturation100,
    HSLLight100,

    HSVHue,
    HSVSaturation,
    HSVValue,

    HSVHue360,
    HSVSaturation100,
    HSVValue100,

    LabL,
    LabA,
    LabB,
    
    LCHabL,
    LCHabC,
    LCHabH,
    
    LuvL,
    LuvU,
    LuvV,
    
    LCHuvL,
    LCHuvC,
    LCHuvH,
    
    xyYx,
    xyYy,
    xyYY,
    
    XYZx,
    XYZy,
    XYZz,
}

fn parse_rgb_symbol(i: &str) -> IResult<&str, ColorSymbol, ColorParseError<&str>> {
    alt((
        tag("r255").map(|_| ColorSymbol::Red255),
        tag("g255").map(|_| ColorSymbol::Green255),
        tag("b255").map(|_| ColorSymbol::Blue255),
        char('r').map(|_| ColorSymbol::Red),
        char('g').map(|_| ColorSymbol::Green),
        char('b').map(|_| ColorSymbol::Blue),
    ))(i)
}

fn parse_cmyk_symbol(i: &str) -> IResult<&str, ColorSymbol, ColorParseError<&str>> {
    alt((
        tag("c100").map(|_| ColorSymbol::Cyan100),
        tag("m100").map(|_| ColorSymbol::Magenta100),
        tag("y100").map(|_| ColorSymbol::Yellow100),
        tag("k100").map(|_| ColorSymbol::Key100),
        char('c').map(|_| ColorSymbol::Cyan),
        char('m').map(|_| ColorSymbol::Magenta),
        char('y').map(|_| ColorSymbol::Yellow),
        char('k').map(|_| ColorSymbol::Key),
    ))(i)
}

fn parse_hsl_symbol(i: &str) -> IResult<&str, ColorSymbol, ColorParseError<&str>> {
    alt((
        tag("hsl_h360").map(|_| ColorSymbol::HSLHue360),
        tag("hsl_s100").map(|_| ColorSymbol::HSLSaturation100),
        tag("hsl_l100").map(|_| ColorSymbol::HSLLight100),
        tag("hsl_h").map(|_| ColorSymbol::HSLHue),
        tag("hsl_s").map(|_| ColorSymbol::HSLSaturation),
        tag("hsl_l").map(|_| ColorSymbol::HSLLight),
    ))(i)
}

fn parse_hsv_symbol(i: &str) -> IResult<&str, ColorSymbol, ColorParseError<&str>> {
    alt((
        tag("hsv_h360").map(|_| ColorSymbol::HSVHue360),
        tag("hsv_s100").map(|_| ColorSymbol::HSVSaturation100),
        tag("hsv_v100").map(|_| ColorSymbol::HSVValue100),
        tag("hsv_h").map(|_| ColorSymbol::HSVHue),
        tag("hsv_s").map(|_| ColorSymbol::HSVSaturation),
        tag("hsv_v").map(|_| ColorSymbol::HSVValue),
    ))(i)
}

fn parse_lab_symbol(i: &str) -> IResult<&str, ColorSymbol, ColorParseError<&str>> {
    alt((
        tag("lab_l").map(|_| ColorSymbol::LabL),
        tag("lab_a").map(|_| ColorSymbol::LabA),
        tag("lab_b").map(|_| ColorSymbol::LabB),
    ))(i)
}

fn parse_lch_ab_symbol(i: &str) -> IResult<&str, ColorSymbol, ColorParseError<&str>> {
    alt((
        tag("lch_ab_l").map(|_| ColorSymbol::LCHabL),
        tag("lch_ab_c").map(|_| ColorSymbol::LCHabC),
        tag("lch_ab_h").map(|_| ColorSymbol::LCHabH),
    ))(i)
}

fn parse_luv_symbol(i: &str) -> IResult<&str, ColorSymbol, ColorParseError<&str>> {
    alt((
        tag("luv_l").map(|_| ColorSymbol::LuvL),
        tag("luv_u").map(|_| ColorSymbol::LuvU),
        tag("luv_v").map(|_| ColorSymbol::LuvV),
    ))(i)
}

fn parse_lch_uv_symbol(i: &str) -> IResult<&str, ColorSymbol, ColorParseError<&str>> {
    alt((
        tag("lch_uv_l").map(|_| ColorSymbol::LCHuvL),
        tag("lch_uv_c").map(|_| ColorSymbol::LCHuvC),
        tag("lch_uv_h").map(|_| ColorSymbol::LCHuvH),
    ))(i)
}

fn parse_xyy_symbol(i: &str) -> IResult<&str, ColorSymbol, ColorParseError<&str>> {
    alt((
        tag("xyy_x").map(|_| ColorSymbol::xyYx),
        tag("xyy_y").map(|_| ColorSymbol::xyYy),
        tag("xyy_Y").map(|_| ColorSymbol::xyYY),
    ))(i)
}

fn parse_xyz_symbol(i: &str) -> IResult<&str, ColorSymbol, ColorParseError<&str>> {
    alt((
        tag("xyz_x").map(|_| ColorSymbol::XYZx),
        tag("xyz_y").map(|_| ColorSymbol::XYZy),
        tag("xyz_z").map(|_| ColorSymbol::XYZz),
    ))(i)
}

fn parse_color_symbol(i: &str) -> IResult<&str, ColorSymbol, ColorParseError<&str>> {
    alt((
        parse_rgb_symbol,
        parse_cmyk_symbol,
        parse_hsl_symbol,
        parse_hsv_symbol,
        parse_lab_symbol,
        parse_lch_ab_symbol,
        parse_luv_symbol,
        parse_lch_uv_symbol,
        parse_xyy_symbol,
        parse_xyz_symbol,
    ))(i)
}

fn parse_decimal_format(i: &str) -> IResult<&str, DigitFormat, ColorParseError<&str>> {
    map(char('d'), |_| DigitFormat::Decimal)(i)
}

fn parse_hex_format(i: &str) -> IResult<&str, DigitFormat, ColorParseError<&str>> {
    map(char('x'), |_| DigitFormat::Hex)(i)
}

fn parse_hex_uppercase_format(i: &str) -> IResult<&str, DigitFormat, ColorParseError<&str>> {
    map(char('X'), |_| DigitFormat::UppercaseHex)(i)
}

fn parse_octal_format(i: &str) -> IResult<&str, DigitFormat, ColorParseError<&str>> {
    map(char('o'), |_| DigitFormat::Octal)(i)
}

fn parse_float_format(i: &str) -> IResult<&str, DigitFormat, ColorParseError<&str>> {
    map(
        preceded(char('.'), map_res(digit1, |s: &str| s.parse::<u8>())),
        |precision| DigitFormat::Float { precision },
    )(i)
}

fn parse_digit_format(i: &str) -> IResult<&str, DigitFormat, ColorParseError<&str>> {
    preceded(
        char(':'),
        alt((
            parse_hex_format,
            parse_hex_uppercase_format,
            parse_octal_format,
            parse_decimal_format,
            parse_float_format,
        )),
    )(i)
}

fn parse_color_field(i: &str) -> IResult<&str, ColorField, ColorParseError<&str>> {
    delimited(
        char('{'),
        preceded(
            space0,
            map(
                tuple((parse_color_symbol, opt(parse_digit_format))),
                |(symbol, digit_format)| ColorField {
                    symbol,
                    digit_format,
                },
            ),
        ),
        preceded(space0, char('}')),
    )(i)
}

#[inline]
fn is_not_variable_start(chr: char) -> bool {
    chr != '{'
}

fn parse_text(i: &str) -> IResult<&str, &str, ColorParseError<&str>> {
    if i.is_empty() {
        return Err(Err::Error(ColorParseError::InputEmpty));
    }

    take_while(is_not_variable_start)(i)
}

fn parse_brace(i: &str) -> IResult<&str, FormatToken, ColorParseError<&str>> {
    map(tag("{"), FormatToken::Text)(i)
}

fn parse_format_token(i: &str) -> IResult<&str, FormatToken, ColorParseError<&str>> {
    alt((
        map(parse_color_field, FormatToken::Color),
        parse_brace,
        map(parse_text, FormatToken::Text),
    ))(i)
}

fn parse_color_format(i: &str) -> IResult<&str, CustomColorFormat, ColorParseError<&str>> {
    map(many0(parse_format_token), CustomColorFormat)(i)
}

#[cfg(test)]
mod tests {
    use crate::color::{
        format::{ColorField, ColorSymbol, CustomColorFormat, DigitFormat, FormatToken},
        Color, Illuminant, Rgb, RgbWorkingSpace,
    };
    macro_rules! field {
        ($sym:tt) => {
            FormatToken::Color(ColorField {
                symbol: ColorSymbol::$sym,
                digit_format: None,
            })
        };
        ($sym:tt, $fmt:tt) => {
            FormatToken::Color(ColorField {
                symbol: ColorSymbol::$sym,
                digit_format: Some(DigitFormat::$fmt),
            })
        };
        ($sym:tt, $fmt:expr) => {
            FormatToken::Color(ColorField {
                symbol: ColorSymbol::$sym,
                digit_format: Some($fmt),
            })
        };
    }
    macro_rules! test_case {
        ($input:literal, $want:expr) => {
            let parsed = CustomColorFormat::parse($input).unwrap();
            assert_eq!(parsed, $want);
        };
    }

    #[test]
    fn formats_custom_color_string() {
        macro_rules! test_case {
            ($fmt:literal => $want:literal, $color:expr) => {
                let color_format = CustomColorFormat::parse($fmt).unwrap();
                let color = $color;
                let formatted = color_format
                    .format_color(&color, RgbWorkingSpace::SRGB, Illuminant::D65)
                    .unwrap();
                assert_eq!(formatted, $want);
            };
        }
        test_case!(
            "{r} {g} {b}" => "0.5 0.5 0.5",
            Color::Rgb(Rgb::new(0.5, 0.5, 0.5))
        );
        test_case!(
            "{r255} {g255} {b255}" => "127 127 127",
            Color::Rgb(Rgb::new(0.5, 0.5, 0.5))
        );
        test_case!(
            "r:0o{r255:o} g:0x{g255:X} b:0x{b255:x}" => "r:0o177 g:0x7F b:0x7f",
            Color::Rgb(Rgb::new(0.5, 0.5, 0.5))
        );
        test_case!(
            "{lab_l:.4} {lab_a:.0} {lab_b:.2}" => "55.6818 -17 -27.27",
            Color::Rgb(Rgb::new_scaled(35, 144, 180))
        );
        test_case!(
            "{hsv_h360:d} {hsv_s100:X} {hsv_v100:x}" => "326 4B 2f",
            Color::Rgb(Rgb::new_scaled(120, 30, 80))
        );
    }

    #[test]
    fn parses_basic_color_format() {
        test_case!(
            "{r} {g} {b}",
            vec![
                field!(Red),
                FormatToken::Text(" "),
                field!(Green),
                FormatToken::Text(" "),
                field!(Blue),
            ]
            .into()
        );
        test_case!(
            "{ c }{m} { y  }   {   k }%",
            vec![
                field!(Cyan),
                field!(Magenta),
                FormatToken::Text(" "),
                field!(Yellow),
                FormatToken::Text("   "),
                field!(Key),
                FormatToken::Text("%"),
            ]
            .into()
        );
        test_case!(
            "{hsv_h} {{ {hsv_s} }} {hsv_v}",
            vec![
                field!(HSVHue),
                FormatToken::Text(" "),
                FormatToken::Text("{"),
                FormatToken::Text("{"),
                FormatToken::Text(" "),
                field!(HSVSaturation),
                FormatToken::Text(" }} "),
                field!(HSVValue),
            ]
            .into()
        );
    }

    #[test]
    fn parses_digit_format() {
        test_case!(
            "L:{ lch_ab_l:x } C:{lch_ab_c:X} H:{lch_ab_h:o} r:{r:.4}",
            vec![
                FormatToken::Text("L:"),
                field!(LCHabL, Hex),
                FormatToken::Text(" C:"),
                field!(LCHabC, UppercaseHex),
                FormatToken::Text(" H:"),
                field!(LCHabH, Octal),
                FormatToken::Text(" r:"),
                field!(Red, DigitFormat::Float { precision: 4 }),
            ]
            .into()
        );
    }
}
