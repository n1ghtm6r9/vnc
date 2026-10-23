use crate::{Language, starting_language};

#[test]
fn test_the_first_windows_language_the_client_knows_is_used() {
    let cases = [
        (vec!["uk-UA", "en-US"], Language::Ukrainian),
        (vec!["en-GB", "ru-RU"], Language::English),
        (vec!["de-DE", "ru-RU"], Language::Russian),
    ];
    for (windows_languages, expected) in cases {
        let windows_languages: Vec<String> = windows_languages.into_iter().map(str::to_owned).collect();
        assert_eq!(
            starting_language(None, &windows_languages),
            expected,
            "with no saved choice the client speaks the first language of {windows_languages:?} it knows"
        );
    }
}
