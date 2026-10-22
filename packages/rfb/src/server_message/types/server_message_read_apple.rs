use std::io::Read;

use super::ServerMessage;
use crate::{
    ProtocolError, SERVER_APPLE_ACK, SERVER_APPLE_CLIPBOARD, SERVER_APPLE_DISPLAY_INFO,
    SERVER_APPLE_DRAG, SERVER_APPLE_NOP, SERVER_APPLE_STATE_CHANGE, apple_display_geometry,
    apple_display_pixel_size, read_apple_clipboard, read_bytes, read_u16, read_u32, skip_bytes,
};

impl ServerMessage {
    pub(super) fn read_apple(
        message_type: u8,
        reader: &mut impl Read,
    ) -> Result<ServerMessage, ProtocolError> {
        match message_type {
            SERVER_APPLE_ACK => Ok(ServerMessage::AppleAck),
            SERVER_APPLE_NOP => Ok(ServerMessage::AppleNop),
            SERVER_APPLE_STATE_CHANGE => {
                skip_bytes(reader, 1)?;
                let size = read_u16(reader)?;
                let body = read_bytes(reader, usize::from(size))?;
                let status = body
                    .get(2..4)
                    .map_or(0, |status| u16::from_be_bytes([status[0], status[1]]));
                Ok(ServerMessage::AppleStateChange { status })
            }
            SERVER_APPLE_CLIPBOARD => {
                let (session_id, text) = read_apple_clipboard(reader)?;
                Ok(ServerMessage::AppleClipboard { session_id, text })
            }
            SERVER_APPLE_DRAG => {
                skip_bytes(reader, 4)?;
                let extra = read_u32(reader)?;
                skip_bytes(reader, extra as usize)?;
                Ok(ServerMessage::AppleDrag)
            }
            SERVER_APPLE_DISPLAY_INFO => {
                let size = read_u16(reader)?;
                let payload = read_bytes(reader, usize::from(size))?;
                Ok(ServerMessage::AppleDisplayInfo {
                    size: apple_display_pixel_size(&payload),
                    geometry: apple_display_geometry(&payload),
                })
            }
            other => Err(ProtocolError::UnknownMessage(other)),
        }
    }
}
