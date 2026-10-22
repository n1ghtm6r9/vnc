use std::fs;

use super::unique_temp_path::unique_temp_path;
use crate::{DEFAULT_PORT, Settings};

#[test]
fn test_a_partial_settings_file_keeps_the_other_defaults() {
    let path = unique_temp_path("partial");
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(&path, r#"{ "host": "mac.local" }"#).unwrap();
    let settings = Settings::load(&path);
    let _ = fs::remove_dir_all(path.parent().unwrap());
    assert_eq!(settings.host, "mac.local", "the saved field is read");
    assert_eq!(settings.port, DEFAULT_PORT, "a field the file does not have keeps its default");
}
