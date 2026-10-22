use std::io;

use rfb::{ENCODING_RAW, Rect};

use crate::DecodeError;
use crate::test_support::DecodeHarness;

#[test]
fn test_a_truncated_raw_rect_reports_the_io_error() {
    let mut harness = DecodeHarness::new(4, 4);
    let result = harness.decode(ENCODING_RAW, Rect::new(0, 0, 4, 4), &[0u8; 20]);
    match result {
        Err(DecodeError::Io(error)) => assert_eq!(
            error.kind(),
            io::ErrorKind::UnexpectedEof,
            "a short payload ends the stream early"
        ),
        other => panic!("a truncated raw rect must report an io error, got {other:?}"),
    }
}
