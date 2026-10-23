use std::io;

use crate::write_apple_set_input_source;

#[test]
fn test_an_overlong_input_source_name_is_refused() {
    let source = "a".repeat(usize::from(u16::MAX));
    let mut bytes = Vec::new();
    let error = write_apple_set_input_source(&mut bytes, &source).unwrap_err();
    assert_eq!(error.kind(), io::ErrorKind::InvalidInput, "a name whose message size passes 65535 bytes is refused");
    assert!(bytes.is_empty(), "nothing goes out for a refused name");
}
