use rfb::Rect;

use super::{MvsBits, MvsState};
use crate::{
    DecodeError, Framebuffer, MVS_PARTIAL_UPDATE_END, expect_marker, tile_grid, tile_rect,
};

impl MvsState {
    pub(super) fn partial_update(
        &mut self,
        data: &[u8],
        rect: Rect,
        framebuffer: &mut Framebuffer,
        changed: &mut Rect,
    ) -> Result<(), DecodeError> {
        let [_, red_limit, blue_limit, ..] = *data else {
            return Err(DecodeError::Corrupt(
                "the MVS partial update is shorter than its header".to_owned(),
            ));
        };
        let limits = [
            usize::from(red_limit).min(64),
            usize::from(blue_limit).min(64),
        ];
        let mut bits = MvsBits::new(&data[3..]);
        let (tiles_across, tile_count) = tile_grid(rect);
        for index in 0..tile_count {
            let tile = tile_rect(rect, tiles_across, index);
            if self.partial_tile(&mut bits, framebuffer, limits, tile)? {
                *changed = changed.union(tile);
            }
        }
        expect_marker(&mut bits, 24, MVS_PARTIAL_UPDATE_END)
    }
}
