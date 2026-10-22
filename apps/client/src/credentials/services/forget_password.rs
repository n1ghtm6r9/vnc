use log::warn;
use platform::delete_password;

use crate::{ConnectRequest, credential_target};

pub fn forget_password(request: &ConnectRequest) {
    let target = credential_target(&request.username, &request.host, request.port);
    if let Err(error) = delete_password(&target) {
        warn!("[UI] the remembered password could not be removed: {error}");
    }
}
