use winit::keyboard::KeyCode;

use crate::keysym_for_key_code;

#[test]
fn test_keys_without_a_us_layout_keysym_send_nothing() {
    let unmapped = [
        KeyCode::Fn,
        KeyCode::F25,
        KeyCode::IntlRo,
        KeyCode::MediaPlayPause,
        KeyCode::AudioVolumeUp,
    ];
    for code in unmapped {
        assert_eq!(keysym_for_key_code(code), None, "{code:?} has no keysym in the table, so nothing is sent");
    }
}
