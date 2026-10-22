use rfb_session::SessionError;

pub fn should_reconnect(error: Option<&SessionError>, logged_in: bool) -> bool {
    logged_in && error.is_some_and(|error| !error.is_authentication_failure() && !matches!(error, SessionError::ClosedOnTheMac))
}
