use winit::keyboard::{KeyCode, PhysicalKey};

use crate::{KeyMapping, keysym_for_typed_key};

#[test]
fn test_letters_and_signs_follow_the_windows_layout() {
    let rows = [
        (KeyCode::KeyQ, 'й', 0x0100_0439),
        (KeyCode::KeyQ, 'Й', 0x0100_0419),
        (KeyCode::Backquote, 'ё', 0x0100_0451),
        (KeyCode::Digit3, '№', 0x0100_2116),
        (KeyCode::Slash, '.', 0x2e),
        (KeyCode::KeyQ, 'Q', 0x51),
    ];
    for (code, character, expected) in rows {
        assert_eq!(
            keysym_for_typed_key(
                PhysicalKey::Code(code),
                Some(character),
                KeyMapping::default(),
                false
            ),
            Some(expected),
            "{code:?} types {character} from the Windows layout, because Apple's server types the keysym it gets and ignores the Mac's input source"
        );
    }
}
