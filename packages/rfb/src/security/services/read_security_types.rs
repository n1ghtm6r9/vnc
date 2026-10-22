use std::io::{self, Read};

use crate::{ProtocolError, ProtocolVersion, read_bytes, read_text, read_u8, read_u32};

pub fn read_security_types(
    reader: &mut impl Read,
    version: ProtocolVersion,
) -> Result<Vec<u8>, ProtocolError> {
    match version {
        ProtocolVersion::V3_3 => {
            let chosen = read_u32(reader)?;
            if chosen == 0 {
                return Err(ProtocolError::ConnectionRefused(read_text(reader)?));
            }
            let chosen = u8::try_from(chosen).map_err(|_| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("the server chose security type {chosen}, which does not fit a byte"),
                )
            })?;
            Ok(vec![chosen])
        }
        ProtocolVersion::V3_7 | ProtocolVersion::V3_8 | ProtocolVersion::AppleRemoteDesktop => {
            let count = read_u8(reader)?;
            if count == 0 {
                return Err(ProtocolError::ConnectionRefused(read_text(reader)?));
            }
            Ok(read_bytes(reader, usize::from(count))?)
        }
    }
}
