use std::fmt;

#[derive(Clone, PartialEq, Eq)]
pub struct ConnectRequest {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub remember_password: bool,
    pub apple_remote_desktop: bool,
    pub high_performance: bool,
    pub retina: bool,
}

impl fmt::Debug for ConnectRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ConnectRequest")
            .field("host", &self.host)
            .field("port", &self.port)
            .field("username", &self.username)
            .field("password", &"***")
            .field("remember_password", &self.remember_password)
            .field("apple_remote_desktop", &self.apple_remote_desktop)
            .field("high_performance", &self.high_performance)
            .field("retina", &self.retina)
            .finish()
    }
}
