use std::io::Cursor;

use crate::{AuthError, Credentials, authenticate_apple};

#[test]
fn test_a_zero_key_length_is_refused() {
    let mut stream = Cursor::new(vec![0, 2, 0, 0]);
    let error = authenticate_apple(&mut stream, &Credentials::new("bogdan", "secret")).unwrap_err();
    assert!(
        matches!(error, AuthError::InvalidServerKey(_)),
        "a key length of zero is an invalid server key, got {error}"
    );
    assert_eq!(
        stream.get_ref().len(),
        4,
        "nothing is written back to a server that sent an unusable key"
    );
}
