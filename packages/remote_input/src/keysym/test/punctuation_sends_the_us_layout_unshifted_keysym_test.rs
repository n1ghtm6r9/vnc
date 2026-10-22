use winit::keyboard::KeyCode;

use crate::keysym_for_key_code;

#[test]
fn test_punctuation_sends_the_us_layout_unshifted_keysym() {
    let rows = [
        (KeyCode::Minus, 0x2d),
        (KeyCode::Equal, 0x3d),
        (KeyCode::BracketLeft, 0x5b),
        (KeyCode::BracketRight, 0x5d),
        (KeyCode::Backslash, 0x5c),
        (KeyCode::Semicolon, 0x3b),
        (KeyCode::Quote, 0x27),
        (KeyCode::Backquote, 0x60),
        (KeyCode::Comma, 0x2c),
        (KeyCode::Period, 0x2e),
        (KeyCode::Slash, 0x2f),
        (KeyCode::IntlBackslash, 0x3c),
    ];
    for (code, expected) in rows {
        assert_eq!(keysym_for_key_code(code), Some(expected), "{code:?} sends the character printed on the US-layout key");
    }
}
