use super::unique_temp_path::unique_temp_path;
use crate::{DEFAULT_PORT, ScaleMode, Settings};

#[test]
fn test_missing_settings_give_defaults() {
    let settings = Settings::load(&unique_temp_path("missing"));
    assert_eq!(settings, Settings::default(), "a first start uses the defaults");
    assert_eq!(settings.port, DEFAULT_PORT, "the default port is the VNC port 5900");
    assert_eq!(settings.scale_mode, ScaleMode::Fit, "the screen fits the window by default");
}
