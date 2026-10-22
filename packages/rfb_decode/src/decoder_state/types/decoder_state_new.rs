use std::collections::HashMap;

use flate2::Decompress;

use super::DecoderState;
use crate::MvsState;

impl DecoderState {
    pub fn new() -> DecoderState {
        DecoderState {
            zlib_stream: Decompress::new(true),
            zrle_stream: Decompress::new(true),
            tight_streams: std::array::from_fn(|_| Decompress::new(true)),
            compressed: Vec::new(),
            inflated: Vec::new(),
            wire: Vec::new(),
            mvs: MvsState::new(),
            apple_cursors: HashMap::new(),
        }
    }
}
