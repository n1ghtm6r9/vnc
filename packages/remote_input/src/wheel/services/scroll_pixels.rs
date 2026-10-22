use winit::event::MouseScrollDelta;

use crate::{MAX_SCROLL_PIXELS, SCROLL_PIXELS_PER_NOTCH};

pub fn scroll_pixels(delta: MouseScrollDelta) -> (i32, i32) {
    let (left, up) = match delta {
        MouseScrollDelta::LineDelta(x, y) => (
            f64::from(x) * SCROLL_PIXELS_PER_NOTCH,
            f64::from(y) * SCROLL_PIXELS_PER_NOTCH,
        ),
        MouseScrollDelta::PixelDelta(position) => (position.x, position.y),
    };
    let web_pixels = |pixels: f64| (-pixels).clamp(-MAX_SCROLL_PIXELS, MAX_SCROLL_PIXELS).round() as i32;
    (web_pixels(left), web_pixels(up))
}
