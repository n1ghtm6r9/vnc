use std::io::{self, Cursor};

use crate::{AuthError, Credentials, authenticate_apple};

#[test]
fn test_a_truncated_hello_is_an_io_error() {
    let mut stream = Cursor::new(vec![0, 2, 0, 16, 0x7f, 0xff, 0xff]);
    let error = authenticate_apple(&mut stream, &Credentials::new("bogdan", "secret")).unwrap_err();
    assert!(
        matches!(&error, AuthError::Io(inner) if inner.kind() == io::ErrorKind::UnexpectedEof),
        "a prime that ends after 3 of 16 bytes is an unexpected end of stream, got {error}"
    );
}
