use rfb::{ENCODING_APPLE_DISPLAY_INFO_2, Rect};

use crate::RectOutcome;
use crate::test_support::{DecodeHarness, apple_display_info_payload, rgba_pixel};

#[test]
fn test_an_apple_display_info_without_any_size_keeps_the_desktop() {
    for payload in [vec![0, 2, 0, 1], apple_display_info_payload(0, 0)] {
        let mut harness = DecodeHarness::new(4, 4);
        harness
            .framebuffer
            .fill_rect(Rect::new(0, 0, 4, 4), rgba_pixel(9, 9, 9));
        let outcome = harness
            .decode(
                ENCODING_APPLE_DISPLAY_INFO_2,
                Rect::new(0, 0, 0, 0),
                &payload,
            )
            .expect("a display info without a size decodes");
        assert_eq!(
            outcome,
            RectOutcome::AppleDisplay {
                width: 4,
                height: 4,
                geometry: None
            },
            "with no size anywhere the desktop keeps its size, and the screen is still asked for again"
        );
        assert_eq!(harness.pixel(3, 3), rgba_pixel(9, 9, 9), "the pixels stay");
    }
}
