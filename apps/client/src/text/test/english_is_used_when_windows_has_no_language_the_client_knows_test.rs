use crate::{Language, starting_language};

#[test]
fn test_english_is_used_when_windows_has_no_language_the_client_knows() {
    for windows_languages in [vec!["de-DE".to_owned(), "pl-PL".to_owned()], Vec::new()] {
        assert_eq!(
            starting_language(None, &windows_languages),
            Language::English,
            "{windows_languages:?} has no English, Russian or Ukrainian, so the client speaks English"
        );
    }
}
