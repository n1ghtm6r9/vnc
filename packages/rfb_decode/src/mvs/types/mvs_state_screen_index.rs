use rfb::Rect;

use super::MvsState;
use crate::MVS_TILE_SIDE;

impl MvsState {
    pub(super) fn screen_index(&self, tile: Rect) -> usize {
        let side = usize::from(MVS_TILE_SIDE);
        usize::from(tile.y) / side * self.screen_tiles_across + usize::from(tile.x) / side
    }
}
