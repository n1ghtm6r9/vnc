use super::ZrleDecoder;
use crate::{Framebuffer, OPAQUE_BLACK, PixelConverter, ZRLE_MAX_PALETTE_SIZE};

impl<'a> ZrleDecoder<'a> {
    pub(crate) fn new(
        data: &'a [u8],
        converter: &'a PixelConverter,
        framebuffer: &'a mut Framebuffer,
    ) -> ZrleDecoder<'a> {
        ZrleDecoder {
            data,
            position: 0,
            converter,
            framebuffer,
            palette: [OPAQUE_BLACK; ZRLE_MAX_PALETTE_SIZE],
        }
    }
}
