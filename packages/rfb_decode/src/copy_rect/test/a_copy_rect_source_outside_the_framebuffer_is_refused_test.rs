use rfb::{ENCODING_COPY_RECT, Rect};

use crate::DecodeError;
use crate::test_support::DecodeHarness;

#[test]
fn test_a_copy_rect_source_outside_the_framebuffer_is_refused() {
    let mut harness = DecodeHarness::new(6, 6);
    let payload = [5u16.to_be_bytes(), 5u16.to_be_bytes()].concat();
    let (outcome, consumed) =
        harness.decode_counting(ENCODING_COPY_RECT, Rect::new(0, 0, 4, 4), &payload);
    match outcome {
        Err(DecodeError::RectOutOfBounds(source)) => assert_eq!(
            source,
            Rect::new(5, 5, 4, 4),
            "the error names the source rect"
        ),
        other => panic!("a source outside the frame must be refused, got {other:?}"),
    }
    assert_eq!(consumed, 4, "only the source position was read");
}
