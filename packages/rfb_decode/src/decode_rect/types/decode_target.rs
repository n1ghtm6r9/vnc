use crate::{DecoderState, Framebuffer, PixelConverter};

pub struct DecodeTarget<'a> {
    pub converter: &'a PixelConverter,
    pub state: &'a mut DecoderState,
    pub framebuffer: &'a mut Framebuffer,
}
