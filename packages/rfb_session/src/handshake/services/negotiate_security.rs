use std::io::{Read, Write};

use log::debug;
use rfb::{ProtocolVersion, read_security_result, read_security_types, write_security_choice};
use rfb_auth::{AppleSessionKey, Credentials};

use super::{authenticate, choose_security, expects_security_result};
use crate::SessionError;

pub(crate) fn negotiate_security(
    stream: &mut (impl Read + Write),
    version: ProtocolVersion,
    credentials: &Credentials,
) -> Result<Option<AppleSessionKey>, SessionError> {
    let offered = read_security_types(stream, version)?;
    let security = choose_security(&offered, credentials)?;
    debug!("[SESSION] the server offers security types {offered:?}, the client takes {security}");
    if version != ProtocolVersion::V3_3 {
        write_security_choice(stream, security)?;
    }
    let apple_key = authenticate(stream, security, credentials)?;
    if expects_security_result(version, security) {
        read_security_result(stream, version)?;
    }
    Ok(apple_key)
}
