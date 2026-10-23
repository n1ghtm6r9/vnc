use rfb_session::{ConnectOptions, Credentials, encodings_for_quality};
use winit::dpi::PhysicalSize;

use crate::{CONNECT_TIMEOUT, ConnectRequest, ImageQuality, SoundPlace, high_performance_options, session_quality};

pub fn connect_options(
    request: &ConnectRequest,
    quality: ImageQuality,
    sound: SoundPlace,
    display: PhysicalSize<u32>,
) -> ConnectOptions {
    let high_performance = request.apple_remote_desktop && request.high_performance && sound != SoundPlace::Mac;
    ConnectOptions {
        host: request.host.clone(),
        port: request.port,
        credentials: Credentials::new(request.username.as_str(), request.password.as_str()),
        shared: true,
        encodings: encodings_for_quality(session_quality(quality)),
        connect_timeout: CONNECT_TIMEOUT,
        apple_remote_desktop: request.apple_remote_desktop,
        high_performance: high_performance.then(|| high_performance_options(display, request.retina, sound)),
    }
}
