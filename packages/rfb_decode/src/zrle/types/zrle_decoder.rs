use crate::{Framebuffer, PixelConverter, ZRLE_MAX_PALETTE_SIZE};

pub(crate) struct ZrleDecoder<'a> {
    pub(super) data: &'a [u8],
    pub(super) position: usize,
    pub(super) converter: &'a PixelConverter,
    pub(super) framebuffer: &'a mut Framebuffer,
    pub(super) palette: [u32; ZRLE_MAX_PALETTE_SIZE],
}
