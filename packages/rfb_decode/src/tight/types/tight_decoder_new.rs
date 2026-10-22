use std::io::Read;

use rfb::Rect;

use super::TightDecoder;
use crate::{DecodeTarget, OPAQUE_BLACK, TIGHT_MAX_PALETTE};

impl<'a, R: Read> TightDecoder<'a, R> {
    pub(crate) fn new(
        reader: &'a mut R,
        rect: Rect,
        target: DecodeTarget<'a>,
    ) -> TightDecoder<'a, R> {
        TightDecoder {
            reader,
            rect,
            converter: target.converter,
            state: target.state,
            framebuffer: target.framebuffer,
            palette: [OPAQUE_BLACK; TIGHT_MAX_PALETTE],
        }
    }
}
