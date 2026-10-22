use std::time::Duration;

use rfb_auth::Credentials;

use super::HighPerformanceOptions;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectOptions {
    pub host: String,
    pub port: u16,
    pub credentials: Credentials,
    pub shared: bool,
    pub encodings: Vec<i32>,
    pub connect_timeout: Duration,
    pub apple_remote_desktop: bool,
    pub high_performance: Option<HighPerformanceOptions>,
}
