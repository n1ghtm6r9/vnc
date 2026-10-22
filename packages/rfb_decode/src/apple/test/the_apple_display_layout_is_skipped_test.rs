use rfb::{ENCODING_APPLE_DISPLAY_INFO, Rect};

use crate::RectOutcome;
use crate::test_support::DecodeHarness;

#[test]
fn test_the_apple_display_layout_is_skipped() {
    let mut payload = vec![0x0b, 0xd0, 0x07, 0xac, 0, 2, 0, 0];
    payload.extend_from_slice(&[7; 56]);
    let mut harness = DecodeHarness::new(4, 4);
    let outcome = harness
        .decode(ENCODING_APPLE_DISPLAY_INFO, Rect::new(0, 0, 0, 0), &payload)
        .expect("a display layout decodes");
    assert_eq!(
        outcome,
        RectOutcome::Ignored,
        "the layout of two displays is read whole and not used"
    );
}
