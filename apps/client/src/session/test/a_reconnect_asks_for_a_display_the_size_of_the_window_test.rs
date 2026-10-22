use rfb_session::HighPerformanceOptions;
use winit::dpi::PhysicalSize;

use crate::{ConnectRequest, ImageQuality, SoundPlace, connect_options, fit_display_to_window};

#[test]
fn test_a_reconnect_asks_for_a_display_the_size_of_the_window() {
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
    let mut options = connect_options(&request, ImageQuality::Balanced, SoundPlace::Here, PhysicalSize::new(2304, 1296));

    fit_display_to_window(&mut options, PhysicalSize::new(2560, 1440));

    assert_eq!(
        options.high_performance,
        Some(HighPerformanceOptions {
            display_width: 2560,
            display_height: 1440,
            display_scale: 2,
            play_sound: true,
        }),
        "a reconnect asks the Mac for a display the size the window has now"
    );
}
