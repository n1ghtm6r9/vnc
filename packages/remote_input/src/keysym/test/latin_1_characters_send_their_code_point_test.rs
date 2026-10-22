use crate::keysym_for_character;

#[test]
fn test_latin_1_characters_send_their_code_point() {
    for character in ['q', 'Q', '1', '!', ' ', '~', 'é', 'ÿ'] {
        assert_eq!(
            keysym_for_character(character),
            Some(u32::from(character)),
            "{character} is its own keysym in ASCII and Latin-1"
        );
    }
}
