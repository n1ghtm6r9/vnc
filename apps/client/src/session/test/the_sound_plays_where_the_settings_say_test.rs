use winit::dpi::PhysicalSize;

use crate::{ConnectRequest, ImageQuality, SoundPlace, connect_options};

#[test]
fn test_the_sound_plays_where_the_settings_say() {
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

    for (sound, played_here) in [(SoundPlace::Here, true), (SoundPlace::Mac, false)] {
        let options = connect_options(&request, ImageQuality::Balanced, sound, PhysicalSize::new(2304, 1296));

        assert_eq!(
            options.high_performance.map(|wanted| wanted.play_sound),
            Some(played_here),
            "sound set to {sound:?} is played on this PC: {played_here}"
        );
    }
}
