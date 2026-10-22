use std::io::{Read, Write};

use rfb::{SECURITY_APPLE_REMOTE_DESKTOP, SECURITY_VNC_AUTH};
use rfb_auth::{AppleSessionKey, Credentials, authenticate_apple, authenticate_vnc};

use crate::SessionError;

pub(crate) fn authenticate(
    stream: &mut (impl Read + Write),
    security: u8,
    credentials: &Credentials,
) -> Result<Option<AppleSessionKey>, SessionError> {
    match security {
        SECURITY_APPLE_REMOTE_DESKTOP => Ok(Some(authenticate_apple(stream, credentials)?)),
        SECURITY_VNC_AUTH => {
            authenticate_vnc(stream, &credentials.password)?;
            Ok(None)
        }
        _ => Ok(None),
    }
}
