use rfb::{ENCODING_APPLE_KEYBOARD_INPUT, Rect};

use crate::RectOutcome;
use crate::test_support::DecodeHarness;

#[test]
fn test_the_keyboard_info_reports_the_mac_input_source() {
    let mut payload = vec![0, 35, 0, 1, 0, 0, 0, 0, 0, 27];
    payload.extend_from_slice(b"com.apple.keylayout.Russian");
    let mut harness = DecodeHarness::new(4, 4);
    let outcome = harness
        .decode(
            ENCODING_APPLE_KEYBOARD_INPUT,
            Rect::new(0, 0, 0, 0),
            &payload,
        )
        .expect("a keyboard info block decodes");
    assert_eq!(
        outcome,
        RectOutcome::AppleInputSource("com.apple.keylayout.Russian".to_owned()),
        "the keyboard info reports the input source the Mac uses now"
    );
}
