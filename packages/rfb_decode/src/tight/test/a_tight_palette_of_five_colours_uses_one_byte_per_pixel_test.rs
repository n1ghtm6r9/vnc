use rfb::{ENCODING_TIGHT, Rect};

use crate::test_support::{DecodeHarness, ZlibFixture, compact_pixel, rgba_pixel};

#[test]
fn test_a_tight_palette_of_five_colours_uses_one_byte_per_pixel() {
    let mut harness = DecodeHarness::new(4, 4);
    let palette = [
        (250, 0, 0),
        (0, 250, 0),
        (0, 0, 250),
        (250, 250, 0),
        (0, 250, 250),
    ];
    let indices = [0u8, 1, 2, 3, 4, 4, 3, 2, 1, 0, 0, 1, 2, 3, 4, 0];
    let mut payload = vec![0x40u8, 1, 4];
    for (red, green, blue) in palette {
        payload.extend_from_slice(&compact_pixel(red, green, blue));
    }
    payload.extend_from_slice(&ZlibFixture::new().tight_payload(&indices));
    harness
        .decode(ENCODING_TIGHT, Rect::new(0, 0, 4, 4), &payload)
        .expect("a five colour palette rect decodes");
    for (position, &index) in indices.iter().enumerate() {
        let (red, green, blue) = palette[usize::from(index)];
        let (x, y) = ((position % 4) as u16, (position / 4) as u16);
        assert_eq!(
            harness.pixel(x, y),
            rgba_pixel(red, green, blue),
            "pixel ({x}, {y}) takes palette entry {index}"
        );
    }
}
