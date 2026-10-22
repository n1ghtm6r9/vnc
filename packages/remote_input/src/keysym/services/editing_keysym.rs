use winit::keyboard::KeyCode;

pub(crate) fn editing_keysym(code: KeyCode) -> Option<u32> {
    match code {
        KeyCode::Space => Some(0x20),
        KeyCode::Enter => Some(0xff0d),
        KeyCode::Tab => Some(0xff09),
        KeyCode::Backspace => Some(0xff08),
        KeyCode::Escape => Some(0xff1b),
        KeyCode::Delete => Some(0xffff),
        KeyCode::Insert => Some(0xff63),
        _ => None,
    }
}
