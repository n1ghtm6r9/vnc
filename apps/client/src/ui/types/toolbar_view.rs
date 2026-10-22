use crate::{ImageQuality, ScaleMode, SoundPlace};

pub struct ToolbarView {
    pub scale_mode: ScaleMode,
    pub quality: ImageQuality,
    pub sound: SoundPlace,
    pub sound_streamed: bool,
    pub swap_control_and_command: bool,
    pub fullscreen: bool,
    pub stats: Option<String>,
}
