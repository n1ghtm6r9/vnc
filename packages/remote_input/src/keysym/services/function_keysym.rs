use winit::keyboard::KeyCode;

pub(crate) fn function_keysym(code: KeyCode) -> Option<u32> {
    match code {
        KeyCode::F1 => Some(0xffbe),
        KeyCode::F2 => Some(0xffbf),
        KeyCode::F3 => Some(0xffc0),
        KeyCode::F4 => Some(0xffc1),
        KeyCode::F5 => Some(0xffc2),
        KeyCode::F6 => Some(0xffc3),
        KeyCode::F7 => Some(0xffc4),
        KeyCode::F8 => Some(0xffc5),
        KeyCode::F9 => Some(0xffc6),
        KeyCode::F10 => Some(0xffc7),
        KeyCode::F11 => Some(0xffc8),
        KeyCode::F12 => Some(0xffc9),
        KeyCode::F13 => Some(0xffca),
        KeyCode::F14 => Some(0xffcb),
        KeyCode::F15 => Some(0xffcc),
        KeyCode::F16 => Some(0xffcd),
        KeyCode::F17 => Some(0xffce),
        KeyCode::F18 => Some(0xffcf),
        KeyCode::F19 => Some(0xffd0),
        KeyCode::F20 => Some(0xffd1),
        KeyCode::F21 => Some(0xffd2),
        KeyCode::F22 => Some(0xffd3),
        KeyCode::F23 => Some(0xffd4),
        KeyCode::F24 => Some(0xffd5),
        _ => None,
    }
}
