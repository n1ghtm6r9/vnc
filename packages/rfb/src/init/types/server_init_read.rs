use std::io::Read;

use super::ServerInit;
use crate::{PixelFormat, ProtocolError, read_text, read_u16};

impl ServerInit {
    pub fn read(reader: &mut impl Read) -> Result<ServerInit, ProtocolError> {
        let width = read_u16(reader)?;
        let height = read_u16(reader)?;
        let pixel_format = PixelFormat::read(reader)?;
        let name = read_text(reader)?;
        Ok(ServerInit { width, height, pixel_format, name })
    }
}
