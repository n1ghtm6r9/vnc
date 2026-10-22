use rfb::{ENCODING_APPLE_CURSOR_ALPHA, Rect};

use crate::test_support::{DecodeHarness, apple_cursor_payload, finished_zlib_stream};

#[test]
fn test_a_known_apple_cursor_is_reused_without_data() {
    let data = finished_zlib_stream(&[1, 2, 3, 0, 200]);
    let mut harness = DecodeHarness::new(4, 4);
    let first = harness
        .decode(
            ENCODING_APPLE_CURSOR_ALPHA,
            Rect::new(0, 0, 1, 1),
            &apple_cursor_payload(1001, &data),
        )
        .expect("the cursor decodes the first time");
    let again = harness
        .decode(
            ENCODING_APPLE_CURSOR_ALPHA,
            Rect::new(0, 0, 1, 1),
            &apple_cursor_payload(1001, &[]),
        )
        .expect("the cursor decodes again without data");
    assert_eq!(
        again, first,
        "a cursor sent again without data is the one sent before under its id"
    );
}
