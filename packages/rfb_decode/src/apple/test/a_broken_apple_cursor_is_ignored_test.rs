use rfb::{ENCODING_APPLE_CURSOR_ALPHA, Rect};

use crate::RectOutcome;
use crate::test_support::{DecodeHarness, apple_cursor_payload, finished_zlib_stream};

#[test]
fn test_a_broken_apple_cursor_is_ignored() {
    let cases = [
        (
            "data that is not zlib",
            Rect::new(0, 0, 1, 1),
            vec![1, 2, 3, 4, 5],
        ),
        (
            "data too short for its size",
            Rect::new(0, 0, 1, 1),
            finished_zlib_stream(&[1, 2, 3, 0]),
        ),
        (
            "a size beyond the cursor cap",
            Rect::new(0, 0, 600, 1),
            finished_zlib_stream(&[0; 3000]),
        ),
    ];
    for (problem, rect, data) in cases {
        let mut harness = DecodeHarness::new(4, 4);
        let outcome = harness
            .decode(
                ENCODING_APPLE_CURSOR_ALPHA,
                rect,
                &apple_cursor_payload(1000, &data),
            )
            .expect("a broken cursor does not end the session");
        assert_eq!(
            outcome,
            RectOutcome::Ignored,
            "a cursor with {problem} keeps the current cursor"
        );
    }
}
