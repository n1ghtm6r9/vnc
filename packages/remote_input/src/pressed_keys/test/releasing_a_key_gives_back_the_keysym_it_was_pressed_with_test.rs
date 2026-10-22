use winit::keyboard::{KeyCode, PhysicalKey};

use crate::PressedKeys;

#[test]
fn test_releasing_a_key_gives_back_the_keysym_it_was_pressed_with() {
    let mut keys = PressedKeys::new();
    let q = PhysicalKey::Code(KeyCode::KeyQ);
    assert_eq!(keys.release(q), None, "a key that was never pressed was not held");
    keys.press(q, 0x0100_0419);
    assert_eq!(
        keys.release(PhysicalKey::Code(KeyCode::KeyW)),
        None,
        "releasing another key does not touch the held one"
    );
    assert_eq!(
        keys.release(q),
        Some(0x0100_0419),
        "a held key goes up with the keysym it went down with, whatever the layout gives now"
    );
    assert_eq!(keys.release(q), None, "a second release finds nothing held");
}
