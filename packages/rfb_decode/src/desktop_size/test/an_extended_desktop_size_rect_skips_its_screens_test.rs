use rfb::{ENCODING_EXTENDED_DESKTOP_SIZE, Rect};

use crate::RectOutcome;
use crate::test_support::DecodeHarness;

#[test]
fn test_an_extended_desktop_size_rect_skips_its_screens() {
    let mut harness = DecodeHarness::new(4, 4);
    let mut payload = vec![2u8, 0, 0, 0];
    payload.extend_from_slice(&[7u8; 32]);
    let outcome = harness
        .decode(
            ENCODING_EXTENDED_DESKTOP_SIZE,
            Rect::new(0, 0, 8, 6),
            &payload,
        )
        .expect("an extended desktop size rect decodes");
    assert_eq!(
        outcome,
        RectOutcome::DesktopSize {
            width: 8,
            height: 6
        },
        "the outcome reports the new size"
    );
    assert_eq!(
        (harness.framebuffer.width(), harness.framebuffer.height()),
        (8, 6),
        "the framebuffer has the new size"
    );
}
