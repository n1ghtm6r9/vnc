use crate::keysym_for_character;

#[test]
fn test_control_characters_send_no_keysym() {
    for character in ['\u{1}', '\r', '\u{7f}', '\u{85}'] {
        assert_eq!(
            keysym_for_character(character),
            None,
            "{character:?} is nothing to type, so the key falls back to its own keysym"
        );
    }
}
