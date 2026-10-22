use rfb::Rect;

use super::ZrleDecoder;
use crate::{
    DecodeError, ZRLE_LOW_SEVEN_BITS, ZRLE_MAX_PACKED_PALETTE_SIZE, ZRLE_RLE_FLAG,
    unused_subencoding,
};

impl ZrleDecoder<'_> {
    pub(super) fn decode_tile(&mut self, tile: Rect) -> Result<(), DecodeError> {
        let subencoding = self.take_byte()?;
        let palette_size = usize::from(subencoding & ZRLE_LOW_SEVEN_BITS);
        if subencoding & ZRLE_RLE_FLAG == 0 {
            match palette_size {
                0 => self.raw_tile(tile),
                1 => self.solid_tile(tile),
                2..=ZRLE_MAX_PACKED_PALETTE_SIZE => self.packed_palette_tile(tile, palette_size),
                _ => Err(unused_subencoding(subencoding)),
            }
        } else {
            match palette_size {
                1 => Err(unused_subencoding(subencoding)),
                _ => self.rle_tile(tile, palette_size),
            }
        }
    }
}
