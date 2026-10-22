use rfb::Rect;

use crate::{Framebuffer, MVS_TILE_SIDE};

pub(crate) fn put_tile(framebuffer: &mut Framebuffer, tile: Rect, pixels: &[u32; 64]) {
    let side = usize::from(MVS_TILE_SIDE);
    let rows = tile.y..tile.y.saturating_add(tile.height);
    for (y, line) in rows.zip(pixels.chunks_exact(side)) {
        let source = &line[..usize::from(tile.width).min(side)];
        let target = framebuffer.row_mut(tile.x, y, tile.width);
        if target.len() == source.len() {
            target.copy_from_slice(source);
        }
    }
}
