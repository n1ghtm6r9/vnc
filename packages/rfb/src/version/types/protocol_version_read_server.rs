use std::io::Read;

use super::ProtocolVersion;
use crate::{ProtocolError, parse_decimal};

impl ProtocolVersion {
    pub fn read_server(reader: &mut impl Read) -> Result<ProtocolVersion, ProtocolError> {
        let mut banner = [0u8; 12];
        reader.read_exact(&mut banner)?;
        let unsupported =
            || ProtocolError::UnsupportedVersion(String::from_utf8_lossy(&banner).trim_end().to_string());
        if &banner[..4] != b"RFB " || banner[7] != b'.' || banner[11] != b'\n' {
            return Err(unsupported());
        }
        let major = parse_decimal(&banner[4..7]).ok_or_else(unsupported)?;
        let minor = parse_decimal(&banner[8..11]).ok_or_else(unsupported)?;
        match (major, minor) {
            (3, minor) if minor < 7 => Ok(ProtocolVersion::V3_3),
            (3, 7) => Ok(ProtocolVersion::V3_7),
            (3, 889) => Ok(ProtocolVersion::AppleRemoteDesktop),
            (3, _) => Ok(ProtocolVersion::V3_8),
            (major, _) if major > 3 => Ok(ProtocolVersion::V3_8),
            _ => Err(unsupported()),
        }
    }
}
