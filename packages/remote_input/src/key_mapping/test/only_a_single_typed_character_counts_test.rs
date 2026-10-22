use winit::keyboard::{Key, NamedKey};

use crate::character_of_key;

#[test]
fn test_only_a_single_typed_character_counts() {
    assert_eq!(
        character_of_key(&Key::Character("й".into())),
        Some('й'),
        "a key that types one character gives that character"
    );
    for key in [
        Key::Character("ab".into()),
        Key::Character("".into()),
        Key::Named(NamedKey::Enter),
        Key::Dead(Some('´')),
    ] {
        assert_eq!(
            character_of_key(&key),
            None,
            "{key:?} gives no single character, so the key falls back to its own keysym"
        );
    }
}
