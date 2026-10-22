use crate::ConnectOptions;
use crate::test_support::test_options;

pub fn apple_test_options(port: u16) -> ConnectOptions {
    ConnectOptions {
        apple_remote_desktop: true,
        ..test_options(port)
    }
}
