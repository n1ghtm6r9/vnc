use winit::keyboard::KeyCode;

use crate::keysym_for_key_code;

#[test]
fn test_numpad_digits_send_keypad_keysyms() {
    let rows = [
        (KeyCode::Numpad0, 0xffb0),
        (KeyCode::Numpad1, 0xffb1),
        (KeyCode::Numpad2, 0xffb2),
        (KeyCode::Numpad3, 0xffb3),
        (KeyCode::Numpad4, 0xffb4),
        (KeyCode::Numpad5, 0xffb5),
        (KeyCode::Numpad6, 0xffb6),
        (KeyCode::Numpad7, 0xffb7),
        (KeyCode::Numpad8, 0xffb8),
        (KeyCode::Numpad9, 0xffb9),
    ];
    for (code, expected) in rows {
        assert_eq!(keysym_for_key_code(code), Some(expected), "{code:?} sends a keypad digit, not the top-row digit");
    }
}
