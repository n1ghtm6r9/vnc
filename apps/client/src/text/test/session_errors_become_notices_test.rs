use std::io;

use rfb::ProtocolError;
use rfb_auth::AuthError;
use rfb_session::SessionError;

use crate::{
    CONNECTION_REFUSED_ERROR, LINK_LOST_ERROR, MAC_USER_ENDED_ERROR, NOT_ANSWERING_ERROR, Notice, PROTOCOL_ERROR,
    SERVER_REFUSED_ERROR, SERVER_REFUSED_WITH_REASON_ERROR, UNREACHABLE_ERROR, WRONG_CREDENTIALS_ERROR,
    session_error_notice,
};

#[test]
fn test_session_errors_become_notices() {
    let cases = [
        (
            "a rejected password",
            SessionError::Protocol(ProtocolError::AuthenticationFailed("authentication failed".to_owned())),
            Notice::from(WRONG_CREDENTIALS_ERROR),
        ),
        ("a missing user name", SessionError::MissingCredentials, Notice::from(WRONG_CREDENTIALS_ERROR)),
        (
            "a closed port",
            SessionError::Connect(io::Error::from(io::ErrorKind::ConnectionRefused)),
            Notice::from(CONNECTION_REFUSED_ERROR),
        ),
        (
            "a silent address",
            SessionError::Connect(io::Error::from(io::ErrorKind::TimedOut)),
            Notice::from(NOT_ANSWERING_ERROR),
        ),
        (
            "an unknown host name",
            SessionError::Connect(io::Error::new(io::ErrorKind::NotFound, "the host name resolves to no address")),
            Notice::from(UNREACHABLE_ERROR),
        ),
        (
            "a refusal with a reason",
            SessionError::Protocol(ProtocolError::ConnectionRefused(" too many viewers ".to_owned())),
            Notice { phrase: SERVER_REFUSED_WITH_REASON_ERROR, detail: Some("too many viewers".to_owned()) },
        ),
        (
            "a refusal without a reason",
            SessionError::Protocol(ProtocolError::ConnectionRefused(String::new())),
            Notice::from(SERVER_REFUSED_ERROR),
        ),
        (
            "a reset link",
            SessionError::Protocol(ProtocolError::Io(io::Error::from(io::ErrorKind::ConnectionReset))),
            Notice::from(LINK_LOST_ERROR),
        ),
        (
            "a link closed mid-message",
            SessionError::Protocol(ProtocolError::Io(io::Error::from(io::ErrorKind::UnexpectedEof))),
            Notice::from(LINK_LOST_ERROR),
        ),
        (
            "a handshake that ran out of time",
            SessionError::Protocol(ProtocolError::Io(io::Error::from(io::ErrorKind::TimedOut))),
            Notice::from(NOT_ANSWERING_ERROR),
        ),
        (
            "an unknown protocol version",
            SessionError::Protocol(ProtocolError::UnsupportedVersion("RFB 004.000".to_owned())),
            Notice::from(PROTOCOL_ERROR),
        ),
        (
            "a link that breaks during the login",
            SessionError::Auth(AuthError::Io(io::Error::from(io::ErrorKind::ConnectionReset))),
            Notice::from(LINK_LOST_ERROR),
        ),
        ("no common security type", SessionError::NoSupportedSecurity(vec![2]), Notice::from(PROTOCOL_ERROR)),
        ("a session the user of the Mac ended", SessionError::ClosedOnTheMac, Notice::from(MAC_USER_ENDED_ERROR)),
    ];

    for (case, error, expected) in cases {
        assert_eq!(session_error_notice(&error), expected, "{case} reads as {expected:?}");
    }
}
