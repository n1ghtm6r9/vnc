use std::fs;

use super::unique_temp_path::unique_temp_path;
use crate::{SessionWindow, Settings};

#[test]
fn test_a_remembered_window_with_missing_fields_keeps_the_other_settings() {
    let path = unique_temp_path("partial_window");
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(&path, r#"{ "host": "mac.local", "session_window": { "width": 2560, "height": 1368 } }"#).unwrap();
    let settings = Settings::load(&path);
    let _ = fs::remove_dir_all(path.parent().unwrap());
    assert_eq!(settings.host, "mac.local", "the other settings are still read");
    assert_eq!(
        settings.session_window,
        Some(SessionWindow { width: 2560, height: 1368, maximized: false, fullscreen: false }),
        "a field the remembered window lacks takes its default"
    );
}
