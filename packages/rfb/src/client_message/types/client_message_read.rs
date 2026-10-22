use std::io::{self, Read};

use super::ClientMessage;
use crate::{
    CLIENT_CUT_TEXT, CLIENT_FRAMEBUFFER_UPDATE_REQUEST, CLIENT_KEY_EVENT, CLIENT_POINTER_EVENT,
    CLIENT_SET_ENCODINGS, CLIENT_SET_PIXEL_FORMAT, MAX_CUT_TEXT_LENGTH, PixelFormat, ProtocolError,
    Rect, read_bytes, read_i32, read_u8, read_u16, read_u32, skip_bytes,
};

impl ClientMessage {
    pub fn read(reader: &mut impl Read) -> Result<ClientMessage, ProtocolError> {
        let message_type = read_u8(reader)?;
        match message_type {
            CLIENT_SET_PIXEL_FORMAT => {
                skip_bytes(reader, 3)?;
                Ok(ClientMessage::SetPixelFormat(PixelFormat::read(reader)?))
            }
            CLIENT_SET_ENCODINGS => {
                skip_bytes(reader, 1)?;
                let count = read_u16(reader)?;
                let encodings = (0..count)
                    .map(|_| read_i32(reader))
                    .collect::<io::Result<Vec<i32>>>()?;
                Ok(ClientMessage::SetEncodings(encodings))
            }
            CLIENT_FRAMEBUFFER_UPDATE_REQUEST => {
                let incremental = read_u8(reader)? != 0;
                let rect = Rect::new(
                    read_u16(reader)?,
                    read_u16(reader)?,
                    read_u16(reader)?,
                    read_u16(reader)?,
                );
                Ok(ClientMessage::FramebufferUpdateRequest { incremental, rect })
            }
            CLIENT_KEY_EVENT => {
                let down = read_u8(reader)? != 0;
                skip_bytes(reader, 2)?;
                let keysym = read_u32(reader)?;
                Ok(ClientMessage::KeyEvent { down, keysym })
            }
            CLIENT_POINTER_EVENT => {
                let buttons = read_u8(reader)?;
                let x = read_u16(reader)?;
                let y = read_u16(reader)?;
                Ok(ClientMessage::PointerEvent { buttons, x, y })
            }
            CLIENT_CUT_TEXT => {
                skip_bytes(reader, 3)?;
                let len = read_u32(reader)?;
                if len > MAX_CUT_TEXT_LENGTH {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("a clipboard text of {len} bytes is longer than this program accepts"),
                    )
                    .into());
                }
                let text = read_bytes(reader, len as usize)?;
                Ok(ClientMessage::ClientCutText { text })
            }
            other => ClientMessage::read_apple(other, reader),
        }
    }
}
