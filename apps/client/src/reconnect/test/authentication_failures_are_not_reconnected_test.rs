use rfb::ProtocolError;
use rfb_session::SessionError;

use crate::should_reconnect;

#[test]
fn test_authentication_failures_are_not_reconnected() {
    let wrong_password = SessionError::Protocol(ProtocolError::AuthenticationFailed("authentication failed".to_owned()));

    assert!(
        !should_reconnect(Some(&wrong_password), true),
        "a password the Mac rejects is not tried again"
    );
    assert!(
        !should_reconnect(Some(&SessionError::MissingCredentials), true),
        "a Mac that asks for a missing user name is not tried again"
    );
}
