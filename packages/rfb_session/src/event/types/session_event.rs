use hp_media::{MediaState, StripeImage};
use rfb::AppleDisplayGeometry;
use rfb_decode::{CursorShape, SystemCursor};

use crate::{Patch, SessionError, SessionStats};

#[derive(Debug)]
pub enum SessionEvent {
    Connected {
        width: u16,
        height: u16,
        name: String,
        apple_remote_desktop: bool,
    },
    FrameUpdated {
        patches: Vec<Patch>,
    },
    PictureStripe(StripeImage),
    Resized {
        width: u16,
        height: u16,
    },
    DisplayGeometry(AppleDisplayGeometry),
    CursorChanged(CursorShape),
    SystemCursorChanged(SystemCursor),
    CursorVisibilityChanged {
        visible: bool,
    },
    Bell,
    ClipboardText(String),
    InputSource(String),
    Stats(SessionStats),
    HighPerformance(MediaState),
    Disconnected {
        error: Option<SessionError>,
    },
}
