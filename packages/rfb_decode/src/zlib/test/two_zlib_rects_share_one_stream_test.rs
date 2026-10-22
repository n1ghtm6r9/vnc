use rfb::{ENCODING_ZLIB, Rect};

use crate::test_support::{DecodeHarness, ZlibFixture, length_prefixed, rgba_pixel, wire_pixel};

#[test]
fn test_two_zlib_rects_share_one_stream() {
    let mut harness = DecodeHarness::new(8, 4);
    let mut fixture = ZlibFixture::new();
    let pixels: Vec<u8> = (0..8u8)
        .flat_map(|index| wire_pixel(index * 20, index * 10 + 1, 255 - index))
        .collect();
    let first = length_prefixed(&fixture.compress(&pixels));
    let second = length_prefixed(&fixture.compress(&pixels));
    assert!(
        second.len() < first.len(),
        "the second chunk is a back reference into the first, so it only decodes with the stream history"
    );

    harness
        .decode(ENCODING_ZLIB, Rect::new(0, 0, 4, 2), &first)
        .expect("the first zlib rect decodes");
    harness
        .decode(ENCODING_ZLIB, Rect::new(4, 2, 4, 2), &second)
        .expect("the second zlib rect decodes with the history of the first");
    assert_eq!(
        harness.pixel(0, 0),
        rgba_pixel(0, 1, 255),
        "the first pixel of the first rect"
    );
    assert_eq!(
        harness.pixel(3, 1),
        rgba_pixel(140, 71, 248),
        "the last pixel of the first rect"
    );
    assert_eq!(
        harness.pixel(4, 2),
        rgba_pixel(0, 1, 255),
        "the first pixel of the second rect"
    );
    assert_eq!(
        harness.pixel(7, 3),
        rgba_pixel(140, 71, 248),
        "the last pixel of the second rect"
    );
}
