use rfb::{ENCODING_TIGHT, Rect};

use crate::test_support::{DecodeHarness, ZlibFixture, compact_pixel, rgba_pixel};

#[test]
fn test_a_tight_fill_still_resets_the_streams_named_in_the_control_byte() {
    let mut harness = DecodeHarness::new(4, 3);
    let pixels: Vec<u8> = (0..36u8).map(|index| index * 3 + 1).collect();
    let mut first = vec![0x20u8];
    first.extend_from_slice(&ZlibFixture::new().tight_payload(&pixels));
    harness
        .decode(ENCODING_TIGHT, Rect::new(0, 0, 4, 3), &first)
        .expect("a rect on stream 2 decodes");
    let mut fill = vec![0x84u8];
    fill.extend_from_slice(&compact_pixel(9, 8, 7));
    harness
        .decode(ENCODING_TIGHT, Rect::new(0, 0, 4, 3), &fill)
        .expect("a fill that resets stream 2 decodes");
    assert_eq!(
        harness.pixel(3, 2),
        rgba_pixel(9, 8, 7),
        "the fill painted the rect"
    );
    let mut after = vec![0x20u8];
    after.extend_from_slice(&ZlibFixture::new().tight_payload(&pixels));
    harness
        .decode(ENCODING_TIGHT, Rect::new(0, 0, 4, 3), &after)
        .expect("a fresh stream 2 decodes because the fill reset it");
    assert_eq!(
        harness.pixel(0, 0),
        rgba_pixel(1, 4, 7),
        "the rect after the reset holds the new pixels"
    );
}
