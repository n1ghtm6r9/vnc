use crate::preferred_ui_languages;

#[test]
fn test_windows_names_its_ui_languages() {
    let languages = preferred_ui_languages();
    assert!(!languages.is_empty(), "every Windows has a display language");
    assert!(
        languages.iter().all(|name| name.len() >= 2 && name.is_ascii() && !name.contains('\0')),
        "each display language is a name like en-US, got {languages:?}"
    );
}
