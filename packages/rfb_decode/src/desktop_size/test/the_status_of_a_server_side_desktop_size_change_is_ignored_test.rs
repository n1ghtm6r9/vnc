use rfb::{ENCODING_EXTENDED_DESKTOP_SIZE, Rect};

use crate::RectOutcome;
use crate::test_support::DecodeHarness;

#[test]
fn test_the_status_of_a_server_side_desktop_size_change_is_ignored() {
    for (reason, status) in [(0u16, 3u16), (2, 1), (7, 2)] {
        let mut harness = DecodeHarness::new(4, 4);
        let outcome = harness
            .decode(
                ENCODING_EXTENDED_DESKTOP_SIZE,
                Rect::new(reason, status, 8, 8),
                &[0, 0, 0, 0],
            )
            .expect("a server side change decodes");
        assert_eq!(
            outcome,
            RectOutcome::DesktopSize {
                width: 8,
                height: 8
            },
            "reason {reason} with status {status} still resizes"
        );
        assert_eq!(
            (harness.framebuffer.width(), harness.framebuffer.height()),
            (8, 8),
            "the framebuffer follows reason {reason}"
        );
    }
}
