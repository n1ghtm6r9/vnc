use rfb::{ENCODING_ZRLE, Rect};

use crate::test_support::{DecodeHarness, ZlibFixture, compact_pixel, rgba_pixel};

#[test]
fn test_a_packed_palette_of_three_colours_uses_two_bits_per_pixel() {
    let mut harness = DecodeHarness::new(5, 2);
    let palette = [(250, 0, 0), (0, 250, 0), (0, 0, 250)];
    let indices = [[0usize, 1, 2, 2, 1], [2, 0, 0, 1, 2]];
    let mut tile = vec![3u8];
    for (red, green, blue) in palette {
        tile.extend_from_slice(&compact_pixel(red, green, blue));
    }
    tile.extend_from_slice(&[0b0001_1010, 0b0100_0000, 0b1000_0001, 0b1000_0000]);
    let payload = ZlibFixture::new().payload(&tile);
    harness
        .decode(ENCODING_ZRLE, Rect::new(0, 0, 5, 2), &payload)
        .expect("a three colour packed palette tile decodes");
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
