use winit::keyboard::KeyCode;

pub(crate) fn system_keysym(code: KeyCode) -> Option<u32> {
    match code {
        KeyCode::CapsLock => Some(0xffe5),
        KeyCode::NumLock => Some(0xff7f),
        KeyCode::ScrollLock => Some(0xff14),
        KeyCode::PrintScreen => Some(0xff61),
        KeyCode::Pause => Some(0xff13),
        KeyCode::ContextMenu => Some(0xff67),
        _ => None,
    }
}
