use crate::test_support::{EVENT_TIMEOUT, test_credentials};
use crate::{ConnectOptions, Quality, encodings_for_quality};

pub fn test_options(port: u16) -> ConnectOptions {
    ConnectOptions {
        host: "127.0.0.1".to_owned(),
        port,
        credentials: test_credentials(),
        shared: true,
        encodings: encodings_for_quality(Quality::Balanced),
        connect_timeout: EVENT_TIMEOUT,
        apple_remote_desktop: false,
        high_performance: None,
    }
}
