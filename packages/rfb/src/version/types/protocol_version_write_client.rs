use std::io::{self, Write};

use super::ProtocolVersion;

impl ProtocolVersion {
    pub fn write_client(&self, writer: &mut impl Write) -> io::Result<()> {
        let banner: &[u8; 12] = match self {
            ProtocolVersion::V3_3 => b"RFB 003.003\n",
            ProtocolVersion::V3_7 => b"RFB 003.007\n",
            ProtocolVersion::V3_8 => b"RFB 003.008\n",
            ProtocolVersion::AppleRemoteDesktop => b"RFB 003.889\n",
        };
        writer.write_all(banner)
    }
}
