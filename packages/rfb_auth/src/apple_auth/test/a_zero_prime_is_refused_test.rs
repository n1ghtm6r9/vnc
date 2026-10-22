use std::io::Cursor;

use crate::{AuthError, Credentials, authenticate_apple};

#[test]
fn test_a_zero_prime_is_refused() {
    let mut hello = vec![0, 2, 0, 4];
    hello.extend_from_slice(&[0, 0, 0, 0]);
    hello.extend_from_slice(&[0, 0, 0, 5]);
    let mut stream = Cursor::new(hello);
    let error = authenticate_apple(&mut stream, &Credentials::new("bogdan", "secret")).unwrap_err();
    assert!(
        matches!(error, AuthError::InvalidServerKey(_)),
        "a prime of zero is an invalid server key, got {error}"
    );
    assert_eq!(
        stream.get_ref().len(),
        12,
        "nothing is written back to a server that sent an unusable prime"
    );
}
