use crate::color::Color;
use eframe::egui::{
    lerp, {Color32, Rgba},
};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Gradient(pub Vec<Color32>);

#[allow(dead_code)]
impl Gradient {
    pub fn from_colors(colors: impl IntoIterator<Item = Color>) -> Self {
        Self(colors.into_iter().map(|c| c.color32()).collect())
    }

    pub fn one_color(srgba: Color32) -> Self {
        Self(vec![srgba, srgba])
    }

    pub fn as_hex(&self) -> Option<String> {
        self.0
            .first()
            .map(|color| format!("{:02x}{:02x}{:02x}", color.r(), color.g(), color.b()))
    }

    pub fn texture_gradient(left: Color, right: Color) -> Self {
        Self(vec![left.color32(), right.color32()])
    }
    pub fn ground_truth_linear_gradient(left: Color, right: Color) -> Self {
        let left = left.rgba();
        let right = right.rgba();

        const N: f32 = 255.0;
        let mut colors = vec![];
        for i in 0..=N as i32 {
            colors.push(Color32::from(lerp(left..=right, i as f32 / N)));
        }
        Self(colors)
    }

    /// Do premultiplied alpha-aware blending of the gradient on top of the fill color
    pub fn with_bg_fill(self, bg: Color32) -> Self {
        let bg = Rgba::from(bg);
        Self(
            self.0
                .into_iter()
                .map(|fg| {
                    let fg = Rgba::from(fg);
                    Color32::from(bg * (1.0 - fg.a()) + fg)
                })
                .collect(),
        )
    }

    pub fn to_pixel_row(&self) -> Vec<Color32> {
        self.0.clone()
    }
}
