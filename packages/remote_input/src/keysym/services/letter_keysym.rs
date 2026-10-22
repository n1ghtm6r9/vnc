use winit::keyboard::KeyCode;

pub(crate) fn letter_keysym(code: KeyCode) -> Option<u32> {
    match code {
        KeyCode::KeyA => Some(0x61),
        KeyCode::KeyB => Some(0x62),
        KeyCode::KeyC => Some(0x63),
        KeyCode::KeyD => Some(0x64),
        KeyCode::KeyE => Some(0x65),
        KeyCode::KeyF => Some(0x66),
        KeyCode::KeyG => Some(0x67),
        KeyCode::KeyH => Some(0x68),
        KeyCode::KeyI => Some(0x69),
        KeyCode::KeyJ => Some(0x6a),
        KeyCode::KeyK => Some(0x6b),
        KeyCode::KeyL => Some(0x6c),
        KeyCode::KeyM => Some(0x6d),
        KeyCode::KeyN => Some(0x6e),
        KeyCode::KeyO => Some(0x6f),
        KeyCode::KeyP => Some(0x70),
        KeyCode::KeyQ => Some(0x71),
        KeyCode::KeyR => Some(0x72),
        KeyCode::KeyS => Some(0x73),
        KeyCode::KeyT => Some(0x74),
        KeyCode::KeyU => Some(0x75),
        KeyCode::KeyV => Some(0x76),
        KeyCode::KeyW => Some(0x77),
        KeyCode::KeyX => Some(0x78),
        KeyCode::KeyY => Some(0x79),
        KeyCode::KeyZ => Some(0x7a),
        _ => None,
    }
}
