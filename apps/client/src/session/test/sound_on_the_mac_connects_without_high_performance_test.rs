use winit::dpi::PhysicalSize;

use crate::{ConnectRequest, ImageQuality, SoundPlace, connect_options};

#[test]
fn test_sound_on_the_mac_connects_without_high_performance() {
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

    let options = connect_options(&request, ImageQuality::Balanced, SoundPlace::Mac, PhysicalSize::new(2304, 1296));

    assert!(
        options.high_performance.is_none(),
        "sound kept on the Mac needs standard mode, because High Performance always takes the Mac's sound"
    );
    assert!(options.apple_remote_desktop, "the session without High Performance stays in Apple Remote Desktop mode");
}
