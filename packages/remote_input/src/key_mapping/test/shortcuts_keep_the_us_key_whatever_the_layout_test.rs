use winit::keyboard::{KeyCode, PhysicalKey};

use crate::{KeyMapping, keysym_for_typed_key};

#[test]
fn test_shortcuts_keep_the_us_key_whatever_the_layout() {
    for (code, character, expected) in [(KeyCode::KeyC, 'с', 0x63), (KeyCode::KeyV, 'м', 0x76)] {
        assert_eq!(
            keysym_for_typed_key(
                PhysicalKey::Code(code),
                Some(character),
                KeyMapping::default(),
                true
            ),
            Some(expected),
            "with Cmd, Ctrl or Option held {code:?} stays a Latin key, so Cmd+C copies in the Russian layout too"
        );
    }
}
