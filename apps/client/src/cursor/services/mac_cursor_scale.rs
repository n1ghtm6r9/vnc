use remote_input::ViewRect;
use rfb_session::AppleDisplayGeometry;
use winit::dpi::PhysicalSize;

use crate::view_scale;

pub fn mac_cursor_scale(
    view: ViewRect,
    remote: PhysicalSize<u16>,
    display: Option<AppleDisplayGeometry>,
    streamed_display_scale: Option<u32>,
) -> f64 {
    let described = display
        .filter(|display| (display.pixel_width, display.pixel_height) == (remote.width, remote.height))
        .map(|display| f64::from(display.pixel_width) / f64::from(display.point_width.max(1)));
    let streamed = streamed_display_scale.map(|scale| f64::from(scale.max(1)));
    view_scale(view, remote) * described.or(streamed).unwrap_or(1.0)
}
