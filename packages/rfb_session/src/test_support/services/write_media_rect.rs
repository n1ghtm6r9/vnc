use std::io::Write;

use rfb::{Rect, RectHeader};

pub fn write_media_rect(stream: &mut impl Write, encoding: i32, body: &[u8]) {
    RectHeader {
        rect: Rect::new(0, 0, 0, 0),
        encoding,
    }
    .write(stream)
    .expect("the fake server sends a media rect header");
    let length = u16::try_from(body.len()).expect("a test media body fits a 16-bit length");
    stream
        .write_all(&length.to_be_bytes())
        .expect("the fake server sends the media length");
    stream
        .write_all(body)
        .expect("the fake server sends the media body");
}
