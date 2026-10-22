use rfb::ProtocolError;

use super::SessionError;

impl SessionError {
    pub fn is_authentication_failure(&self) -> bool {
        matches!(
            self,
            SessionError::Protocol(ProtocolError::AuthenticationFailed(_))
                | SessionError::MissingCredentials
        )
    }
}
