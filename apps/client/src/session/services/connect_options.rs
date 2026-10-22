use rfb_session::{ConnectOptions, Credentials, HighPerformanceOptions, encodings_for_quality};
use winit::dpi::PhysicalSize;

use crate::{
    CONNECT_TIMEOUT, ConnectRequest, ImageQuality, RETINA_DISPLAY_SCALE, STANDARD_DISPLAY_SCALE, SoundPlace,
    session_quality,
};

pub fn connect_options(
    request: &ConnectRequest,
    quality: ImageQuality,
    sound: SoundPlace,
    display: PhysicalSize<u32>,
) -> ConnectOptions {
    let high_performance = request.apple_remote_desktop && request.high_performance;
    ConnectOptions {
        host: request.host.clone(),
        port: request.port,
        credentials: Credentials::new(request.username.as_str(), request.password.as_str()),
        shared: true,
        encodings: encodings_for_quality(session_quality(quality)),
        connect_timeout: CONNECT_TIMEOUT,
        apple_remote_desktop: request.apple_remote_desktop,
        high_performance: high_performance.then_some(HighPerformanceOptions {
            display_width: display.width,
            display_height: display.height,
            display_scale: if request.retina { RETINA_DISPLAY_SCALE } else { STANDARD_DISPLAY_SCALE },
            play_sound: sound == SoundPlace::Here,
        }),
    }
}
