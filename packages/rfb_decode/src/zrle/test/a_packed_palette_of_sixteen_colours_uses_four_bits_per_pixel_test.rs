use rfb::{ENCODING_ZRLE, Rect};

use crate::test_support::{DecodeHarness, ZlibFixture, compact_pixel, rgba_pixel};

#[test]
fn test_a_packed_palette_of_sixteen_colours_uses_four_bits_per_pixel() {
    let mut harness = DecodeHarness::new(3, 2);
    let palette: Vec<(u8, u8, u8)> = (0..16u8)
        .map(|index| (index * 16, 255 - index * 16, index * 8))
        .collect();
    let indices = [[15usize, 0, 7], [8, 9, 1]];
    let mut tile = vec![16u8];
    for &(red, green, blue) in &palette {
        tile.extend_from_slice(&compact_pixel(red, green, blue));
    }
    tile.extend_from_slice(&[0xF0, 0x70, 0x89, 0x10]);
    let payload = ZlibFixture::new().payload(&tile);
    harness
        .decode(ENCODING_ZRLE, Rect::new(0, 0, 3, 2), &payload)
        .expect("a sixteen colour packed palette tile decodes");
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
