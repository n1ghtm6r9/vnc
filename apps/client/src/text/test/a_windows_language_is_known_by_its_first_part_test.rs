use crate::{Language, language_of_locale};

#[test]
fn test_a_windows_language_is_known_by_its_first_part() {
    let cases = [
        ("uk-UA", Some(Language::Ukrainian)),
        ("ru", Some(Language::Russian)),
        ("ru-UA", Some(Language::Russian)),
        ("EN-gb", Some(Language::English)),
        ("en-US", Some(Language::English)),
        ("be-BY", None),
        ("", None),
    ];
    for (locale, expected) in cases {
        assert_eq!(language_of_locale(locale), expected, "{locale:?} names the language before the dash");
    }
}
