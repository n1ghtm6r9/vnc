use rfb::{ENCODING_RRE, Rect};

use crate::RectOutcome;
use crate::test_support::{DecodeHarness, rgba_pixel, rre_payload, rre_subrect, wire_pixel};

#[test]
fn test_rre_paints_the_background_and_every_subrect() {
    let mut harness = DecodeHarness::new(10, 8);
    let rect = Rect::new(1, 1, 6, 5);
    let payload = rre_payload(
        wire_pixel(10, 20, 30),
        &[
            rre_subrect(wire_pixel(200, 0, 0), Rect::new(0, 0, 2, 1)),
            rre_subrect(wire_pixel(0, 200, 0), Rect::new(4, 3, 2, 2)),
        ],
    );
    let outcome = harness
        .decode(ENCODING_RRE, rect, &payload)
        .expect("a well formed RRE rect decodes");
    assert_eq!(
        outcome,
        RectOutcome::Pixels(rect),
        "the outcome names the painted rect"
    );
    assert_eq!(
        harness.pixel(3, 1),
        rgba_pixel(10, 20, 30),
        "the background fills the rect"
    );
    assert_eq!(
        harness.pixel(1, 1),
        rgba_pixel(200, 0, 0),
        "the first subrect starts at the rect origin"
    );
    assert_eq!(
        harness.pixel(2, 1),
        rgba_pixel(200, 0, 0),
        "the first subrect is two pixels wide"
    );
    assert_eq!(
        harness.pixel(2, 2),
        rgba_pixel(10, 20, 30),
        "the first subrect is one pixel tall"
    );
    assert_eq!(
        harness.pixel(5, 4),
        rgba_pixel(0, 200, 0),
        "the second subrect is placed relative to the rect"
    );
    assert_eq!(
        harness.pixel(6, 5),
        rgba_pixel(0, 200, 0),
        "the second subrect ends at its bottom right pixel"
    );
    assert_eq!(
        harness.pixel(7, 1),
        rgba_pixel(0, 0, 0),
        "the pixel right of the rect is untouched"
    );
    assert_eq!(
        harness.pixel(1, 6),
        rgba_pixel(0, 0, 0),
        "the pixel below the rect is untouched"
    );
}
