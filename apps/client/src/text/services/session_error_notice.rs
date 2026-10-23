use std::io::ErrorKind;

use rfb::ProtocolError;
use rfb_auth::AuthError;
use rfb_session::SessionError;

use crate::{
    CONNECTION_REFUSED_ERROR, LINK_LOST_ERROR, MAC_USER_ENDED_ERROR, NOT_ANSWERING_ERROR, Notice, PROTOCOL_ERROR,
    SERVER_REFUSED_ERROR, SERVER_REFUSED_WITH_REASON_ERROR, UNREACHABLE_ERROR, WRONG_CREDENTIALS_ERROR,
};

pub fn session_error_notice(error: &SessionError) -> Notice {
    let phrase = match error {
        SessionError::Protocol(ProtocolError::AuthenticationFailed(_)) | SessionError::MissingCredentials => {
            WRONG_CREDENTIALS_ERROR
        }
        SessionError::Connect(error) => match error.kind() {
            ErrorKind::ConnectionRefused => CONNECTION_REFUSED_ERROR,
            ErrorKind::TimedOut => NOT_ANSWERING_ERROR,
            _ => UNREACHABLE_ERROR,
        },
        SessionError::Protocol(ProtocolError::ConnectionRefused(reason)) => match reason.trim() {
            "" => SERVER_REFUSED_ERROR,
            reason => return Notice { phrase: SERVER_REFUSED_WITH_REASON_ERROR, detail: Some(reason.to_owned()) },
        },
        SessionError::Protocol(ProtocolError::Io(error)) | SessionError::Auth(AuthError::Io(error)) => {
            if error.kind() == ErrorKind::TimedOut {
                NOT_ANSWERING_ERROR
            } else {
                LINK_LOST_ERROR
            }
        }
        SessionError::ClosedOnTheMac => MAC_USER_ENDED_ERROR,
        _ => PROTOCOL_ERROR,
    };
    Notice::from(phrase)
}
