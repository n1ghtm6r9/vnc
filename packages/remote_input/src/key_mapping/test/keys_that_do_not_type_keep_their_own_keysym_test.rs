use winit::keyboard::{KeyCode, PhysicalKey};

use crate::{KeyMapping, keysym_for_typed_key};

#[test]
fn test_keys_that_do_not_type_keep_their_own_keysym() {
    let rows = [
        (KeyCode::Numpad1, Some('1'), 0xffb1),
        (KeyCode::Space, Some(' '), 0x20),
        (KeyCode::Enter, None, 0xff0d),
        (KeyCode::KeyQ, None, 0x71),
        (KeyCode::KeyQ, Some('\u{11}'), 0x71),
    ];
    for (code, character, expected) in rows {
        assert_eq!(
            keysym_for_typed_key(
                PhysicalKey::Code(code),
                character,
                KeyMapping::default(),
                false
            ),
            Some(expected),
            "{code:?} with {character:?} sends its own keysym"
        );
    }
}
