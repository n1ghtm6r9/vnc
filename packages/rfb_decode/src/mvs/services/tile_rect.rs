use rfb::Rect;

use crate::MVS_TILE_SIDE;

pub(crate) fn tile_rect(rect: Rect, tiles_across: usize, index: usize) -> Rect {
    let side = usize::from(MVS_TILE_SIDE);
    let left = usize::from(rect.x) + index % tiles_across * side;
    let top = usize::from(rect.y) + index / tiles_across * side;
    let right = usize::from(rect.x) + usize::from(rect.width);
    let bottom = usize::from(rect.y) + usize::from(rect.height);
    let narrow = |value: usize| u16::try_from(value).unwrap_or(u16::MAX);
    Rect::new(
        narrow(left),
        narrow(top),
        narrow(right.saturating_sub(left).min(side)),
        narrow(bottom.saturating_sub(top).min(side)),
    )
}
