use std::io::{self, Write};

use crate::CLIENT_APPLE_VIEWER_INFO;

pub fn write_apple_viewer_info(writer: &mut impl Write) -> io::Result<()> {
    let mut bytes = vec![CLIENT_APPLE_VIEWER_INFO, 0];
    bytes.extend_from_slice(&62u16.to_be_bytes());
    bytes.extend_from_slice(&1u16.to_be_bytes());
    bytes.extend_from_slice(&2u32.to_be_bytes());
    for version in [6u32, 1, 0, 15, 0, 0] {
        bytes.extend_from_slice(&version.to_be_bytes());
    }
    let mut capabilities = [0u8; 32];
    capabilities[0] = 0xb0;
    capabilities[2] = 0x0c;
    capabilities[3] = 0x03;
    capabilities[4] = 0x90;
    capabilities[10] = 0x40;
    bytes.extend_from_slice(&capabilities);
    writer.write_all(&bytes)
}
