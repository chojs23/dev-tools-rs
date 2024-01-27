use eframe::epaint::Rect;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq)]
pub enum ScreenSize {
    Phone(f32, f32),
    Tablet(f32, f32),
    Laptop(f32, f32),
    Desktop(f32, f32),
}

impl ScreenSize {
    pub fn width(&self) -> f32 {
        match &self {
            ScreenSize::Phone(w, _) => *w,
            ScreenSize::Tablet(w, _) => *w,
            ScreenSize::Laptop(w, _) => *w,
            ScreenSize::Desktop(w, _) => *w,
        }
    }

    pub fn height(&self) -> f32 {
        match &self {
            ScreenSize::Phone(_, h) => *h,
            ScreenSize::Tablet(_, h) => *h,
            ScreenSize::Laptop(_, h) => *h,
            ScreenSize::Desktop(_, h) => *h,
        }
    }
}

impl From<Rect> for ScreenSize {
    fn from(screen: Rect) -> Self {
        match screen.width().round() as u32 {
            0..=480 => ScreenSize::Phone(screen.width(), screen.height()),
            481..=768 => ScreenSize::Tablet(screen.width(), screen.height()),
            769..=992 => ScreenSize::Laptop(screen.width(), screen.height()),
            _ => ScreenSize::Desktop(screen.width(), screen.height()),
        }
    }
}
