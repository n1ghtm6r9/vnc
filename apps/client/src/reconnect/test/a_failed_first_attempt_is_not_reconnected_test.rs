use std::io;

use rfb_session::SessionError;

use crate::should_reconnect;

#[test]
fn test_a_failed_first_attempt_is_not_reconnected() {
    let refused = SessionError::Connect(io::Error::from(io::ErrorKind::ConnectionRefused));

    assert!(
        !should_reconnect(Some(&refused), false),
        "a first attempt that never logged in goes back to the form"
    );
    assert!(
        !should_reconnect(None, true),
        "a session ended without an error is not opened again"
    );
}
