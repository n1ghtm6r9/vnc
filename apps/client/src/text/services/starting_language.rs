use crate::{Language, language_of_locale};

pub fn starting_language(saved: Option<Language>, windows_languages: &[String]) -> Language {
    saved
        .or_else(|| windows_languages.iter().find_map(|locale| language_of_locale(locale)))
        .unwrap_or(Language::English)
}
