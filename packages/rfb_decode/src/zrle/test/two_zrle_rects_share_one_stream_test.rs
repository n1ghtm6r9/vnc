use rfb::{ENCODING_ZRLE, Rect};

use crate::test_support::{DecodeHarness, ZlibFixture, compact_pixel};

#[test]
fn test_two_zrle_rects_share_one_stream() {
    let mut harness = DecodeHarness::new(8, 4);
    let mut fixture = ZlibFixture::new();
    let mut tile = vec![0u8];
    for index in 0..16u8 {
        tile.extend_from_slice(&compact_pixel(index * 16, 255 - index * 16, index * 3));
    }
    let first = fixture.payload(&tile);
    let second = fixture.payload(&tile);
    assert!(
        second.len() < first.len(),
        "the second chunk reuses the stream history and is shorter"
    );
    harness
        .decode(ENCODING_ZRLE, Rect::new(0, 0, 4, 4), &first)
        .expect("the first chunk decodes");
    harness
        .decode(ENCODING_ZRLE, Rect::new(4, 0, 4, 4), &second)
        .expect("the second chunk decodes with the shared stream");
    for y in 0..4 {
        for x in 0..4 {
            assert_eq!(
                harness.pixel(x + 4, y),
                harness.pixel(x, y),
                "pixel ({x}, {y}) is the same in both rects"
            );
        }
    }
    assert_ne!(
        harness.pixel(0, 0),
        harness.pixel(3, 3),
        "the tile holds more than one colour"
    );
}
