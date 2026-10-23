use crate::{Language, starting_language};

#[test]
fn test_a_saved_language_comes_before_the_windows_language() {
    assert_eq!(
        starting_language(Some(Language::Ukrainian), &["ru-RU".to_owned()]),
        Language::Ukrainian,
        "the language picked in the client stays after a restart whatever Windows speaks"
    );
}
