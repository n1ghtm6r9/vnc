use winit::keyboard::KeyCode;

use crate::keysym_for_key_code;

#[test]
fn test_lock_and_system_keys_send_their_named_keysym() {
    let rows = [
        (KeyCode::CapsLock, 0xffe5),
        (KeyCode::NumLock, 0xff7f),
        (KeyCode::ScrollLock, 0xff14),
        (KeyCode::PrintScreen, 0xff61),
        (KeyCode::Pause, 0xff13),
        (KeyCode::ContextMenu, 0xff67),
    ];
    for (code, expected) in rows {
        assert_eq!(keysym_for_key_code(code), Some(expected), "{code:?} sends its X11 keysym");
    }
}
