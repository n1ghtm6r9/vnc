use crate::Settings;

#[test]
fn test_apple_remote_desktop_mode_is_on_by_default() {
    assert!(
        Settings::default().apple_remote_desktop,
        "a fresh install and an older settings file connect in the faster Apple Remote Desktop mode"
    );
}
