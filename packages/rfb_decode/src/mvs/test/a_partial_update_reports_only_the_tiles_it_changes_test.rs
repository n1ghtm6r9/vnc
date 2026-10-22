use rfb::Rect;

use crate::RectOutcome;
use crate::test_support::{DecodeHarness, mvs_gray_full_update, mvs_red_step_partial_update};

#[test]
fn test_a_partial_update_reports_only_the_tiles_it_changes() {
    let mut harness = DecodeHarness::new(16, 8);
    let rect = Rect::new(0, 0, 16, 8);
    harness.decode_mvs(rect, &mvs_gray_full_update());

    let outcome = harness.decode_mvs(rect, &mvs_red_step_partial_update());

    assert_eq!(
        outcome,
        RectOutcome::Pixels(Rect::new(0, 0, 8, 8)),
        "only the recoded first tile needs to reach the screen"
    );
}
