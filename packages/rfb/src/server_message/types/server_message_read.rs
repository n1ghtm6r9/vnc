use std::io::{self, Read};

use super::ServerMessage;
use crate::{
    MAX_CUT_TEXT_LENGTH, ProtocolError, SERVER_BELL, SERVER_CUT_TEXT, SERVER_FRAMEBUFFER_UPDATE,
    SERVER_SET_COLOUR_MAP_ENTRIES, read_bytes, read_u8, read_u16, read_u32, skip_bytes,
};

impl ServerMessage {
    pub fn read(reader: &mut impl Read) -> Result<ServerMessage, ProtocolError> {
        let message_type = read_u8(reader)?;
        match message_type {
            SERVER_FRAMEBUFFER_UPDATE => {
                skip_bytes(reader, 1)?;
                let rect_count = read_u16(reader)?;
                Ok(ServerMessage::FramebufferUpdate { rect_count })
            }
            SERVER_SET_COLOUR_MAP_ENTRIES => {
                skip_bytes(reader, 1)?;
                let first_colour = read_u16(reader)?;
                let count = read_u16(reader)?;
                let colours = (0..count)
                    .map(|_| Ok([read_u16(reader)?, read_u16(reader)?, read_u16(reader)?]))
                    .collect::<io::Result<Vec<[u16; 3]>>>()?;
                Ok(ServerMessage::SetColourMapEntries { first_colour, colours })
            }
            SERVER_BELL => Ok(ServerMessage::Bell),
            SERVER_CUT_TEXT => {
                skip_bytes(reader, 3)?;
                let len = read_u32(reader)?;
                if len > MAX_CUT_TEXT_LENGTH {
                    skip_bytes(reader, len as usize)?;
                    return Ok(ServerMessage::OversizedCutText { length: len });
                }
                let text = read_bytes(reader, len as usize)?;
                Ok(ServerMessage::ServerCutText { text })
            }
            other => ServerMessage::read_apple(other, reader),
        }
    }
}
