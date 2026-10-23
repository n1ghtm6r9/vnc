use crate::{ImageQuality, Language, ScaleMode, SoundPlace};

pub struct ToolbarView {
    pub language: Language,
    pub scale_mode: ScaleMode,
    pub quality: ImageQuality,
    pub sound: SoundPlace,
    pub sound_streamed: bool,
    pub high_performance_allowed: bool,
    pub swap_control_and_command: bool,
    pub fullscreen: bool,
    pub stats: Option<String>,
}
