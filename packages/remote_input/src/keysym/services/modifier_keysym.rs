use winit::keyboard::KeyCode;

use crate::{KEYSYM_SUPER_LEFT, KEYSYM_SUPER_RIGHT};

pub(crate) fn modifier_keysym(code: KeyCode) -> Option<u32> {
    match code {
        KeyCode::ShiftLeft => Some(0xffe1),
        KeyCode::ShiftRight => Some(0xffe2),
        KeyCode::ControlLeft => Some(0xffe3),
        KeyCode::ControlRight => Some(0xffe4),
        KeyCode::AltLeft => Some(0xffe9),
        KeyCode::AltRight => Some(0xffea),
        KeyCode::SuperLeft => Some(KEYSYM_SUPER_LEFT),
        KeyCode::SuperRight => Some(KEYSYM_SUPER_RIGHT),
        _ => None,
    }
}
