use crate::{DecoderState, Framebuffer, PixelConverter};

pub struct DecodeHarness {
    pub converter: PixelConverter,
    pub state: DecoderState,
    pub framebuffer: Framebuffer,
}
