use winit::dpi::PhysicalPosition;
use winit::event::MouseScrollDelta;

use crate::scroll_pixels;

#[test]
fn test_scroll_pixels_follow_the_web_convention() {
    let cases = [
        (MouseScrollDelta::LineDelta(0.0, -1.0), (0, 100), "a notch toward the user scrolls down by 100 pixels"),
        (MouseScrollDelta::LineDelta(0.0, 2.0), (0, -200), "two notches away from the user scroll up by 200 pixels"),
        (MouseScrollDelta::LineDelta(-0.25, 0.0), (25, 0), "a quarter notch of tilt to the right scrolls right by 25 pixels"),
        (MouseScrollDelta::PixelDelta(PhysicalPosition::new(3.4, -12.6)), (-3, 13), "touchpad pixels keep their size"),
        (MouseScrollDelta::LineDelta(0.0, -1000.0), (0, 2000), "a huge delta is capped at twenty notches"),
        (MouseScrollDelta::LineDelta(f32::NAN, f32::INFINITY), (0, -2000), "a broken delta never becomes a wild scroll"),
    ];
    for (delta, expected, meaning) in cases {
        assert_eq!(scroll_pixels(delta), expected, "{meaning}");
    }
}
