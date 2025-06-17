use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Debug, Copy, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum HarmonyLayout {
    // [ ][ ]
    // [ ][ ]
    Square,
    // [  ]
    // [  ]
    // [  ]
    // [  ]
    Stacked,
    // ________
    // ||||||||
    // ||||||||
    // --------
    Line,
    Gradient,
}

impl Default for HarmonyLayout {
    fn default() -> Self {
        HarmonyLayout::Square
    }
}

impl AsRef<str> for HarmonyLayout {
    fn as_ref(&self) -> &str {
        match self {
            Self::Square => "square",
            Self::Stacked => "stacked",
            Self::Line => "line",
            Self::Gradient => "gradient",
        }
    }
}
