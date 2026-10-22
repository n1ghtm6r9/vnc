use rfb::{ENCODING_ZRLE, Rect};

use crate::test_support::{DecodeHarness, ZlibFixture, compact_pixel, rgba_pixel};

#[test]
fn test_zrle_tiles_cover_a_rect_larger_than_one_tile() {
    let mut harness = DecodeHarness::new(140, 70);
    let colours: Vec<(u8, u8, u8)> = (1..=6u8)
        .map(|index| (index * 40, index * 30, index * 20))
        .collect();
    let mut tiles = Vec::new();
    for &(red, green, blue) in &colours {
        tiles.push(1u8);
        tiles.extend_from_slice(&compact_pixel(red, green, blue));
    }
    let payload = ZlibFixture::new().payload(&tiles);
    harness
        .decode(ENCODING_ZRLE, Rect::new(3, 2, 130, 66), &payload)
        .expect("a six tile rect decodes");
    let probes = [
        (3, 2, 0),
        (66, 65, 0),
        (67, 2, 1),
        (130, 65, 1),
        (131, 2, 2),
        (132, 65, 2),
        (3, 66, 3),
        (66, 67, 3),
        (67, 66, 4),
        (130, 67, 4),
        (131, 66, 5),
        (132, 67, 5),
    ];
    for (x, y, tile) in probes {
        let (red, green, blue) = colours[tile];
        assert_eq!(
            harness.pixel(x, y),
            rgba_pixel(red, green, blue),
            "pixel ({x}, {y}) belongs to tile {tile}"
        );
    }
    assert_eq!(
        harness.pixel(2, 2),
        rgba_pixel(0, 0, 0),
        "a pixel left of the rect is untouched"
    );
    assert_eq!(
        harness.pixel(133, 2),
        rgba_pixel(0, 0, 0),
        "a pixel right of the rect is untouched"
    );
}
