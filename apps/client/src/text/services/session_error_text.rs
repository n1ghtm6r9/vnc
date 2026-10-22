use std::io::ErrorKind;

use rfb::ProtocolError;
use rfb_auth::AuthError;
use rfb_session::SessionError;

use crate::{
    CONNECTION_REFUSED_ERROR, LINK_LOST_ERROR, NOT_ANSWERING_ERROR, PROTOCOL_ERROR, SERVER_REFUSED_ERROR,
    UNREACHABLE_ERROR, WRONG_CREDENTIALS_ERROR,
};

pub fn session_error_text(error: &SessionError) -> String {
    let text = match error {
        SessionError::Protocol(ProtocolError::AuthenticationFailed(_)) | SessionError::MissingCredentials => {
            WRONG_CREDENTIALS_ERROR
        }
        SessionError::Connect(error) => match error.kind() {
            ErrorKind::ConnectionRefused => CONNECTION_REFUSED_ERROR,
            ErrorKind::TimedOut => NOT_ANSWERING_ERROR,
            _ => UNREACHABLE_ERROR,
        },
        SessionError::Protocol(ProtocolError::ConnectionRefused(reason)) => {
            let reason = reason.trim();
            if reason.is_empty() {
                return format!("{SERVER_REFUSED_ERROR}.");
            }
            return format!("{SERVER_REFUSED_ERROR}: {reason}");
        }
        SessionError::Protocol(ProtocolError::Io(error)) | SessionError::Auth(AuthError::Io(error)) => {
            if error.kind() == ErrorKind::TimedOut {
                NOT_ANSWERING_ERROR
            } else {
                LINK_LOST_ERROR
            }
        }
        _ => PROTOCOL_ERROR,
    };
    text.to_owned()
}
