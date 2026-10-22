use std::path::PathBuf;

use crate::APP_DIR_NAME;

pub fn app_data_dir() -> Option<PathBuf> {
    let base = std::env::var_os("APPDATA")?;
    Some(PathBuf::from(base).join(APP_DIR_NAME))
}
