use std::io::ErrorKind;

use rfb::ProtocolError;
use rfb_auth::AuthError;
use rfb_session::SessionError;

use crate::FIRST_CONNECT_ATTEMPTS;

pub fn should_retry_first_connect(error: Option<&SessionError>, attempt: usize) -> bool {
    let unanswered = match error {
        Some(SessionError::Protocol(ProtocolError::Io(cause)) | SessionError::Auth(AuthError::Io(cause))) => {
            cause.kind() == ErrorKind::TimedOut
        }
        _ => false,
    };
    unanswered && attempt.saturating_add(1) < FIRST_CONNECT_ATTEMPTS
}
