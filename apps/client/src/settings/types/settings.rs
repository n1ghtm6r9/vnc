use serde::{Deserialize, Serialize};

use crate::{DEFAULT_PORT, ImageQuality, ScaleMode, SoundPlace};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub remember_password: bool,
    pub scale_mode: ScaleMode,
    pub quality: ImageQuality,
    pub sound: SoundPlace,
    pub swap_control_and_command: bool,
    pub grab_keyboard: bool,
    pub clipboard_sync: bool,
    pub apple_remote_desktop: bool,
    pub high_performance: bool,
    pub retina: bool,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            host: String::new(),
            port: DEFAULT_PORT,
            username: String::new(),
            remember_password: true,
            scale_mode: ScaleMode::default(),
            quality: ImageQuality::default(),
            sound: SoundPlace::default(),
            swap_control_and_command: false,
            grab_keyboard: true,
            clipboard_sync: true,
            apple_remote_desktop: true,
            high_performance: false,
            retina: true,
        }
    }
}
