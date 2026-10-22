use super::SAME_PRIMARIES;
use crate::YuvColour;

pub const PLANES_AS_RGB: YuvColour = YuvColour {
    rows: [
        [0.0, 0.0, 1.0, 0.0],
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
    ],
    primaries: SAME_PRIMARIES,
};
