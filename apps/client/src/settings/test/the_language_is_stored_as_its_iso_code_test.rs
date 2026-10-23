use std::fs;

use super::unique_temp_path::unique_temp_path;
use crate::{Language, Settings};

#[test]
fn test_the_language_is_stored_as_its_iso_code() {
    let path = unique_temp_path("language");
    let settings = Settings { language: Some(Language::Ukrainian), ..Settings::default() };
    settings.save(&path).unwrap();
    let saved = fs::read_to_string(&path).unwrap();
    fs::write(&path, r#"{ "language": "ru" }"#).unwrap();
    let loaded = Settings::load(&path);
    let _ = fs::remove_dir_all(path.parent().unwrap());
    assert!(saved.contains(r#""language": "uk""#), "Ukrainian is saved as \"uk\", got {saved}");
    assert_eq!(loaded.language, Some(Language::Russian), "\"ru\" in the settings file reads as Russian");
}
