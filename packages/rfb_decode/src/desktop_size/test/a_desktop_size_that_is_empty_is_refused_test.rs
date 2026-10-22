use rfb::{ENCODING_DESKTOP_SIZE, ENCODING_EXTENDED_DESKTOP_SIZE, Rect};

use crate::DecodeError;
use crate::test_support::DecodeHarness;

#[test]
fn test_a_desktop_size_that_is_empty_is_refused() {
    for (width, height) in [(0u16, 5u16), (5, 0), (0, 0)] {
        let mut harness = DecodeHarness::new(4, 4);
        let plain = harness.decode(ENCODING_DESKTOP_SIZE, Rect::new(0, 0, width, height), &[]);
        assert!(
            matches!(plain, Err(DecodeError::Corrupt(_))),
            "a {width}x{height} desktop is refused, got {plain:?}"
        );
        let extended = harness.decode(
            ENCODING_EXTENDED_DESKTOP_SIZE,
            Rect::new(0, 0, width, height),
            &[0, 0, 0, 0],
        );
        assert!(
            matches!(extended, Err(DecodeError::Corrupt(_))),
            "a {width}x{height} extended desktop is refused, got {extended:?}"
        );
        assert_eq!(
            (harness.framebuffer.width(), harness.framebuffer.height()),
            (4, 4),
            "the framebuffer keeps its size after the refusal"
        );
    }
}
