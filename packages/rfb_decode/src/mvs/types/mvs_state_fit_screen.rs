use super::{MvsState, MvsTileDescriptor};
use crate::MVS_TILE_SIDE;

impl MvsState {
    pub(super) fn fit_screen(&mut self, width: u16, height: u16) {
        if (self.screen_width, self.screen_height) == (width, height) {
            return;
        }
        let side = usize::from(MVS_TILE_SIDE);
        self.screen_width = width;
        self.screen_height = height;
        self.screen_tiles_across = usize::from(width).div_ceil(side);
        let tiles_down = usize::from(height).div_ceil(side);
        self.descriptors =
            vec![MvsTileDescriptor::default(); self.screen_tiles_across * tiles_down];
    }
}
