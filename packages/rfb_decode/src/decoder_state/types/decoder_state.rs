use std::collections::HashMap;

use flate2::Decompress;

use crate::{CursorShape, MvsState};

#[derive(Debug)]
pub struct DecoderState {
    pub(crate) zlib_stream: Decompress,
    pub(crate) zrle_stream: Decompress,
    pub(crate) tight_streams: [Decompress; 4],
    pub(crate) compressed: Vec<u8>,
    pub(crate) inflated: Vec<u8>,
    pub(crate) wire: Vec<u8>,
    pub(crate) mvs: MvsState,
    pub(crate) apple_cursors: HashMap<u32, CursorShape>,
}

impl Default for DecoderState {
    fn default() -> DecoderState {
        DecoderState::new()
    }
}
