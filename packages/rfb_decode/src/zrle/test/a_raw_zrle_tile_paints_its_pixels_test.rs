use rfb::{ENCODING_ZRLE, Rect};

use crate::test_support::{DecodeHarness, ZlibFixture, compact_pixel, rgba_pixel};

#[test]
fn test_a_raw_zrle_tile_paints_its_pixels() {
    let mut harness = DecodeHarness::new(5, 4);
    let colours = [
        (10, 20, 30),
        (40, 50, 60),
        (70, 80, 90),
        (100, 110, 120),
        (130, 140, 150),
        (160, 170, 180),
    ];
    let mut tile = vec![0u8];
    for (red, green, blue) in colours {
        tile.extend_from_slice(&compact_pixel(red, green, blue));
    }
    let payload = ZlibFixture::new().payload(&tile);
    harness
        .decode(ENCODING_ZRLE, Rect::new(1, 1, 3, 2), &payload)
        .expect("a raw ZRLE tile decodes");
    for (index, (red, green, blue)) in colours.into_iter().enumerate() {
        let (x, y) = (1 + (index % 3) as u16, 1 + (index / 3) as u16);
        assert_eq!(
            harness.pixel(x, y),
            rgba_pixel(red, green, blue),
            "raw pixel {index} lands at ({x}, {y})"
        );
    }
    assert_eq!(
        harness.pixel(0, 0),
        rgba_pixel(0, 0, 0),
        "a pixel outside the rect is untouched"
    );
}
