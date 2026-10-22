use std::io;

use crate::skip_bytes;

#[test]
fn test_skipping_past_the_end_reports_eof() {
    let bytes = [0u8; 2];
    let error = skip_bytes(&mut &bytes[..], 3).unwrap_err();
    assert_eq!(error.kind(), io::ErrorKind::UnexpectedEof, "padding that never arrives means the stream broke");
}
