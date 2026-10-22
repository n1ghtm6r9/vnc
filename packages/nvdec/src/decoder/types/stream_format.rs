use super::DisplayArea;
use crate::{ChromaLayout, ColourTransform};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct StreamFormat {
    pub(crate) coded_width: u32,
    pub(crate) coded_height: u32,
    pub(crate) chroma: ChromaLayout,
    pub(crate) decode_surfaces: u8,
    pub(crate) display: DisplayArea,
    pub(crate) colour: ColourTransform,
}
