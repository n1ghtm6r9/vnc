use rfb::{ENCODING_HEXTILE, Rect};

use crate::test_support::{DecodeHarness, rgba_pixel, wire_pixel};

#[test]
fn test_a_hextile_raw_tile_is_copied_as_pixels() {
    let mut harness = DecodeHarness::new(8, 4);
    let mut payload = vec![1];
    for index in 0..8u8 {
        payload.extend_from_slice(&wire_pixel(index * 10, 5, 6));
    }
    harness
        .decode(ENCODING_HEXTILE, Rect::new(1, 1, 4, 2), &payload)
        .expect("a raw hextile tile decodes");
    assert_eq!(
        harness.pixel(1, 1),
        rgba_pixel(0, 5, 6),
        "the first raw pixel is the tile top left"
    );
    assert_eq!(
        harness.pixel(4, 1),
        rgba_pixel(30, 5, 6),
        "the fourth raw pixel ends the first row"
    );
    assert_eq!(
        harness.pixel(1, 2),
        rgba_pixel(40, 5, 6),
        "the fifth raw pixel starts the second row"
    );
    assert_eq!(
        harness.pixel(4, 2),
        rgba_pixel(70, 5, 6),
        "the last raw pixel is the tile bottom right"
    );
    assert_eq!(
        harness.pixel(0, 1),
        rgba_pixel(0, 0, 0),
        "the pixel left of the tile is untouched"
    );
}
