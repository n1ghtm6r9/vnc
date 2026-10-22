use winit::keyboard::KeyCode;

use crate::keysym_for_key_code;

#[test]
fn test_numpad_operators_send_keypad_keysyms() {
    let rows = [
        (KeyCode::NumpadDecimal, 0xffae),
        (KeyCode::NumpadDivide, 0xffaf),
        (KeyCode::NumpadMultiply, 0xffaa),
        (KeyCode::NumpadSubtract, 0xffad),
        (KeyCode::NumpadAdd, 0xffab),
        (KeyCode::NumpadEnter, 0xff8d),
        (KeyCode::NumpadEqual, 0xffbd),
    ];
    for (code, expected) in rows {
        assert_eq!(keysym_for_key_code(code), Some(expected), "{code:?} sends its keypad keysym");
    }
}
