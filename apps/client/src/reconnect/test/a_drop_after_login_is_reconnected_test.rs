use std::io;

use rfb::ProtocolError;
use rfb_session::SessionError;

use crate::should_reconnect;

#[test]
fn test_a_drop_after_login_is_reconnected() {
    let dropped = SessionError::Protocol(ProtocolError::Io(io::Error::from(io::ErrorKind::ConnectionReset)));
    let still_offline = SessionError::Connect(io::Error::from(io::ErrorKind::TimedOut));

    assert!(
        should_reconnect(Some(&dropped), true),
        "a link that breaks after the login is opened again"
    );
    assert!(
        should_reconnect(Some(&still_offline), true),
        "a Mac that is still unreachable during reconnecting is tried again"
    );
}
