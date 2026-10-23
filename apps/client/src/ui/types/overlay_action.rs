use crate::{ImageQuality, Language, ScaleMode, SoundPlace};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverlayAction {
    Disconnect,
    CancelReconnect,
    ScaleMode(ScaleMode),
    Quality(ImageQuality),
    Sound(SoundPlace),
    Language(Language),
    SwapControlAndCommand(bool),
    Fullscreen(bool),
    Refresh,
}
