use winit::keyboard::KeyCode;

use crate::keysym_for_key_code;

#[test]
fn test_digits_send_their_ascii_keysym() {
    let rows = [
        (KeyCode::Digit0, 0x30),
        (KeyCode::Digit1, 0x31),
        (KeyCode::Digit2, 0x32),
        (KeyCode::Digit3, 0x33),
        (KeyCode::Digit4, 0x34),
        (KeyCode::Digit5, 0x35),
        (KeyCode::Digit6, 0x36),
        (KeyCode::Digit7, 0x37),
        (KeyCode::Digit8, 0x38),
        (KeyCode::Digit9, 0x39),
    ];
    for (code, expected) in rows {
        assert_eq!(keysym_for_key_code(code), Some(expected), "{code:?} sends the unshifted ASCII digit of the top row");
    }
}
