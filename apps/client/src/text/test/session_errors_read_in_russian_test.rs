use std::io;

use rfb::ProtocolError;
use rfb_auth::AuthError;
use rfb_session::SessionError;

use crate::{
    CONNECTION_REFUSED_ERROR, LINK_LOST_ERROR, MAC_USER_ENDED_ERROR, NOT_ANSWERING_ERROR, PROTOCOL_ERROR,
    UNREACHABLE_ERROR, WRONG_CREDENTIALS_ERROR, session_error_text,
};

#[test]
fn test_session_errors_read_in_russian() {
    let cases = [
        (
            "a rejected password",
            SessionError::Protocol(ProtocolError::AuthenticationFailed("authentication failed".to_owned())),
            WRONG_CREDENTIALS_ERROR.to_owned(),
        ),
        ("a missing user name", SessionError::MissingCredentials, WRONG_CREDENTIALS_ERROR.to_owned()),
        (
            "a closed port",
            SessionError::Connect(io::Error::from(io::ErrorKind::ConnectionRefused)),
            CONNECTION_REFUSED_ERROR.to_owned(),
        ),
        (
            "a silent address",
            SessionError::Connect(io::Error::from(io::ErrorKind::TimedOut)),
            NOT_ANSWERING_ERROR.to_owned(),
        ),
        (
            "an unknown host name",
            SessionError::Connect(io::Error::new(io::ErrorKind::NotFound, "the host name resolves to no address")),
            UNREACHABLE_ERROR.to_owned(),
        ),
        (
            "a refusal with a reason",
            SessionError::Protocol(ProtocolError::ConnectionRefused("too many viewers".to_owned())),
            "Mac \u{43e}\u{442}\u{43a}\u{430}\u{437}\u{430}\u{43b} \u{432} \u{43f}\u{43e}\u{434}\u{43a}\u{43b}\u{44e}\u{447}\u{435}\u{43d}\u{438}\u{438}: too many viewers".to_owned(),
        ),
        (
            "a refusal without a reason",
            SessionError::Protocol(ProtocolError::ConnectionRefused(String::new())),
            "Mac \u{43e}\u{442}\u{43a}\u{430}\u{437}\u{430}\u{43b} \u{432} \u{43f}\u{43e}\u{434}\u{43a}\u{43b}\u{44e}\u{447}\u{435}\u{43d}\u{438}\u{438}.".to_owned(),
        ),
        (
            "a reset link",
            SessionError::Protocol(ProtocolError::Io(io::Error::from(io::ErrorKind::ConnectionReset))),
            LINK_LOST_ERROR.to_owned(),
        ),
        (
            "a link closed mid-message",
            SessionError::Protocol(ProtocolError::Io(io::Error::from(io::ErrorKind::UnexpectedEof))),
            LINK_LOST_ERROR.to_owned(),
        ),
        (
            "a handshake that ran out of time",
            SessionError::Protocol(ProtocolError::Io(io::Error::from(io::ErrorKind::TimedOut))),
            NOT_ANSWERING_ERROR.to_owned(),
        ),
        (
            "an unknown protocol version",
            SessionError::Protocol(ProtocolError::UnsupportedVersion("RFB 004.000".to_owned())),
            PROTOCOL_ERROR.to_owned(),
        ),
        (
            "a link that breaks during the login",
            SessionError::Auth(AuthError::Io(io::Error::from(io::ErrorKind::ConnectionReset))),
            LINK_LOST_ERROR.to_owned(),
        ),
        ("no common security type", SessionError::NoSupportedSecurity(vec![2]), PROTOCOL_ERROR.to_owned()),
        ("a session the user of the Mac ended", SessionError::ClosedOnTheMac, MAC_USER_ENDED_ERROR.to_owned()),
    ];

    for (case, error, expected) in cases {
        assert_eq!(session_error_text(&error), expected, "{case} reads as {expected:?}");
    }
}
