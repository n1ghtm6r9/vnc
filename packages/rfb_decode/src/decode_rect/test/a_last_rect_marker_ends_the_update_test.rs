use rfb::{ENCODING_LAST_RECT, Rect};

use crate::RectOutcome;
use crate::test_support::DecodeHarness;

#[test]
fn test_a_last_rect_marker_ends_the_update() {
    let mut harness = DecodeHarness::new(4, 4);
    let outcome = harness
        .decode(ENCODING_LAST_RECT, Rect::default(), &[])
        .expect("the marker carries no payload");
    assert_eq!(
        outcome,
        RectOutcome::LastRect,
        "the marker is reported as the end of the update"
    );
}
