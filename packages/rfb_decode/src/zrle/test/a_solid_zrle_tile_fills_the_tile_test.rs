use rfb::{ENCODING_ZRLE, Rect};

use crate::test_support::{DecodeHarness, ZlibFixture, compact_pixel, rgba_pixel};

#[test]
fn test_a_solid_zrle_tile_fills_the_tile() {
    let mut harness = DecodeHarness::new(6, 5);
    let mut tile = vec![1u8];
    tile.extend_from_slice(&compact_pixel(200, 100, 50));
    let payload = ZlibFixture::new().payload(&tile);
    harness
        .decode(ENCODING_ZRLE, Rect::new(1, 1, 4, 3), &payload)
        .expect("a solid ZRLE tile decodes");
    for y in 1..4 {
        for x in 1..5 {
            assert_eq!(
                harness.pixel(x, y),
                rgba_pixel(200, 100, 50),
                "solid pixel ({x}, {y}) has the tile colour"
            );
        }
    }
    assert_eq!(
        harness.pixel(5, 1),
        rgba_pixel(0, 0, 0),
        "the column right of the tile is untouched"
    );
    assert_eq!(
        harness.pixel(1, 4),
        rgba_pixel(0, 0, 0),
        "the row below the tile is untouched"
    );
}
