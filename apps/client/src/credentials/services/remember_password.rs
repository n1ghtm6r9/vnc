use log::warn;
use platform::save_password;
use rfb_session::ConnectOptions;

use crate::credential_target;

pub fn remember_password(options: &ConnectOptions) {
    let credentials = &options.credentials;
    let target = credential_target(&credentials.username, &options.host, options.port);
    if let Err(error) = save_password(&target, &credentials.username, &credentials.password) {
        warn!("[UI] the password could not be remembered: {error}");
    }
}
