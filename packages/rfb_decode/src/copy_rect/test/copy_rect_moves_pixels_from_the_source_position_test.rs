use rfb::{ENCODING_COPY_RECT, Rect};

use crate::RectOutcome;
use crate::test_support::{DecodeHarness, rgba_pixel};

#[test]
fn test_copy_rect_moves_pixels_from_the_source_position() {
    let mut harness = DecodeHarness::new(8, 4);
    harness
        .framebuffer
        .fill_rect(Rect::new(0, 0, 2, 2), rgba_pixel(200, 10, 10));
    let payload = [0u16.to_be_bytes(), 0u16.to_be_bytes()].concat();
    let target = Rect::new(5, 2, 2, 2);
    let outcome = harness
        .decode(ENCODING_COPY_RECT, target, &payload)
        .expect("a copy inside the frame decodes");
    assert_eq!(
        outcome,
        RectOutcome::Pixels(target),
        "the outcome names the destination rect"
    );
    assert_eq!(
        harness.pixel(5, 2),
        rgba_pixel(200, 10, 10),
        "the destination top left holds the copied pixel"
    );
    assert_eq!(
        harness.pixel(6, 3),
        rgba_pixel(200, 10, 10),
        "the destination bottom right holds the copied pixel"
    );
    assert_eq!(
        harness.pixel(4, 2),
        rgba_pixel(0, 0, 0),
        "the pixel left of the destination is untouched"
    );
    assert_eq!(
        harness.pixel(0, 0),
        rgba_pixel(200, 10, 10),
        "the source keeps its pixels"
    );
}
