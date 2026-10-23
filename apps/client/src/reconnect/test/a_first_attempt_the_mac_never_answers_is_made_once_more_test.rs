use std::io::{self, ErrorKind};

use rfb::ProtocolError;
use rfb_auth::AuthError;
use rfb_session::SessionError;

use crate::should_retry_first_connect;

#[test]
fn test_a_first_attempt_the_mac_never_answers_is_made_once_more() {
    let silent = SessionError::Protocol(ProtocolError::Io(io::Error::from(ErrorKind::TimedOut)));
    let silent_at_login = SessionError::Auth(AuthError::Io(io::Error::from(ErrorKind::TimedOut)));

    assert!(
        should_retry_first_connect(Some(&silent), 0),
        "a Mac that accepts the connection and then stays silent gets one more attempt before the form shows an error"
    );
    assert!(
        should_retry_first_connect(Some(&silent_at_login), 0),
        "a Mac that stays silent while it checks the password gets one more attempt"
    );
    assert!(
        !should_retry_first_connect(Some(&silent), 1),
        "a Mac that stays silent on the second attempt as well is reported in the form"
    );
}
