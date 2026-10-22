use rfb::{ENCODING_HEXTILE, Rect};

use crate::test_support::{DecodeHarness, rgba_pixel, wire_pixel};

#[test]
fn test_partial_edge_tiles_are_clipped_to_the_rect() {
    let mut harness = DecodeHarness::new(30, 24);
    let mut payload = Vec::new();
    for red in 1..=4u8 {
        payload.push(2);
        payload.extend_from_slice(&wire_pixel(red, 0, 0));
    }
    harness
        .decode(ENCODING_HEXTILE, Rect::new(3, 2, 20, 18), &payload)
        .expect("a rect with edge tiles decodes");
    assert_eq!(
        harness.pixel(3, 2),
        rgba_pixel(1, 0, 0),
        "the first tile is a full 16 by 16 tile"
    );
    assert_eq!(
        harness.pixel(19, 2),
        rgba_pixel(2, 0, 0),
        "the right edge tile starts after sixteen columns"
    );
    assert_eq!(
        harness.pixel(3, 18),
        rgba_pixel(3, 0, 0),
        "the bottom edge tile starts after sixteen rows"
    );
    assert_eq!(
        harness.pixel(19, 18),
        rgba_pixel(4, 0, 0),
        "the corner tile is the last one"
    );
    assert_eq!(
        harness.pixel(22, 19),
        rgba_pixel(4, 0, 0),
        "the corner tile reaches the bottom right of the rect"
    );
    assert_eq!(
        harness.pixel(23, 2),
        rgba_pixel(0, 0, 0),
        "the column right of the rect is untouched"
    );
    assert_eq!(
        harness.pixel(3, 20),
        rgba_pixel(0, 0, 0),
        "the row below the rect is untouched"
    );
}
