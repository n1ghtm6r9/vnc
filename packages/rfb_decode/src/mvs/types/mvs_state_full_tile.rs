use rfb::Rect;

use super::{MvsFullPass, MvsState};
use crate::{
    DecodeError, Framebuffer, MVS_TILE_SIDE, MVS_WHITE, OPAQUE_BLACK, copy_tile, put_tile,
    read_two_colour_bitmap,
};

impl MvsState {
    pub(super) fn full_tile(
        &mut self,
        pass: &mut MvsFullPass<'_>,
        framebuffer: &mut Framebuffer,
        kind: u32,
        tile: Rect,
    ) -> Result<(), DecodeError> {
        let screen_index = self.screen_index(tile);
        let position = (i32::from(tile.x), i32::from(tile.y));
        if let Some(descriptor) = self.descriptors.get_mut(screen_index) {
            descriptor.frame = self.frame;
            descriptor.reference = None;
        }
        let pixels = match kind {
            0 => Some([MVS_WHITE; 64]),
            1 => {
                self.record_match(
                    screen_index,
                    screen_index.checked_sub(1),
                    pass.previous_tile,
                );
                copy_tile(framebuffer, pass.previous_tile, tile);
                None
            }
            2 => {
                let above = (position.0, position.1 - i32::from(MVS_TILE_SIDE));
                let reference = screen_index.checked_sub(self.screen_tiles_across);
                self.record_match(screen_index, reference, above);
                copy_tile(framebuffer, above, tile);
                None
            }
            3 => Some(read_two_colour_bitmap(
                &mut pass.body,
                MVS_WHITE,
                OPAQUE_BLACK,
            )?),
            4 => Some(self.solid_or_two_colour_tile(&mut pass.body)?),
            5 => {
                let pixels = self.full_dct_tile(pass, screen_index)?;
                pass.previous_dct = Some(screen_index);
                Some(pixels)
            }
            6 => {
                let slot = pass.body.read_bits(16)? as usize;
                self.load_cached_tile(slot)
            }
            _ => self.load_cached_tile(self.cache.next_slot()),
        };
        if let Some(pixels) = pixels {
            put_tile(framebuffer, tile, &pixels);
        }
        pass.previous_tile = position;
        Ok(())
    }
}
