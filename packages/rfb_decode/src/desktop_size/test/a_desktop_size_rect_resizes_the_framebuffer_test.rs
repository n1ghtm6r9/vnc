use rfb::{ENCODING_DESKTOP_SIZE, Rect};

use crate::RectOutcome;
use crate::test_support::{DecodeHarness, rgba_pixel};

#[test]
fn test_a_desktop_size_rect_resizes_the_framebuffer() {
    let mut harness = DecodeHarness::new(4, 4);
    harness
        .framebuffer
        .fill_rect(Rect::new(0, 0, 4, 4), rgba_pixel(9, 9, 9));
    let outcome = harness
        .decode(ENCODING_DESKTOP_SIZE, Rect::new(0, 0, 6, 5), &[])
        .expect("a desktop size rect decodes");
    assert_eq!(
        outcome,
        RectOutcome::DesktopSize {
            width: 6,
            height: 5
        },
        "the outcome reports the new size"
    );
    assert_eq!(
        (harness.framebuffer.width(), harness.framebuffer.height()),
        (6, 5),
        "the framebuffer has the new size"
    );
    assert_eq!(
        harness.pixel(3, 3),
        rgba_pixel(9, 9, 9),
        "pixels inside the old size survive"
    );
    assert_eq!(
        harness.pixel(5, 4),
        rgba_pixel(0, 0, 0),
        "the newly exposed area is black"
    );
}
