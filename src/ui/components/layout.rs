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
