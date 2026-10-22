use rfb_session::CursorShape;

use crate::{MAX_CURSOR_LENGTH, ScaledCursor, area_weights, blend_cursor_pixel};

pub fn scale_cursor(shape: &CursorShape, scale: f64) -> Option<ScaledCursor> {
    let pixels = usize::from(shape.width) * usize::from(shape.height);
    if pixels == 0 || shape.rgba.len() < pixels * 4 || !scale.is_finite() || scale <= 0.0 {
        return None;
    }
    let longest = f64::from(shape.width.max(shape.height));
    let scale = scale.min(f64::from(MAX_CURSOR_LENGTH) / longest);
    let scaled = |length: u16| (f64::from(length) * scale).round().clamp(1.0, f64::from(MAX_CURSOR_LENGTH)) as u16;
    let hotspot = |hotspot: u16, length: u16| ((f64::from(hotspot) * scale) as u16).min(length - 1);
    let width = scaled(shape.width);
    let height = scaled(shape.height);
    let columns = area_weights(shape.width, width);
    let rows = area_weights(shape.height, height);
    let rgba = rows
        .iter()
        .flat_map(|row| columns.iter().flat_map(|column| blend_cursor_pixel(shape, row, column)))
        .collect();
    Some(ScaledCursor {
        width,
        height,
        hotspot_x: hotspot(shape.hotspot_x, width),
        hotspot_y: hotspot(shape.hotspot_y, height),
        rgba,
    })
}
