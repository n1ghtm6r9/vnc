use std::io::{self, Cursor};

use crate::{AuthError, authenticate_vnc};

#[test]
fn test_a_truncated_challenge_is_an_io_error() {
    let mut stream = Cursor::new(vec![0u8; 10]);
    let error = authenticate_vnc(&mut stream, "secret").unwrap_err();
    assert!(
        matches!(&error, AuthError::Io(inner) if inner.kind() == io::ErrorKind::UnexpectedEof),
        "a challenge that ends after 10 of 16 bytes is an unexpected end of stream, got {error}"
    );
    assert_eq!(
        stream.get_ref().len(),
        10,
        "nothing is written when the challenge is incomplete"
    );
}
