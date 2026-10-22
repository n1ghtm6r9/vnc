use rfb::{ENCODING_ZRLE, Rect};

use crate::test_support::{DecodeHarness, ZlibFixture, compact_pixel, rgba_pixel};

#[test]
fn test_a_packed_palette_of_two_colours_uses_one_bit_per_pixel() {
    let mut harness = DecodeHarness::new(9, 2);
    let palette = [(250, 0, 0), (0, 0, 250)];
    let indices = [
        [0usize, 1, 0, 1, 1, 0, 0, 1, 1],
        [1, 1, 1, 1, 1, 1, 1, 1, 0],
    ];
    let mut tile = vec![2u8];
    for (red, green, blue) in palette {
        tile.extend_from_slice(&compact_pixel(red, green, blue));
    }
    tile.extend_from_slice(&[0b0101_1001, 0b1000_0000, 0b1111_1111, 0b0000_0000]);
    let payload = ZlibFixture::new().payload(&tile);
    harness
        .decode(ENCODING_ZRLE, Rect::new(0, 0, 9, 2), &payload)
        .expect("a two colour packed palette tile decodes");
    for (y, row) in indices.iter().enumerate() {
        for (x, &index) in row.iter().enumerate() {
            let (red, green, blue) = palette[index];
            assert_eq!(
                harness.pixel(x as u16, y as u16),
                rgba_pixel(red, green, blue),
                "pixel ({x}, {y}) takes palette entry {index}"
            );
        }
    }
}
