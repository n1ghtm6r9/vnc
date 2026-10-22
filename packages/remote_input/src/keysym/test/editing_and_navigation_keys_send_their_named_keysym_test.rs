use winit::keyboard::KeyCode;

use crate::keysym_for_key_code;

#[test]
fn test_editing_and_navigation_keys_send_their_named_keysym() {
    let rows = [
        (KeyCode::Space, 0x20),
        (KeyCode::Enter, 0xff0d),
        (KeyCode::Tab, 0xff09),
        (KeyCode::Backspace, 0xff08),
        (KeyCode::Escape, 0xff1b),
        (KeyCode::Delete, 0xffff),
        (KeyCode::Insert, 0xff63),
        (KeyCode::Home, 0xff50),
        (KeyCode::End, 0xff57),
        (KeyCode::PageUp, 0xff55),
        (KeyCode::PageDown, 0xff56),
    ];
    for (code, expected) in rows {
        assert_eq!(keysym_for_key_code(code), Some(expected), "{code:?} sends its X11 keysym");
    }
}
