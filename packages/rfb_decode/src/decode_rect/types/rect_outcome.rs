use rfb::{AppleDisplayGeometry, Rect};

use crate::{CursorShape, SystemCursor};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RectOutcome {
    Pixels(Rect),
    Cursor(CursorShape),
    SystemCursor(SystemCursor),
    DesktopSize {
        width: u16,
        height: u16,
    },
    AppleDisplay {
        width: u16,
        height: u16,
        geometry: Option<AppleDisplayGeometry>,
    },
    AppleMedia {
        encoding: i32,
        payload: Vec<u8>,
    },
    AppleInputSource(String),
    LastRect,
    Ignored,
}
