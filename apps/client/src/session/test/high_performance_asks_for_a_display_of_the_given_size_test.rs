use rfb_session::HighPerformanceOptions;
use winit::dpi::PhysicalSize;

use crate::{ConnectRequest, ImageQuality, SoundPlace, connect_options};

#[test]
fn test_high_performance_asks_for_a_display_of_the_given_size() {
    let request = ConnectRequest {
        host: "192.168.0.220".to_owned(),
        port: 5900,
        username: "bogdan".to_owned(),
        password: "secret".to_owned(),
        remember_password: true,
        apple_remote_desktop: true,
        high_performance: true,
        retina: true,
    };

    let options = connect_options(&request, ImageQuality::Balanced, SoundPlace::Here, PhysicalSize::new(2304, 1296));

    assert_eq!(
        options.high_performance,
        Some(HighPerformanceOptions {
            display_width: 2304,
            display_height: 1296,
            display_scale: 2,
            play_sound: true,
        }),
        "a High Performance session asks the Mac for a display of the size the window will show"
    );
}
