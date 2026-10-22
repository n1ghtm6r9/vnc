use winit::dpi::PhysicalSize;

use crate::{ConnectRequest, ImageQuality, SoundPlace, connect_options};

#[test]
fn test_high_performance_is_not_asked_without_the_apple_mode() {
    let request = ConnectRequest {
        host: "192.168.0.220".to_owned(),
        port: 5900,
        username: "bogdan".to_owned(),
        password: "secret".to_owned(),
        remember_password: true,
        apple_remote_desktop: false,
        high_performance: true,
        retina: true,
    };

    let options = connect_options(&request, ImageQuality::Balanced, SoundPlace::Here, PhysicalSize::new(2304, 1296));

    assert_eq!(
        options.high_performance,
        None,
        "High Performance only exists in the Apple Remote Desktop mode, so a plain VNC session never asks for it"
    );
}
