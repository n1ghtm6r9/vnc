use rfb::{ENCODING_EXTENDED_DESKTOP_SIZE, Rect};

use crate::RectOutcome;
use crate::test_support::{DecodeHarness, rgba_pixel};

#[test]
fn test_an_extended_desktop_size_of_the_same_size_keeps_the_pixels() {
    let mut harness = DecodeHarness::new(4, 4);
    harness
        .framebuffer
        .fill_rect(Rect::new(0, 0, 4, 4), rgba_pixel(9, 9, 9));
    let mut payload = vec![1u8, 0, 0, 0];
    payload.extend_from_slice(&[3u8; 16]);
    let outcome = harness
        .decode(
            ENCODING_EXTENDED_DESKTOP_SIZE,
            Rect::new(0, 0, 4, 4),
            &payload,
        )
        .expect("a layout change decodes");
    assert_eq!(
        outcome,
        RectOutcome::DesktopSize {
            width: 4,
            height: 4
        },
        "the outcome reports the size"
    );
    assert_eq!(
        harness.pixel(3, 3),
        rgba_pixel(9, 9, 9),
        "a layout change alone keeps the pixels"
    );
}
