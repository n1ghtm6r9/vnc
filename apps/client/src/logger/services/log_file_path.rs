use std::path::PathBuf;

use crate::{LOG_FILE_NAME, app_data_dir};

pub fn log_file_path() -> Option<PathBuf> {
    app_data_dir().map(|dir| dir.join(LOG_FILE_NAME))
}
