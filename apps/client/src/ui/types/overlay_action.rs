use crate::{ImageQuality, ScaleMode, SoundPlace};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverlayAction {
    Disconnect,
    CancelReconnect,
    ScaleMode(ScaleMode),
    Quality(ImageQuality),
    Sound(SoundPlace),
    SwapControlAndCommand(bool),
    Fullscreen(bool),
    Refresh,
}
