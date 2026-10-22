use winit::keyboard::{KeyCode, PhysicalKey};

use crate::PressedKeys;

#[test]
fn test_draining_pressed_keys_returns_every_held_key_once() {
    let mut keys = PressedKeys::new();
    let shift = PhysicalKey::Code(KeyCode::ShiftLeft);
    let a = PhysicalKey::Code(KeyCode::KeyA);
    keys.press(shift, 0xffe1);
    keys.press(PhysicalKey::Code(KeyCode::ControlLeft), 0xffe3);
    keys.press(shift, 0xffe1);
    keys.press(a, 0x61);
    keys.release(a);
    let mut drained = keys.drain();
    drained.sort_unstable();
    assert_eq!(
        drained,
        vec![0xffe1, 0xffe3],
        "a held key comes back once however often it repeated, and a released key does not come back"
    );
    assert!(
        keys.drain().is_empty(),
        "draining empties the set, so a second drain finds nothing"
    );
    assert_eq!(keys.release(shift), None, "a drained key is no longer held");
}
