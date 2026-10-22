use std::fs;
use std::io;
use std::path::Path;

use log::warn;

use super::Settings;

impl Settings {
    pub fn load(path: &Path) -> Settings {
        let text = match fs::read_to_string(path) {
            Ok(text) => text,
            Err(error) if error.kind() == io::ErrorKind::NotFound => return Settings::default(),
            Err(error) => {
                warn!("[UI] settings at {} could not be read, defaults are used: {error}", path.display());
                return Settings::default();
            }
        };
        serde_json::from_str(&text).unwrap_or_else(|error| {
            warn!("[UI] settings at {} are not valid json, defaults are used: {error}", path.display());
            Settings::default()
        })
    }
}
