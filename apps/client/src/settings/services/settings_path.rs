use std::path::PathBuf;

use crate::{SETTINGS_FILE_NAME, app_data_dir};

pub fn settings_path() -> Option<PathBuf> {
    app_data_dir().map(|dir| dir.join(SETTINGS_FILE_NAME))
}
