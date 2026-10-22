use rfb::{ENCODING_RAW, Rect};

use crate::test_support::{DecodeHarness, rgba_pixel};

#[test]
fn test_a_raw_rect_taller_than_one_band_is_read_in_pieces() {
    let (width, height) = (2048u16, 70u16);
    let mut harness = DecodeHarness::new(width, height);
    let colour = |index: usize| {
        [
            (index % 251) as u8,
            (index % 241) as u8,
            (index % 239) as u8,
        ]
    };
    let payload: Vec<u8> = (0..usize::from(width) * usize::from(height))
        .flat_map(|index| {
            let [red, green, blue] = colour(index);
            [red, green, blue, 0]
        })
        .collect();
    harness
        .decode(ENCODING_RAW, Rect::new(0, 0, width, height), &payload)
        .expect("a large raw rect decodes");
    for (x, y) in [
        (0u16, 0u16),
        (2047, 31),
        (0, 32),
        (5, 33),
        (100, 63),
        (7, 64),
        (2047, 69),
    ] {
        let [red, green, blue] = colour(usize::from(y) * usize::from(width) + usize::from(x));
        assert_eq!(
            harness.pixel(x, y),
            rgba_pixel(red, green, blue),
            "pixel ({x}, {y}) survives the band boundaries"
        );
    }
}
