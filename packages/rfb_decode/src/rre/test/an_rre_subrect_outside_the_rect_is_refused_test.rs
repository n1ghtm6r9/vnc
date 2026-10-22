use rfb::{ENCODING_RRE, Rect};

use crate::DecodeError;
use crate::test_support::{DecodeHarness, rre_payload, rre_subrect, wire_pixel};

#[test]
fn test_an_rre_subrect_outside_the_rect_is_refused() {
    let mut harness = DecodeHarness::new(10, 8);
    let payload = rre_payload(
        wire_pixel(1, 2, 3),
        &[rre_subrect(wire_pixel(9, 9, 9), Rect::new(5, 0, 3, 1))],
    );
    let result = harness.decode(ENCODING_RRE, Rect::new(0, 0, 6, 4), &payload);
    assert!(
        matches!(result, Err(DecodeError::Corrupt(_))),
        "a subrect that leaves its rect is corrupt, got {result:?}"
    );
}
