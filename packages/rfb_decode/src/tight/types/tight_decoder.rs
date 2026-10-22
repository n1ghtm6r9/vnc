use std::io::Read;

use rfb::Rect;

use crate::{DecoderState, Framebuffer, PixelConverter, TIGHT_MAX_PALETTE};

pub(crate) struct TightDecoder<'a, R: Read> {
    pub(super) reader: &'a mut R,
    pub(super) rect: Rect,
    pub(super) converter: &'a PixelConverter,
    pub(super) state: &'a mut DecoderState,
    pub(super) framebuffer: &'a mut Framebuffer,
    pub(super) palette: [u32; TIGHT_MAX_PALETTE],
}
