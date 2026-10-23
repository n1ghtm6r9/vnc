use std::io::{self, ErrorKind};

use rfb::ProtocolError;
use rfb_session::SessionError;

use crate::should_retry_first_connect;

#[test]
fn test_other_failed_first_attempts_are_not_made_again() {
    let refused = SessionError::Connect(io::Error::from(ErrorKind::ConnectionRefused));
    let unreachable = SessionError::Connect(io::Error::from(ErrorKind::TimedOut));
    let wrong_password = SessionError::Protocol(ProtocolError::AuthenticationFailed("authentication failed".to_owned()));
    let broken = SessionError::Protocol(ProtocolError::Io(io::Error::from(ErrorKind::ConnectionReset)));

    assert!(
        !should_retry_first_connect(Some(&refused), 0),
        "a Mac that refuses the connection is reported at once"
    );
    assert!(
        !should_retry_first_connect(Some(&unreachable), 0),
        "a Mac whose port never opens is reported after one attempt, so a switched-off Mac does not double the wait"
    );
    assert!(
        !should_retry_first_connect(Some(&wrong_password), 0),
        "a password the Mac rejects is not sent again"
    );
    assert!(
        !should_retry_first_connect(Some(&broken), 0),
        "a connection the Mac breaks is reported at once"
    );
    assert!(
        !should_retry_first_connect(None, 0),
        "a session that ended without an error is not started again"
    );
}
