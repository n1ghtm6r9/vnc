use rfb::{ENCODING_TIGHT, Rect};

use crate::test_support::{DecodeHarness, compact_pixel, rgba_pixel};

#[test]
fn test_a_tight_palette_of_two_colours_uses_one_bit_per_pixel() {
    let mut harness = DecodeHarness::new(10, 3);
    let palette = [(250, 10, 10), (10, 10, 250)];
    let indices = [
        [1usize, 0, 1, 1, 0, 0, 1, 0, 1, 1],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 0],
    ];
    let mut payload = vec![0x40u8, 1, 1];
    for (red, green, blue) in palette {
        payload.extend_from_slice(&compact_pixel(red, green, blue));
    }
    payload.extend_from_slice(&[
        0b1011_0010,
        0b1100_0000,
        0b0000_0000,
        0b0100_0000,
        0b1111_1111,
        0b1000_0000,
    ]);
    harness
        .decode(ENCODING_TIGHT, Rect::new(0, 0, 10, 3), &payload)
        .expect("a two colour palette rect decodes");
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
