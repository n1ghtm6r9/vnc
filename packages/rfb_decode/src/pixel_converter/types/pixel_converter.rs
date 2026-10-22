use super::{ColourChannel, CompactLayout};

#[derive(Debug, Clone)]
pub struct PixelConverter {
    pub(super) bytes_per_pixel: usize,
    pub(super) big_endian: bool,
    pub(super) true_colour: bool,
    pub(super) rgba32_layout: bool,
    pub(super) tight_rgb24: bool,
    pub(super) compact: CompactLayout,
    pub(super) red: ColourChannel,
    pub(super) green: ColourChannel,
    pub(super) blue: ColourChannel,
    pub(super) colour_map: Vec<u32>,
}
