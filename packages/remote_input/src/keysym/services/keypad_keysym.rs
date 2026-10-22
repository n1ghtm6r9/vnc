use winit::keyboard::KeyCode;

pub(crate) fn keypad_keysym(code: KeyCode) -> Option<u32> {
    match code {
        KeyCode::Numpad0 => Some(0xffb0),
        KeyCode::Numpad1 => Some(0xffb1),
        KeyCode::Numpad2 => Some(0xffb2),
        KeyCode::Numpad3 => Some(0xffb3),
        KeyCode::Numpad4 => Some(0xffb4),
        KeyCode::Numpad5 => Some(0xffb5),
        KeyCode::Numpad6 => Some(0xffb6),
        KeyCode::Numpad7 => Some(0xffb7),
        KeyCode::Numpad8 => Some(0xffb8),
        KeyCode::Numpad9 => Some(0xffb9),
        KeyCode::NumpadDecimal => Some(0xffae),
        KeyCode::NumpadDivide => Some(0xffaf),
        KeyCode::NumpadMultiply => Some(0xffaa),
        KeyCode::NumpadSubtract => Some(0xffad),
        KeyCode::NumpadAdd => Some(0xffab),
        KeyCode::NumpadEnter => Some(0xff8d),
        KeyCode::NumpadEqual => Some(0xffbd),
        _ => None,
    }
}
