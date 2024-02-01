use crate::color::{illuminant::Illuminant, lch_ab::LchAB, Xyz, CIE_E, CIE_K};

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub struct Lab {
    l: f32,
    a: f32,
    b: f32,
}

impl Lab {
    pub fn new(l: f32, a: f32, b: f32) -> Self {
        let l = if l.is_nan() { 0. } else { l };
        let a = if a.is_nan() { 0. } else { a };
        let b = if b.is_nan() { 0. } else { b };

        Self { l, a, b }
    }

    #[inline(always)]
    /// Returns Light
    pub fn l(&self) -> f32 {
        self.l
    }

    #[inline(always)]
    /// Returns A coordinate
    pub fn a(&self) -> f32 {
        self.a
    }

    #[inline(always)]
    /// Returns B coordinate
    pub fn b(&self) -> f32 {
        self.b
    }

    #[allow(clippy::many_single_char_names)]
    pub fn from_xyz(color: Xyz, reference_white: Illuminant) -> Self {
        let ref_xyz = reference_white.xyz();
        let x = color.x() / ref_xyz.x();
        let y = color.y() / ref_xyz.y();
        let z = color.z() / ref_xyz.z();

        fn f(num: f32) -> f32 {
            if num > CIE_E {
                num.cbrt()
            } else {
                (CIE_K * num + 16.) / 116.
            }
        }

        let fx = f(x);
        let fy = f(y);
        let fz = f(z);

        let l = 116. * fy - 16.;
        let a = 500. * (fx - fy);
        let b = 200. * (fy - fz);

        Self::new(l, a, b)
    }

    pub fn to_xyz(self, reference_white: Illuminant) -> Xyz {
        let ref_xyz = reference_white.xyz();

        let fy = (self.l + 16.) / 116.;
        let fz = fy - (self.b / 200.);
        let fx = fy + (self.a / 500.);

        let x = if fx.powi(3) > CIE_E {
            fx.powi(3)
        } else {
            (116. * fx - 16.) / CIE_K
        };
        let y = if self.l > CIE_E * CIE_K {
            ((self.l + 16.) / 116.).powi(3)
        } else {
            self.l / CIE_K
        };
        let z = if fz.powi(3) > CIE_E {
            fz.powi(3)
        } else {
            (116. * fz - 16.) / CIE_K
        };

        Xyz::new(x * ref_xyz.x(), y * ref_xyz.y(), z * ref_xyz.z())
    }
}

//####################################################################################################

impl From<LchAB> for Lab {
    fn from(color: LchAB) -> Self {
        let h = color.h().to_radians();

        let l = color.l();
        let a = color.c() * h.cos();
        let b = color.c() * h.sin();

        Self { l, a, b }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let inp = Lab::new(50., 50., 50.);
        let lch_ab = LchAB::from(inp);
        let got = Lab::from(lch_ab);

        assert_eq!(got, inp);
    }
}
