use rfb::{ENCODING_ZRLE, Rect};

use crate::test_support::{DecodeHarness, ZlibFixture, rgb565_big_endian, rgba_pixel};

#[test]
fn test_zrle_pixels_in_a_16_bit_format_use_two_bytes() {
    let mut harness = DecodeHarness::with_format(4, 2, &rgb565_big_endian());
    let mut fixture = ZlibFixture::new();
    let raw = fixture.payload(&[0, 0x84, 0x08, 0xFF, 0xFF]);
    let solid = fixture.payload(&[1, 0x00, 0x1F]);
    harness
        .decode(ENCODING_ZRLE, Rect::new(0, 0, 2, 1), &raw)
        .expect("a raw 16 bit ZRLE tile decodes");
    harness
        .decode(ENCODING_ZRLE, Rect::new(2, 0, 2, 2), &solid)
        .expect("a solid 16 bit ZRLE tile decodes");
    assert_eq!(
        harness.pixel(0, 0),
        rgba_pixel(132, 130, 66),
        "the first 16 bit pixel is scaled to 8 bits per channel"
    );
    assert_eq!(
        harness.pixel(1, 0),
        rgba_pixel(255, 255, 255),
        "the second 16 bit pixel is white"
    );
    assert_eq!(
        harness.pixel(3, 1),
        rgba_pixel(0, 0, 255),
        "the solid tile is blue"
    );
    assert_eq!(
        harness.pixel(0, 1),
        rgba_pixel(0, 0, 0),
        "the row below the raw rect is untouched"
    );
}
