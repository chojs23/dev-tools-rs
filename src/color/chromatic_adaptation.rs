use crate::math::Matrix3;

use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum ChromaticAdaptationMethod {
    Bradford,
    VonKries,
    XYZScaling,
}

impl ChromaticAdaptationMethod {
    #[rustfmt::skip]
    pub fn adaptation_matrix(&self) -> Matrix3 {
        match &self {
            ChromaticAdaptationMethod::Bradford => {
                [
                    [ 0.8951,  0.2664, -0.1614],
                    [-0.7502,  1.7135,  0.367 ],
                    [ 0.0389, -0.0685,  1.0296],
                ]
            },
            ChromaticAdaptationMethod::VonKries => {
                [
                    [0.40024, 0.7076 , -0.08081 ],
                    [-0.2263, 1.16532,  0.0457  ],
                    [0.     , 0.     ,  0.912822]
                ]
            },
            ChromaticAdaptationMethod::XYZScaling => {
                [
                    [1., 0., 0.],
                    [0., 1., 0.],
                    [0., 0., 1.],
                ]
            },
        }.into()
    }
}

impl AsRef<str> for ChromaticAdaptationMethod {
    fn as_ref(&self) -> &str {
        match &self {
            ChromaticAdaptationMethod::Bradford => "Bradford",
            ChromaticAdaptationMethod::VonKries => "Von Kries",
            ChromaticAdaptationMethod::XYZScaling => "XYZ Scaling",
        }
    }
}

impl Default for ChromaticAdaptationMethod {
    fn default() -> Self {
        ChromaticAdaptationMethod::Bradford
    }
}
