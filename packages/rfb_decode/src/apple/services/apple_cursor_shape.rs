use flate2::{Decompress, FlushDecompress};
use rfb::Rect;

use crate::{CursorShape, MAX_CURSOR_SIDE};

pub(crate) fn apple_cursor_shape(data: &[u8], rect: Rect) -> Option<CursorShape> {
    let fits = |side: u16| (1..=MAX_CURSOR_SIDE).contains(&side);
    if !fits(rect.width) || !fits(rect.height) {
        return None;
    }
    let count = usize::from(rect.width) * usize::from(rect.height);
    let mut raw = Vec::with_capacity(count * 5);
    Decompress::new(true)
        .decompress_vec(data, &mut raw, FlushDecompress::Sync)
        .ok()?;
    if raw.len() < count * 5 {
        return None;
    }
    let (colours, alpha) = raw.split_at(count * 4);
    let (pixels, _) = colours.as_chunks::<4>();
    let rgba = pixels
        .iter()
        .zip(alpha)
        .flat_map(|(&[blue, green, red, _], &alpha)| [red, green, blue, alpha])
        .collect();
    Some(CursorShape {
        width: rect.width,
        height: rect.height,
        hotspot_x: rect.x,
        hotspot_y: rect.y,
        rgba,
    })
}
