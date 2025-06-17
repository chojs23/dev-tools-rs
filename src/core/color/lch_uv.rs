use super::{Luv, Xyz};

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub struct LchUV {
    l: f32,
    c: f32,
    h: f32,
}

impl LchUV {
    pub fn new(l: f32, c: f32, h: f32) -> Self {
        let l = if l.is_nan() { 0. } else { l };
        let c = if c.is_nan() { 0. } else { c };
        let h = if h.is_nan() { 0. } else { h };

        Self { l, c, h }
    }

    #[inline(always)]
    /// Returns Light
    pub fn l(&self) -> f32 {
        self.l
    }

    #[inline(always)]
    /// Returns Chroma
    pub fn c(&self) -> f32 {
        self.c
    }

    #[inline(always)]
    /// Returns Hue in the range of 0.0 ..= 360.0
    pub fn h(&self) -> f32 {
        self.h
    }
}

//####################################################################################################

#[allow(clippy::many_single_char_names)]
impl From<Luv> for LchUV {
    fn from(color: Luv) -> Self {
        let u = color.u();
        let v = color.v();
        let c = (u.powi(2) + v.powi(2)).sqrt();
        let vu_atan = f32::atan2(v, u).to_degrees();
        let h = if vu_atan >= 0. {
            vu_atan
        } else {
            vu_atan + 360.
        };

        LchUV::new(color.l(), c, h)
    }
}

impl From<Xyz> for LchUV {
    fn from(color: Xyz) -> Self {
        Luv::from(color).into()
    }
}
