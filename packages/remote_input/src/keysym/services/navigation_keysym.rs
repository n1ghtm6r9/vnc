use winit::keyboard::KeyCode;

pub(crate) fn navigation_keysym(code: KeyCode) -> Option<u32> {
    match code {
        KeyCode::Home => Some(0xff50),
        KeyCode::End => Some(0xff57),
        KeyCode::PageUp => Some(0xff55),
        KeyCode::PageDown => Some(0xff56),
        KeyCode::ArrowLeft => Some(0xff51),
        KeyCode::ArrowUp => Some(0xff52),
        KeyCode::ArrowRight => Some(0xff53),
        KeyCode::ArrowDown => Some(0xff54),
        _ => None,
    }
}
