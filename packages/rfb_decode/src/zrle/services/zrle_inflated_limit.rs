use rfb::Rect;

use crate::{ZRLE_MAX_PALETTE_SIZE, ZRLE_TILE_SIZE};

pub(crate) fn zrle_inflated_limit(rect: Rect, pixel_bytes: usize) -> usize {
    let tile_size = usize::from(ZRLE_TILE_SIZE);
    let tiles =
        usize::from(rect.width).div_ceil(tile_size) * usize::from(rect.height).div_ceil(tile_size);
    tiles * (1 + ZRLE_MAX_PALETTE_SIZE * pixel_bytes) + rect.area() * (pixel_bytes + 1)
}
