use std::io::Read;

use super::{AppleServerInit, ServerInit};
use crate::{PixelFormat, ProtocolError, apple_name_parts, read_text_bytes, read_u16};

impl AppleServerInit {
    pub fn read(reader: &mut impl Read) -> Result<AppleServerInit, ProtocolError> {
        let width = read_u16(reader)?;
        let height = read_u16(reader)?;
        let pixel_format = PixelFormat::read(reader)?;
        let (flags, capabilities, name) = apple_name_parts(&read_text_bytes(reader)?);
        Ok(AppleServerInit {
            init: ServerInit { width, height, pixel_format, name },
            flags,
            capabilities,
        })
    }
}
