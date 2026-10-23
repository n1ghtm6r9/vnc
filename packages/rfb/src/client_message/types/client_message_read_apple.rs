use std::io::{self, Read};

use super::ClientMessage;
use crate::{
    CLIENT_APPLE_AUTO_PASTEBOARD, CLIENT_APPLE_AUTO_UPDATE, CLIENT_APPLE_CLIPBOARD,
    CLIENT_APPLE_CLIPBOARD_REQUEST, CLIENT_APPLE_ENCRYPTED_EVENT, CLIENT_APPLE_GESTURE,
    CLIENT_APPLE_SET_DISPLAY, CLIENT_APPLE_SET_INPUT_SOURCE, CLIENT_APPLE_SET_MODE, CLIENT_APPLE_VIEWER_INFO,
    ProtocolError,
    Rect, read_apple_clipboard, read_bytes, read_u8, read_u16, read_u32, skip_bytes,
};

impl ClientMessage {
    pub(super) fn read_apple(
        message_type: u8,
        reader: &mut impl Read,
    ) -> Result<ClientMessage, ProtocolError> {
        match message_type {
            CLIENT_APPLE_VIEWER_INFO => {
                skip_bytes(reader, 1)?;
                let size = read_u16(reader)?;
                skip_bytes(reader, usize::from(size))?;
                Ok(ClientMessage::AppleViewerInfo)
            }
            CLIENT_APPLE_SET_MODE => {
                skip_bytes(reader, 2)?;
                Ok(ClientMessage::AppleSetMode { mode: read_u8(reader)? })
            }
            CLIENT_APPLE_SET_DISPLAY => {
                let combine_all = read_u8(reader)? != 0;
                skip_bytes(reader, 2)?;
                let display_id = read_u32(reader)?;
                Ok(ClientMessage::AppleSetDisplay { combine_all, display_id })
            }
            CLIENT_APPLE_AUTO_PASTEBOARD => {
                skip_bytes(reader, 1)?;
                let enabled = read_u16(reader)? != 0;
                skip_bytes(reader, 4)?;
                Ok(ClientMessage::AppleAutoPasteboard { enabled })
            }
            CLIENT_APPLE_AUTO_UPDATE => {
                skip_bytes(reader, 1)?;
                let enabled = read_u16(reader)? != 0;
                skip_bytes(reader, 4)?;
                let rect = Rect::new(
                    read_u16(reader)?,
                    read_u16(reader)?,
                    read_u16(reader)?,
                    read_u16(reader)?,
                );
                Ok(ClientMessage::AppleAutoUpdate { enabled, rect })
            }
            CLIENT_APPLE_ENCRYPTED_EVENT => {
                skip_bytes(reader, 1)?;
                let mut block = [0u8; 16];
                reader.read_exact(&mut block)?;
                Ok(ClientMessage::AppleEncryptedEvent { block })
            }
            CLIENT_APPLE_CLIPBOARD_REQUEST => {
                skip_bytes(reader, 3)?;
                Ok(ClientMessage::AppleClipboardRequest { session_id: read_u32(reader)? })
            }
            CLIENT_APPLE_CLIPBOARD => {
                let (session_id, text) = read_apple_clipboard(reader)?;
                Ok(ClientMessage::AppleClipboard { session_id, text })
            }
            CLIENT_APPLE_GESTURE => {
                skip_bytes(reader, 1)?;
                let size = usize::from(read_u16(reader)?);
                if size < 4 {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("a gesture of {size} bytes is too short for its version and kind"),
                    )
                    .into());
                }
                let version = read_u16(reader)?;
                let kind = read_u16(reader)?;
                let data = read_bytes(reader, size - 4)?;
                Ok(ClientMessage::AppleGesture { version, kind, data })
            }
            CLIENT_APPLE_SET_INPUT_SOURCE => {
                skip_bytes(reader, 1)?;
                let size = usize::from(read_u16(reader)?);
                if size < 4 {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("an input source choice of {size} bytes is too short for its version and length"),
                    )
                    .into());
                }
                skip_bytes(reader, 2)?;
                let length = usize::from(read_u16(reader)?);
                let name = read_bytes(reader, size - 4)?;
                let source = String::from_utf8_lossy(&name[..length.min(name.len())]).into_owned();
                Ok(ClientMessage::AppleSetInputSource { source })
            }
            other => Err(ProtocolError::UnknownMessage(other)),
        }
    }
}
