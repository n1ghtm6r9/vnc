use rfb::{ENCODING_ZRLE, Rect};

use crate::test_support::{DecodeHarness, ZlibFixture, compact_pixel, rgba_pixel};

#[test]
fn test_a_plain_rle_tile_repeats_its_runs() {
    let mut harness = DecodeHarness::new(20, 20);
    let mut tile = vec![128u8];
    tile.extend_from_slice(&compact_pixel(255, 0, 0));
    tile.extend_from_slice(&[255, 10]);
    tile.extend_from_slice(&compact_pixel(0, 255, 0));
    tile.push(0);
    tile.extend_from_slice(&compact_pixel(0, 0, 255));
    tile.push(132);
    let payload = ZlibFixture::new().payload(&tile);
    harness
        .decode(ENCODING_ZRLE, Rect::new(0, 0, 20, 20), &payload)
        .expect("a plain RLE tile decodes");
    for index in 0..400u16 {
        let expected = match index {
            0..266 => rgba_pixel(255, 0, 0),
            266 => rgba_pixel(0, 255, 0),
            _ => rgba_pixel(0, 0, 255),
        };
        assert_eq!(
            harness.pixel(index % 20, index / 20),
            expected,
            "pixel {index} belongs to the run covering it"
        );
    }
}
