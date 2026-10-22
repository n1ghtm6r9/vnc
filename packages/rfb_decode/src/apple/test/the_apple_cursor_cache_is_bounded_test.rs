use rfb::{ENCODING_APPLE_CURSOR_ALPHA, Rect};

use crate::RectOutcome;
use crate::test_support::{DecodeHarness, apple_cursor_payload, finished_zlib_stream};

#[test]
fn test_the_apple_cursor_cache_is_bounded() {
    let data = finished_zlib_stream(&[1, 2, 3, 0, 255]);
    let rect = Rect::new(0, 0, 1, 1);
    let mut harness = DecodeHarness::new(4, 4);
    for id in 1000..=1064 {
        harness
            .decode(
                ENCODING_APPLE_CURSOR_ALPHA,
                rect,
                &apple_cursor_payload(id, &data),
            )
            .expect("each cursor decodes");
    }
    let oldest = harness
        .decode(
            ENCODING_APPLE_CURSOR_ALPHA,
            rect,
            &apple_cursor_payload(1000, &[]),
        )
        .expect("the oldest cursor decodes without data");
    let newest = harness
        .decode(
            ENCODING_APPLE_CURSOR_ALPHA,
            rect,
            &apple_cursor_payload(1064, &[]),
        )
        .expect("the newest cursor decodes without data");
    assert_eq!(
        oldest,
        RectOutcome::Ignored,
        "after 64 cursors the cache starts over, so the oldest is gone"
    );
    assert!(
        matches!(newest, RectOutcome::Cursor(_)),
        "the newest cursor is still known, got {newest:?}"
    );
}
