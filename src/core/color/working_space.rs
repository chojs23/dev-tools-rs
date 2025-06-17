#![allow(dead_code)]
use crate::utils::math::{Matrix1x3, Matrix3};

use super::{illuminant::Illuminant, xyY, Rgb};

use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[allow(clippy::upper_case_acronyms)]
pub enum RgbWorkingSpace {
    Adobe,
    Apple,
    CIE,
    ECI,
    NTSC,
    PAL,
    ProPhoto,
    SRGB,
    WideGamut,
}

impl Default for RgbWorkingSpace {
    fn default() -> Self {
        Self::SRGB
    }
}

impl RgbWorkingSpace {
    pub fn reference_illuminant(&self) -> Illuminant {
        use RgbWorkingSpace::*;
        match &self {
            Adobe => Illuminant::D65,
            Apple => Illuminant::D65,
            CIE => Illuminant::E,
            ECI => Illuminant::D50,
            NTSC => Illuminant::C,
            PAL => Illuminant::D65,
            ProPhoto => Illuminant::D50,
            SRGB => Illuminant::D65,
            WideGamut => Illuminant::D50,
        }
    }

    pub fn rgb_matrix(&self) -> Matrix3 {
        let ref_red = self.reference_red_xyy();
        let ref_green = self.reference_green_xyy();
        let ref_blue = self.reference_blue_xyy();
        let ref_white = self.reference_illuminant().xyz();

        let xr = ref_red.x();
        let yr = ref_red.y();
        let xg = ref_green.x();
        let yg = ref_green.y();
        let xb = ref_blue.x();
        let yb = ref_blue.y();

        let xxr = xr / yr;
        let yyr = 1.;
        let zzr = (1. - xr - yr) / yr;

        let xxg = xg / yg;
        let yyg = yyr;
        let zzg = (1. - xg - yg) / yg;

        let xxb = xb / yb;
        let yyb = yyg;
        let zzb = (1. - xb - yb) / yb;

        let s = Matrix3::from([[xxr, xxg, xxb], [yyr, yyg, yyb], [zzr, zzg, zzb]])
            .inverse()
            .expect("inverse matrix")
            * Matrix1x3::from([ref_white.x(), ref_white.y(), ref_white.z()]);

        Matrix3::from([
            [s[0] * xxr, s[1] * xxg, s[2] * xxb],
            [s[0] * yyr, s[1] * yyg, s[2] * yyb],
            [s[0] * zzr, s[1] * zzg, s[2] * zzb],
        ])
    }

    pub fn inverse_rgb_matrix(&self) -> Matrix3 {
        self.rgb_matrix().inverse().expect("inverse matrix")
    }

    pub fn gamma(&self) -> f32 {
        use RgbWorkingSpace::*;
        match &self {
            Adobe => 2.2,
            Apple => 1.8,
            CIE => 2.2,
            ECI => 3.,
            NTSC => 2.2,
            PAL => 2.2,
            ProPhoto => 1.8,
            SRGB => 2.2,
            WideGamut => 2.2,
        }
    }

    pub fn compand_channels(&self, color: Rgb) -> Rgb {
        use RgbWorkingSpace::*;
        match &self {
            Adobe | Apple | CIE | NTSC | PAL | ProPhoto | WideGamut => {
                color.gamma_compand(self.gamma())
            }
            ECI => color.l_compand(),
            SRGB => color.srgb_compand(),
        }
    }

    pub fn inverse_compand_channels(&self, color: Rgb) -> Rgb {
        use RgbWorkingSpace::*;
        match &self {
            Adobe | Apple | CIE | NTSC | PAL | ProPhoto | WideGamut => {
                color.inverse_gamma_compand(self.gamma())
            }
            ECI => color.inverse_l_compand(),
            SRGB => color.inverse_srgb_compand(),
        }
    }

    #[rustfmt::skip]
    pub fn reference_red_xyy(&self) -> xyY {
        use RgbWorkingSpace::*;
        match &self {
            Adobe     => xyY::new(0.6400, 0.3300, 0.297361),
            Apple     => xyY::new(0.6250, 0.3400, 0.244634),
            CIE       => xyY::new(0.7350, 0.2650, 0.176204),
            ECI       => xyY::new(0.6700, 0.3300, 0.320250),
            NTSC      => xyY::new(0.6700, 0.3300, 0.298839),
            PAL       => xyY::new(0.6400, 0.3300, 0.222021),
            ProPhoto  => xyY::new(0.7347, 0.2653, 0.288040),
            SRGB      => xyY::new(0.6400, 0.3300, 0.212656),
            WideGamut => xyY::new(0.7350, 0.2650, 0.258187),
       }
    }

    #[rustfmt::skip]
    pub fn reference_green_xyy(&self) -> xyY {
        use RgbWorkingSpace::*;
        match &self {
            Adobe     => xyY::new(0.2100, 0.7100, 0.627355),
            Apple     => xyY::new(0.2800, 0.5950, 0.672034),
            CIE       => xyY::new(0.2740, 0.7170, 0.812985),
            ECI       => xyY::new(0.2100, 0.7100, 0.602071),
            NTSC      => xyY::new(0.2100, 0.7100, 0.586811),
            PAL       => xyY::new(0.2900, 0.6000, 0.706645),
            ProPhoto  => xyY::new(0.1596, 0.8404, 0.711874),
            SRGB      => xyY::new(0.3000, 0.6000, 0.715158),
            WideGamut => xyY::new(0.1150, 0.8260, 0.724938),
        }
    }

    #[rustfmt::skip]
    pub fn reference_blue_xyy(&self) -> xyY {
        use RgbWorkingSpace::*;
        match &self {
            Adobe     => xyY::new(0.1500, 0.0600, 0.075285),
            Apple     => xyY::new(0.1550, 0.0700, 0.083332),
            CIE       => xyY::new(0.1670, 0.0090, 0.010811),
            ECI       => xyY::new(0.1400, 0.0800, 0.077679),
            NTSC      => xyY::new(0.1400, 0.0800, 0.114350),
            PAL       => xyY::new(0.1500, 0.0600, 0.071334),
            ProPhoto  => xyY::new(0.0366, 0.0001, 0.000086),
            SRGB      => xyY::new(0.1500, 0.0600, 0.072186),
            WideGamut => xyY::new(0.1570, 0.0180, 0.016875),
        }
    }
}

impl AsRef<str> for RgbWorkingSpace {
    fn as_ref(&self) -> &str {
        match &self {
            RgbWorkingSpace::Adobe => "Adobe RGB",
            RgbWorkingSpace::Apple => "Apple RGB",
            RgbWorkingSpace::CIE => "CIE RGB",
            RgbWorkingSpace::ECI => "ECI RGB",
            RgbWorkingSpace::NTSC => "NTSC RGB",
            RgbWorkingSpace::PAL => "PAL RGB",
            RgbWorkingSpace::ProPhoto => "Pro Photo RGB",
            RgbWorkingSpace::SRGB => "SRGB",
            RgbWorkingSpace::WideGamut => "Adobe Wide Gamut RGB",
        }
    }
}

//####################################################################################################
