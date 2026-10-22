use winit::keyboard::KeyCode;

use crate::keysym_for_key_code;

#[test]
fn test_function_keys_f1_to_f24_run_upwards_from_0xffbe() {
    let keys = [
        KeyCode::F1,
        KeyCode::F2,
        KeyCode::F3,
        KeyCode::F4,
        KeyCode::F5,
        KeyCode::F6,
        KeyCode::F7,
        KeyCode::F8,
        KeyCode::F9,
        KeyCode::F10,
        KeyCode::F11,
        KeyCode::F12,
        KeyCode::F13,
        KeyCode::F14,
        KeyCode::F15,
        KeyCode::F16,
        KeyCode::F17,
        KeyCode::F18,
        KeyCode::F19,
        KeyCode::F20,
        KeyCode::F21,
        KeyCode::F22,
        KeyCode::F23,
        KeyCode::F24,
    ];
    for (offset, code) in (0u32..).zip(keys) {
        assert_eq!(keysym_for_key_code(code), Some(0xffbe + offset), "{code:?} sits {offset} keysyms above F1");
    }
}
