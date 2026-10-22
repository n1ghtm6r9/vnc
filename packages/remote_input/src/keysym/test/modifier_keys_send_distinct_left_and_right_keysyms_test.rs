use winit::keyboard::KeyCode;

use crate::keysym_for_key_code;

#[test]
fn test_modifier_keys_send_distinct_left_and_right_keysyms() {
    let rows = [
        (KeyCode::ShiftLeft, 0xffe1),
        (KeyCode::ShiftRight, 0xffe2),
        (KeyCode::ControlLeft, 0xffe3),
        (KeyCode::ControlRight, 0xffe4),
        (KeyCode::AltLeft, 0xffe9),
        (KeyCode::AltRight, 0xffea),
        (KeyCode::SuperLeft, 0xffeb),
        (KeyCode::SuperRight, 0xffec),
    ];
    for (code, expected) in rows {
        assert_eq!(keysym_for_key_code(code), Some(expected), "{code:?} keeps its own side so the Mac sees which key was held");
    }
}
