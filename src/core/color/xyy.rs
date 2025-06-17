use super::Xyz;

use serde::{Deserialize, Serialize};

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub struct xyY {
    x: f32,
    y: f32,
    yy: f32,
}

impl xyY {
    pub fn new(x: f32, y: f32, yy: f32) -> Self {
        let x = if x.is_nan() { 0. } else { x };
        let y = if y.is_nan() { 0. } else { y };
        let yy = if yy.is_nan() { 0. } else { yy };

        Self { x, y, yy }
    }

    #[inline(always)]
    /// Returns x coordinate
    pub fn x(&self) -> f32 {
        self.x
    }

    #[inline(always)]
    /// Returns y coordinate
    pub fn y(&self) -> f32 {
        self.y
    }

    #[inline(always)]
    /// Returns Y coordinate
    pub fn yy(&self) -> f32 {
        self.yy
    }
}

//####################################################################################################

impl From<Xyz> for xyY {
    fn from(color: Xyz) -> Self {
        let xx = color.x();
        let yy = color.y();
        let zz = color.z();
        let x = xx / (xx + yy + zz);
        let y = yy / (xx + yy + zz);
        xyY::new(x, y, yy)
    }
}
