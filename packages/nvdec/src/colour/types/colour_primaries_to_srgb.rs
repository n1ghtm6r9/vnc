use super::ColourPrimaries;
use crate::{BT709_TO_SRGB, DISPLAY_P3_TO_SRGB};

impl ColourPrimaries {
    pub(crate) fn to_srgb(self) -> [[f32; 3]; 3] {
        match self {
            ColourPrimaries::Bt709 => BT709_TO_SRGB,
            ColourPrimaries::DisplayP3 => DISPLAY_P3_TO_SRGB,
        }
    }
}
