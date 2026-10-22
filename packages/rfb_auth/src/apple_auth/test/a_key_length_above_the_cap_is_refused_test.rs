use std::io::Cursor;

use crate::{AuthError, Credentials, authenticate_apple};

#[test]
fn test_a_key_length_above_the_cap_is_refused() {
    let mut stream = Cursor::new(vec![0, 2, 0x04, 0x01]);
    let error = authenticate_apple(&mut stream, &Credentials::new("bogdan", "secret")).unwrap_err();
    assert!(
        matches!(error, AuthError::InvalidServerKey(_)),
        "1025 bytes is above the cap of 1024 and is refused before any key is read, got {error}"
    );
}
