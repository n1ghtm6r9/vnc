use rfb::{ENCODING_ZRLE, Rect};

use crate::test_support::{DecodeHarness, ZlibFixture, compact_pixel, rgba_pixel};

#[test]
fn test_a_palette_rle_tile_repeats_its_runs() {
    let mut harness = DecodeHarness::new(10, 4);
    let mut tile = vec![131u8];
    tile.extend_from_slice(&compact_pixel(250, 0, 0));
    tile.extend_from_slice(&compact_pixel(0, 250, 0));
    tile.extend_from_slice(&compact_pixel(0, 0, 250));
    tile.extend_from_slice(&[1, 0x80, 19, 2, 0x81, 17]);
    let payload = ZlibFixture::new().payload(&tile);
    harness
        .decode(ENCODING_ZRLE, Rect::new(0, 0, 10, 4), &payload)
        .expect("a palette RLE tile decodes");
    for index in 0..40u16 {
        let expected = match index {
            0 => rgba_pixel(0, 250, 0),
            1..21 => rgba_pixel(250, 0, 0),
            21 => rgba_pixel(0, 0, 250),
            _ => rgba_pixel(0, 250, 0),
        };
        assert_eq!(
            harness.pixel(index % 10, index / 10),
            expected,
            "pixel {index} belongs to the run covering it"
        );
    }
}
