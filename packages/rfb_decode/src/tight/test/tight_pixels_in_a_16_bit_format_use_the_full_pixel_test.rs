use rfb::{ENCODING_TIGHT, Rect};

use crate::test_support::{DecodeHarness, rgb565_big_endian, rgba_pixel};

#[test]
fn test_tight_pixels_in_a_16_bit_format_use_the_full_pixel() {
    let mut harness = DecodeHarness::with_format(4, 2, &rgb565_big_endian());
    let payload = [0x00u8, 0x84, 0x08, 0xFF, 0xFF];
    harness
        .decode(ENCODING_TIGHT, Rect::new(1, 0, 2, 1), &payload)
        .expect("a 16 bit copy filter rect decodes");
    assert_eq!(
        harness.pixel(1, 0),
        rgba_pixel(132, 130, 66),
        "the first 16 bit pixel is scaled to 8 bits per channel"
    );
    assert_eq!(
        harness.pixel(2, 0),
        rgba_pixel(255, 255, 255),
        "the second 16 bit pixel is white"
    );
}
