use rfb_session::{ConnectOptions, Credentials, Quality, encodings_for_quality};
use winit::dpi::PhysicalSize;

use crate::{CONNECT_TIMEOUT, ConnectRequest, ImageQuality, SoundPlace, connect_options};

#[test]
fn test_the_connect_options_follow_the_form_and_the_quality() {
    let request = ConnectRequest {
        host: "192.168.0.220".to_owned(),
        port: 5900,
        username: "bogdan".to_owned(),
        password: "secret".to_owned(),
        remember_password: true,
        apple_remote_desktop: true,
        high_performance: false,
        retina: true,
    };

    let options = connect_options(&request, ImageQuality::Fast, SoundPlace::Here, PhysicalSize::new(2304, 1296));

    assert_eq!(
        options,
        ConnectOptions {
            host: "192.168.0.220".to_owned(),
            port: 5900,
            credentials: Credentials::new("bogdan", "secret"),
            shared: true,
            encodings: encodings_for_quality(Quality::Fast),
            connect_timeout: CONNECT_TIMEOUT,
            apple_remote_desktop: true,
            high_performance: None,
        },
        "the session goes to the address of the form as its user, shares the Mac, asks for the encodings of the chosen quality and keeps the Apple Remote Desktop choice"
    );
}
