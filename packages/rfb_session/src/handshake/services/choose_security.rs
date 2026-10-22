use rfb::{SECURITY_APPLE_REMOTE_DESKTOP, SECURITY_NONE, SECURITY_VNC_AUTH};
use rfb_auth::Credentials;

use crate::SessionError;

pub(crate) fn choose_security(
    offered: &[u8],
    credentials: &Credentials,
) -> Result<u8, SessionError> {
    let offers = |security: u8| offered.contains(&security);
    if offers(SECURITY_APPLE_REMOTE_DESKTOP) && !credentials.username.is_empty() {
        return Ok(SECURITY_APPLE_REMOTE_DESKTOP);
    }
    if offers(SECURITY_VNC_AUTH) && !credentials.password.is_empty() {
        return Ok(SECURITY_VNC_AUTH);
    }
    if offers(SECURITY_NONE) {
        return Ok(SECURITY_NONE);
    }
    if offers(SECURITY_APPLE_REMOTE_DESKTOP) {
        return Err(SessionError::MissingCredentials);
    }
    Err(SessionError::NoSupportedSecurity(offered.to_vec()))
}
