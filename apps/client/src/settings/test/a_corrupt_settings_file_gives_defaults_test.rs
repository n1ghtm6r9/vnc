use std::fs;

use super::unique_temp_path::unique_temp_path;
use crate::Settings;

#[test]
fn test_a_corrupt_settings_file_gives_defaults() {
    let path = unique_temp_path("corrupt");
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(&path, "{ not json").unwrap();
    let settings = Settings::load(&path);
    let _ = fs::remove_dir_all(path.parent().unwrap());
    assert_eq!(settings, Settings::default(), "a broken file never stops the program from starting");
}
