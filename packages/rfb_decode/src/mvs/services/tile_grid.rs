use rfb::Rect;

use crate::MVS_TILE_SIDE;

pub(crate) fn tile_grid(rect: Rect) -> (usize, usize) {
    let side = usize::from(MVS_TILE_SIDE);
    let across = usize::from(rect.width).div_ceil(side);
    let down = usize::from(rect.height).div_ceil(side);
    (across, across * down)
}
