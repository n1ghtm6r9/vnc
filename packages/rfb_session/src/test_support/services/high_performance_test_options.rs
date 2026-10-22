use crate::test_support::apple_test_options;
use crate::{ConnectOptions, HighPerformanceOptions};

pub fn high_performance_test_options(port: u16) -> ConnectOptions {
    ConnectOptions {
        high_performance: Some(HighPerformanceOptions {
            display_width: 1280,
            display_height: 720,
            display_scale: 2,
            play_sound: true,
        }),
        ..apple_test_options(port)
    }
}
