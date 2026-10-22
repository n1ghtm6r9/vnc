use winit::keyboard::KeyCode;

pub(crate) fn punctuation_keysym(code: KeyCode) -> Option<u32> {
    match code {
        KeyCode::Minus => Some(0x2d),
        KeyCode::Equal => Some(0x3d),
        KeyCode::BracketLeft => Some(0x5b),
        KeyCode::BracketRight => Some(0x5d),
        KeyCode::Backslash => Some(0x5c),
        KeyCode::Semicolon => Some(0x3b),
        KeyCode::Quote => Some(0x27),
        KeyCode::Backquote => Some(0x60),
        KeyCode::Comma => Some(0x2c),
        KeyCode::Period => Some(0x2e),
        KeyCode::Slash => Some(0x2f),
        KeyCode::IntlBackslash => Some(0x3c),
        _ => None,
    }
}
