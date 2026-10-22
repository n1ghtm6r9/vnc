use winit::dpi::PhysicalSize;

use crate::{ConnectRequest, ImageQuality, SoundPlace, connect_options};

#[test]
fn test_a_retina_display_asks_for_two_pixels_to_the_point() {
    for (retina, expected) in [(true, 2), (false, 1)] {
        let request = ConnectRequest {
            host: "192.168.0.220".to_owned(),
            port: 5900,
            username: "bogdan".to_owned(),
            password: "secret".to_owned(),
            remember_password: true,
            apple_remote_desktop: true,
            high_performance: true,
            retina,
        };

        let options = connect_options(&request, ImageQuality::Balanced, SoundPlace::Here, PhysicalSize::new(2304, 1296));

        assert_eq!(
            options.high_performance.map(|display| display.display_scale),
            Some(expected),
            "Retina {retina} asks the Mac to draw {expected} pixels to the point"
        );
    }
}
