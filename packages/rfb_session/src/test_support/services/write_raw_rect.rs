use std::io::Write;

use rfb::{ENCODING_RAW, Rect, RectHeader};

pub fn write_raw_rect(stream: &mut impl Write, rect: Rect, rgba: u32) {
    RectHeader {
        rect,
        encoding: ENCODING_RAW,
    }
    .write(stream)
    .expect("the fake server sends a rect header");
    let pixel_count = usize::from(rect.width) * usize::from(rect.height);
    stream
        .write_all(&rgba.to_le_bytes().repeat(pixel_count))
        .expect("the fake server sends the raw pixels");
}
