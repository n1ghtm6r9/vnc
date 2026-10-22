use log::warn;
use platform::load_password;

use super::remembered_password_target;
use crate::Settings;

pub fn load_remembered_password(settings: &Settings) -> String {
    let Some(target) = remembered_password_target(settings) else {
        return String::new();
    };
    match load_password(&target) {
        Ok(password) => password.unwrap_or_default(),
        Err(error) => {
            warn!("[UI] the remembered password could not be read: {error}");
            String::new()
        }
    }
}
