use crate::Language;

pub fn language_of_locale(locale: &str) -> Option<Language> {
    let code = locale.split('-').next()?.to_ascii_lowercase();
    match code.as_str() {
        "en" => Some(Language::English),
        "ru" => Some(Language::Russian),
        "uk" => Some(Language::Ukrainian),
        _ => None,
    }
}
