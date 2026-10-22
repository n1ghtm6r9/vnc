use crate::APPLE_SCROLL_PIXELS_PER_LINE;

pub(crate) fn apple_scroll_lines(pixels: i32) -> i16 {
    let lines = (f64::from(pixels) / APPLE_SCROLL_PIXELS_PER_LINE).round();
    let at_least_one = match pixels.signum() {
        1 => lines.max(1.0),
        -1 => lines.min(-1.0),
        _ => 0.0,
    };
    at_least_one.clamp(f64::from(i16::MIN), f64::from(i16::MAX)) as i16
}
