use super::{ConnectForm, ConnectRequest};
use crate::{EMPTY_HOST_ERROR, EMPTY_USERNAME_ERROR, INVALID_PORT_ERROR};

impl ConnectForm {
    pub fn validate(&self) -> Result<ConnectRequest, &'static str> {
        let host = self.host.trim();
        if host.is_empty() {
            return Err(EMPTY_HOST_ERROR);
        }
        let port = match self.port.trim().parse::<u16>() {
            Ok(port) if port > 0 => port,
            _ => return Err(INVALID_PORT_ERROR),
        };
        let username = self.username.trim();
        if username.is_empty() {
            return Err(EMPTY_USERNAME_ERROR);
        }
        Ok(ConnectRequest {
            host: host.to_string(),
            port,
            username: username.to_string(),
            password: self.password.clone(),
            remember_password: self.remember_password,
            apple_remote_desktop: self.apple_remote_desktop,
            high_performance: self.high_performance,
            retina: self.retina,
        })
    }
}
