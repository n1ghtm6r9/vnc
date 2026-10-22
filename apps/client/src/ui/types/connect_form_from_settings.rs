use super::ConnectForm;
use crate::Settings;

impl ConnectForm {
    pub fn from_settings(settings: &Settings) -> ConnectForm {
        ConnectForm {
            host: settings.host.clone(),
            port: settings.port.to_string(),
            username: settings.username.clone(),
            password: String::new(),
            remember_password: settings.remember_password,
            apple_remote_desktop: settings.apple_remote_desktop,
            high_performance: settings.high_performance,
            retina: settings.retina,
            busy: false,
            error: None,
        }
    }
}
