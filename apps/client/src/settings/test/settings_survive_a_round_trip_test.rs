use std::fs;

use super::unique_temp_path::unique_temp_path;
use crate::{ImageQuality, Language, ScaleMode, SessionWindow, Settings, SoundPlace};

#[test]
fn test_settings_survive_a_round_trip() {
    let path = unique_temp_path("round_trip");
    let settings = Settings {
        host: "192.168.0.220".to_string(),
        port: 5901,
        username: "bogdan".to_string(),
        remember_password: false,
        scale_mode: ScaleMode::Actual,
        quality: ImageQuality::Lossless,
        sound: SoundPlace::Mac,
        swap_control_and_command: true,
        grab_keyboard: false,
        clipboard_sync: false,
        apple_remote_desktop: false,
        high_performance: true,
        retina: false,
        language: Some(Language::Ukrainian),
        session_window: Some(SessionWindow { width: 2560, height: 1368, maximized: true, fullscreen: false }),
    };
    settings.save(&path).unwrap();
    let loaded = Settings::load(&path);
    let _ = fs::remove_dir_all(path.parent().unwrap());
    assert_eq!(loaded, settings, "every field reads back as it was saved");
}
