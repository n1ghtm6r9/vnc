use std::fmt;

use super::{MvsTileCache, MvsTileDescriptor};

pub(crate) struct MvsState {
    pub(super) luma_quantization: [u8; 64],
    pub(super) chroma_quantization: [u8; 64],
    pub(super) quantization_generation: u32,
    pub(super) coefficients: [[i16; 64]; 3],
    pub(super) previous: [[i16; 64]; 3],
    pub(super) solid: u32,
    pub(super) set_colour: u32,
    pub(super) clear_colour: u32,
    pub(super) frame: u32,
    pub(super) screen_width: u16,
    pub(super) screen_height: u16,
    pub(super) screen_tiles_across: usize,
    pub(super) descriptors: Vec<MvsTileDescriptor>,
    pub(super) cache: MvsTileCache,
}

impl fmt::Debug for MvsState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MvsState")
            .field("screen_width", &self.screen_width)
            .field("screen_height", &self.screen_height)
            .field("frame", &self.frame)
            .finish_non_exhaustive()
    }
}
