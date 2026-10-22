use rfb::{ENCODING_APPLE_DISPLAY_INFO_2, Rect};

use crate::RectOutcome;
use crate::test_support::{DecodeHarness, apple_display_info_payload};

#[test]
fn test_an_apple_display_info_resizes_to_the_rect_size() {
    let mut harness = DecodeHarness::new(4, 4);
    let outcome = harness
        .decode(
            ENCODING_APPLE_DISPLAY_INFO_2,
            Rect::new(0, 0, 6, 5),
            &apple_display_info_payload(8, 8),
        )
        .expect("a display info decodes");
    assert_eq!(
        outcome,
        RectOutcome::AppleDisplay {
            width: 6,
            height: 5,
            geometry: None
        },
        "the size of the rect wins over the size inside"
    );
    assert_eq!(
        (harness.framebuffer.width(), harness.framebuffer.height()),
        (6, 5),
        "the framebuffer takes the new size"
    );
}
