use rfb::Rect;

use crate::RectOutcome;
use crate::test_support::{DecodeHarness, rgba_pixel};

#[test]
fn test_an_unknown_update_type_changes_nothing() {
    let mut harness = DecodeHarness::new(16, 8);

    let outcome = harness.decode_mvs(Rect::new(0, 0, 16, 8), &[0x07, 1, 2, 3]);

    assert!(
        matches!(outcome, RectOutcome::Pixels(changed) if changed.is_empty()),
        "an update of an unknown type reports no change, got {outcome:?}"
    );
    assert_eq!(
        harness.pixel(0, 0),
        rgba_pixel(0, 0, 0),
        "the screen stays as it was"
    );
}
