use crate::keysym_for_character;

#[test]
fn test_characters_beyond_latin_1_send_unicode_keysyms() {
    let rows = [
        ('й', 0x0100_0439),
        ('Ё', 0x0100_0401),
        ('№', 0x0100_2116),
        ('€', 0x0100_20ac),
    ];
    for (character, expected) in rows {
        assert_eq!(
            keysym_for_character(character),
            Some(expected),
            "{character} goes as a Unicode keysym, which Apple's server types whatever input source the Mac has"
        );
    }
}
