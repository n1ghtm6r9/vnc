use crate::{ChromaLayout, ColourTransform, PlaneBuffer};

#[derive(Clone, Debug, PartialEq)]
pub struct DecodedFrame {
    pub width: u32,
    pub height: u32,
    pub layout: ChromaLayout,
    pub colour: ColourTransform,
    pub planes: PlaneBuffer,
}
