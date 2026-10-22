use crate::{Settings, credential_target};

pub fn remembered_password_target(settings: &Settings) -> Option<String> {
    let known = !settings.username.is_empty() && !settings.host.is_empty();
    (settings.remember_password && known).then(|| credential_target(&settings.username, &settings.host, settings.port))
}
