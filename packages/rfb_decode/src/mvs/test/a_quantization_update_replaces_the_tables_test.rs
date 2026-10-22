use rfb::Rect;

use crate::RectOutcome;
use crate::test_support::{DecodeHarness, mvs_gray_full_update, rgba_pixel};

#[test]
fn test_a_quantization_update_replaces_the_tables() {
    let mut harness = DecodeHarness::new(16, 8);
    let mut tables = vec![8u8; 129];
    tables[0] = 0x02;

    let outcome = harness.decode_mvs(Rect::new(0, 0, 0, 0), &tables);
    harness.decode_mvs(Rect::new(0, 0, 16, 8), &mvs_gray_full_update());

    assert!(
        matches!(outcome, RectOutcome::Pixels(changed) if changed.is_empty()),
        "new tables change no pixel by themselves, got {outcome:?}"
    );
    assert_eq!(
        harness.pixel(0, 0),
        rgba_pixel(138, 138, 138),
        "a luma DC of 10 times a step of 8 lands 10 above mid gray"
    );
}
