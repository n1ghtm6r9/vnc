use std::io::{Read, Write};

use log::debug;
use rfb::{APPLE_SERVER_FLAG_SESSION_SELECT, AppleServerInit, write_apple_client_init};

use crate::SessionError;

pub(crate) fn exchange_apple_init(
    stream: &mut (impl Read + Write),
) -> Result<AppleServerInit, SessionError> {
    write_apple_client_init(stream)?;
    let apple_init = AppleServerInit::read(stream)?;
    debug!(
        "[SESSION] the Mac sends ServerInit flags {:#x}",
        apple_init.flags
    );
    if apple_init.flags & APPLE_SERVER_FLAG_SESSION_SELECT != 0 {
        return Err(SessionError::AppleSessionSelect);
    }
    Ok(apple_init)
}
