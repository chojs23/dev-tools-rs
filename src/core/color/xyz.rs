use super::{
    chromatic_adaptation::ChromaticAdaptationMethod, illuminant::Illuminant, rgb::Rgb,
    working_space::RgbWorkingSpace, xyy::xyY, CIEColor, LchUV, Luv, CIE_E, CIE_K,
};
use crate::utils::math::{Matrix1x3, Matrix3};

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub struct Xyz {
    x: f32,
    y: f32,
    z: f32,
}

impl Xyz {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        let x = if x.is_nan() { 0. } else { x };
        let y = if y.is_nan() { 0. } else { y };
        let z = if z.is_nan() { 0. } else { z };

        Self { x, y, z }
    }

    #[inline(always)]
    pub fn x(&self) -> f32 {
        self.x
    }

    #[inline(always)]
    pub fn y(&self) -> f32 {
        self.y
    }

    #[inline(always)]
    pub fn z(&self) -> f32 {
        self.z
    }

    pub fn x_scaled(&self) -> f32 {
        self.x * 100.
    }

    pub fn y_scaled(&self) -> f32 {
        self.y * 100.
    }

    pub fn z_scaled(&self) -> f32 {
        self.z * 100.
    }

    #[inline(always)]
    pub fn u(&self) -> f32 {
        4. * self.x / (self.x + 15. * self.y + 3. * self.z)
    }

    #[inline(always)]
    pub fn v(&self) -> f32 {
        9. * self.y / (self.x + 15. * self.y + 3. * self.z)
    }

    pub fn chromatic_adaptation_transform(
        &self,
        method: ChromaticAdaptationMethod,
        src_white: Illuminant,
        dst_white: Illuminant,
    ) -> Xyz {
        let src_ref_xyz = Matrix1x3::from(src_white.xyz());
        let dst_ref_xyz = Matrix1x3::from(dst_white.xyz());

        let ma = method.adaptation_matrix();

        let src_lms = ma * src_ref_xyz;
        let dst_lms = ma * dst_ref_xyz;

        let lms = Matrix3::from([
            [src_lms[0] / dst_lms[0], 0., 0.],
            [0., src_lms[1] / dst_lms[1], 0.],
            [0., 0., src_lms[2] / dst_lms[2]],
        ]);

        let m = ma.inverse().expect("inverse adaptation matrix") * lms * ma;

        Xyz::from(m * Matrix1x3::from(*self))
    }
}

impl CIEColor for Xyz {
    fn to_rgb(self, working_space: RgbWorkingSpace) -> Rgb {
        let rgb = Rgb::from(working_space.inverse_rgb_matrix() * Matrix1x3::from(self));
        working_space.compand_channels(rgb)
    }

    fn from_rgb(rgb: Rgb, working_space: RgbWorkingSpace) -> Self {
        let rgb: Matrix1x3 = working_space.inverse_compand_channels(rgb).into();
        (working_space.rgb_matrix() * rgb).into()
    }
}

//####################################################################################################

#[allow(clippy::many_single_char_names)]
impl From<Luv> for Xyz {
    fn from(color: Luv) -> Self {
        let l = color.l();
        let u = color.u();
        let v = color.v();

        let y = if l > CIE_K * CIE_E {
            ((l + 16.) / 116.).powi(3)
        } else {
            l / CIE_K
        };

        let a = ((52. * l / (u + 13. * l * Illuminant::D65.reference_u())) - 1.) / 3.;
        let b = -5. * y;
        let c = -(1.0f32 / 3.);
        let d = y * ((39. * l / (v + 13. * l * Illuminant::D65.reference_v())) - 5.);

        let x = (d - b) / (a - c);
        let z = x * a + b;

        Xyz::new(x, y, z)
    }
}

impl From<LchUV> for Xyz {
    fn from(lch: LchUV) -> Self {
        Luv::from(lch).into()
    }
}

impl From<xyY> for Xyz {
    fn from(color: xyY) -> Self {
        let x = color.x();
        let y = color.y();
        let yy = color.yy();

        if y == 0. {
            return Self {
                x: 0.,
                y: 0.,
                z: 0.,
            };
        }

        let xx = x * yy / y;
        let zz = (1. - x - y) * yy / y;

        Self::new(xx, yy, zz)
    }
}

impl From<Matrix1x3> for Xyz {
    fn from(mx: Matrix1x3) -> Self {
        Self {
            x: mx[0],
            y: mx[1],
            z: mx[2],
        }
    }
}

impl From<Xyz> for Matrix1x3 {
    fn from(color: Xyz) -> Self {
        [color.x, color.y, color.z].into()
    }
}

//####################################################################################################

#[cfg(test)]
mod tests {
    use super::{CIEColor, Rgb, RgbWorkingSpace, Xyz};

    #[test]
    fn rgb_to_xyz() {
        macro_rules! test_case {
            ($ws:expr; Rgb: $r:expr, $g:expr, $b:expr; Xyz: $x:expr, $y:expr, $z:expr) => {
                let expected = Xyz::new($x, $y, $z);
                let got = Xyz::from_rgb(Rgb::new($r, $g, $b), $ws);
                assert_eq!(got, expected);
            };
        }

        test_case!(
            RgbWorkingSpace::SRGB;
            Rgb: 0., 0., 0.;
            Xyz: 0., 0., 0.
        );
        test_case!(
            RgbWorkingSpace::SRGB;
            Rgb: 1., 1., 1.;
            Xyz: 0.9504699, 1., 1.0888301
        );
    }
}
