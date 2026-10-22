use rfb::Rect;

use super::{MvsBits, MvsState};
use crate::{DecodeError, Framebuffer, put_tile};

impl MvsState {
    pub(super) fn partial_tile(
        &mut self,
        bits: &mut MvsBits<'_>,
        framebuffer: &mut Framebuffer,
        limits: [usize; 2],
        tile: Rect,
    ) -> Result<bool, DecodeError> {
        let screen_index = self.screen_index(tile);
        let pixels = match bits.read_bits(2)? {
            0 => return Ok(false),
            1 => Some(self.partial_dct_tile(bits, screen_index, limits)?),
            2 => return Ok(self.match_tile(framebuffer, screen_index, tile)),
            _ => {
                let slot = if bits.read_bit()? == 1 {
                    self.cache.next_slot()
                } else {
                    bits.read_bits(16)? as usize
                };
                self.load_cached_tile(slot)
            }
        };
        let Some(pixels) = pixels else {
            return Ok(false);
        };
        put_tile(framebuffer, tile, &pixels);
        Ok(true)
    }
}
