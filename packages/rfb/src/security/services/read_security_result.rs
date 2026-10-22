use std::io::Read;

use crate::{ProtocolError, ProtocolVersion, read_text, read_u32};

pub fn read_security_result(
    reader: &mut impl Read,
    version: ProtocolVersion,
) -> Result<(), ProtocolError> {
    let status = read_u32(reader)?;
    if status == 0 {
        return Ok(());
    }
    let reason = match version {
        ProtocolVersion::V3_8 => read_text(reader).unwrap_or_default(),
        ProtocolVersion::V3_3 | ProtocolVersion::V3_7 | ProtocolVersion::AppleRemoteDesktop => {
            String::new()
        }
    };
    Err(ProtocolError::AuthenticationFailed(reason))
}
