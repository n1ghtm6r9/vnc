use rfb::{ENCODING_APPLE_DISPLAY_INFO_2, Rect};

use crate::RectOutcome;
use crate::test_support::{DecodeHarness, apple_display_info_payload};

#[test]
fn test_an_apple_display_info_without_a_rect_size_uses_its_pixel_size() {
    let mut harness = DecodeHarness::new(4, 4);
    let outcome = harness
        .decode(
            ENCODING_APPLE_DISPLAY_INFO_2,
            Rect::new(0, 0, 0, 0),
            &apple_display_info_payload(6, 5),
        )
        .expect("a display info decodes");
    assert_eq!(
        outcome,
        RectOutcome::AppleDisplay {
            width: 6,
            height: 5,
            geometry: None
        },
        "an empty rect leaves the size to the display info"
    );
}
