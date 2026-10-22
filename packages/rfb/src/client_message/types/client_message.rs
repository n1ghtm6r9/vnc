use crate::{PixelFormat, Rect};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClientMessage {
    SetPixelFormat(PixelFormat),
    SetEncodings(Vec<i32>),
    FramebufferUpdateRequest { incremental: bool, rect: Rect },
    KeyEvent { down: bool, keysym: u32 },
    PointerEvent { buttons: u8, x: u16, y: u16 },
    ClientCutText { text: Vec<u8> },
    AppleViewerInfo,
    AppleSetMode { mode: u8 },
    AppleSetDisplay { combine_all: bool, display_id: u32 },
    AppleAutoPasteboard { enabled: bool },
    AppleAutoUpdate { enabled: bool, rect: Rect },
    AppleEncryptedEvent { block: [u8; 16] },
    AppleClipboardRequest { session_id: u32 },
    AppleClipboard { session_id: u32, text: Option<String> },
    AppleGesture { version: u16, kind: u16, data: Vec<u8> },
}
