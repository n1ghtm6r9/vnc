use std::io;

use crate::read_bytes;

#[test]
fn test_a_short_read_reports_the_missing_bytes() {
    let bytes = [1u8, 2, 3];
    let error = read_bytes(&mut &bytes[..], 5).unwrap_err();
    assert_eq!(error.kind(), io::ErrorKind::UnexpectedEof, "a stream that ends early is an unexpected end, not a short success");
}
