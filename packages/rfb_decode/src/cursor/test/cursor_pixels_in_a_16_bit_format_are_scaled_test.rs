use rfb::{ENCODING_CURSOR, Rect};

use crate::test_support::{DecodeHarness, rgb565_big_endian};
use crate::{CursorShape, RectOutcome};

#[test]
fn test_cursor_pixels_in_a_16_bit_format_are_scaled() {
    let mut harness = DecodeHarness::with_format(4, 4, &rgb565_big_endian());
    let payload = [0x84, 0x08, 0xFF, 0xFF, 0b1100_0000];
    let outcome = harness
        .decode(ENCODING_CURSOR, Rect::new(1, 0, 2, 1), &payload)
        .expect("a 16 bit cursor decodes");
    let expected = CursorShape {
        width: 2,
        height: 1,
        hotspot_x: 1,
        hotspot_y: 0,
        rgba: vec![132, 130, 66, 255, 255, 255, 255, 255],
    };
    assert_eq!(
        outcome,
        RectOutcome::Cursor(expected),
        "the 16 bit pixels are scaled to 8 bits per channel"
    );
}
