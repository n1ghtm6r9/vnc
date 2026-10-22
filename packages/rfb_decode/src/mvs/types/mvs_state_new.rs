use super::{MvsState, MvsTileCache};
use crate::{
    MVS_DEFAULT_CHROMA_QUANTIZATION, MVS_DEFAULT_CLEAR_COLOUR, MVS_DEFAULT_LUMA_QUANTIZATION,
    MVS_WHITE, OPAQUE_BLACK,
};

impl MvsState {
    pub(crate) fn new() -> MvsState {
        MvsState {
            luma_quantization: MVS_DEFAULT_LUMA_QUANTIZATION,
            chroma_quantization: MVS_DEFAULT_CHROMA_QUANTIZATION,
            quantization_generation: 1,
            coefficients: [[0; 64]; 3],
            previous: [[0; 64]; 3],
            solid: OPAQUE_BLACK,
            set_colour: MVS_WHITE,
            clear_colour: MVS_DEFAULT_CLEAR_COLOUR,
            frame: 0,
            screen_width: 0,
            screen_height: 0,
            screen_tiles_across: 0,
            descriptors: Vec::new(),
            cache: MvsTileCache::default(),
        }
    }
}
