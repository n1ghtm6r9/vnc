use rfb::{ENCODING_RAW, Rect};

use crate::test_support::{DecodeHarness, rgb565_big_endian, rgba_pixel};

#[test]
fn test_raw_pixels_in_a_16_bit_big_endian_format_are_scaled() {
    let mut harness = DecodeHarness::with_format(4, 2, &rgb565_big_endian());
    harness
        .decode(
            ENCODING_RAW,
            Rect::new(1, 0, 2, 1),
            &[0x84, 0x08, 0xFF, 0xFF],
        )
        .expect("a 16 bit raw rect decodes");
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
    assert_eq!(
        harness.pixel(0, 0),
        rgba_pixel(0, 0, 0),
        "the pixel left of the rect is untouched"
    );
}
