use std::io::Read;

use crate::{DecoderState, Framebuffer, PixelConverter};

pub(crate) struct HextileDecoder<'a, R: Read> {
    pub(super) reader: &'a mut R,
    pub(super) converter: &'a PixelConverter,
    pub(super) state: &'a mut DecoderState,
    pub(super) framebuffer: &'a mut Framebuffer,
    pub(super) background: u32,
    pub(super) foreground: u32,
}
