use winit::keyboard::KeyCode;

pub(crate) fn digit_keysym(code: KeyCode) -> Option<u32> {
    match code {
        KeyCode::Digit0 => Some(0x30),
        KeyCode::Digit1 => Some(0x31),
        KeyCode::Digit2 => Some(0x32),
        KeyCode::Digit3 => Some(0x33),
        KeyCode::Digit4 => Some(0x34),
        KeyCode::Digit5 => Some(0x35),
        KeyCode::Digit6 => Some(0x36),
        KeyCode::Digit7 => Some(0x37),
        KeyCode::Digit8 => Some(0x38),
        KeyCode::Digit9 => Some(0x39),
        _ => None,
    }
}
