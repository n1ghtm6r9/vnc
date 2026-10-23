use std::io::Write;

use rfb::ENCODING_APPLE_KEYBOARD_INPUT;

use super::write_media_rect;

pub fn write_input_source_rect(stream: &mut impl Write, source: &str) {
    let length = u16::try_from(source.len()).expect("a test input source name fits a 16-bit length");
    let mut payload = vec![0, 1, 0, 0, 0, 0];
    payload.extend_from_slice(&length.to_be_bytes());
    payload.extend_from_slice(source.as_bytes());
    write_media_rect(stream, ENCODING_APPLE_KEYBOARD_INPUT, &payload);
}
