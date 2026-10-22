use rfb::{ENCODING_APPLE_DEVICE_INFO, ENCODING_APPLE_KEYBOARD_INPUT, Rect};

use crate::RectOutcome;
use crate::test_support::DecodeHarness;

#[test]
fn test_apple_device_and_keyboard_info_are_skipped() {
    for encoding in [ENCODING_APPLE_DEVICE_INFO, ENCODING_APPLE_KEYBOARD_INPUT] {
        let mut harness = DecodeHarness::new(4, 4);
        let outcome = harness
            .decode(encoding, Rect::new(0, 0, 0, 0), &[0, 3, 1, 2, 3])
            .expect("an info block decodes");
        assert_eq!(
            outcome,
            RectOutcome::Ignored,
            "pseudo-encoding {encoding} is read whole and not used"
        );
    }
}
