use rfb::{ENCODING_EXTENDED_DESKTOP_SIZE, Rect};

use crate::RectOutcome;
use crate::test_support::{DecodeHarness, rgba_pixel};

#[test]
fn test_a_refused_resize_request_keeps_the_framebuffer() {
    let mut harness = DecodeHarness::new(4, 4);
    harness
        .framebuffer
        .fill_rect(Rect::new(0, 0, 4, 4), rgba_pixel(9, 9, 9));
    let mut payload = vec![1u8, 0, 0, 0];
    payload.extend_from_slice(&[3u8; 16]);
    let outcome = harness
        .decode(
            ENCODING_EXTENDED_DESKTOP_SIZE,
            Rect::new(1, 1, 100, 100),
            &payload,
        )
        .expect("a refusal is a normal rect");
    assert_eq!(
        outcome,
        RectOutcome::DesktopSize {
            width: 4,
            height: 4
        },
        "the outcome reports the unchanged size"
    );
    assert_eq!(
        (harness.framebuffer.width(), harness.framebuffer.height()),
        (4, 4),
        "the framebuffer keeps its size"
    );
    assert_eq!(
        harness.pixel(2, 2),
        rgba_pixel(9, 9, 9),
        "the framebuffer keeps its pixels"
    );
}
