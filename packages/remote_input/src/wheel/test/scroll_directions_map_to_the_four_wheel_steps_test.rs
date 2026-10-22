use winit::dpi::PhysicalPosition;
use winit::event::MouseScrollDelta;

use crate::{WheelAccumulator, WheelStep};

#[test]
fn test_scroll_directions_map_to_the_four_wheel_steps() {
    let mut accumulator = WheelAccumulator::new();
    assert_eq!(accumulator.push(MouseScrollDelta::LineDelta(0.0, 1.0)), vec![WheelStep::Up], "positive vertical lines scroll up");
    assert_eq!(accumulator.push(MouseScrollDelta::LineDelta(0.0, -2.0)), vec![WheelStep::Down; 2], "negative vertical lines scroll down");
    assert_eq!(accumulator.push(MouseScrollDelta::LineDelta(1.0, 0.0)), vec![WheelStep::Left], "positive horizontal lines scroll left");
    assert_eq!(accumulator.push(MouseScrollDelta::LineDelta(-2.0, 0.0)), vec![WheelStep::Right; 2], "negative horizontal lines scroll right");
    let pixels = |x, y| MouseScrollDelta::PixelDelta(PhysicalPosition::new(x, y));
    assert_eq!(accumulator.push(pixels(0.0, 50.0)), vec![WheelStep::Up], "positive vertical pixels scroll up");
    assert_eq!(accumulator.push(pixels(0.0, -50.0)), vec![WheelStep::Down], "negative vertical pixels scroll down");
    assert_eq!(accumulator.push(pixels(50.0, 0.0)), vec![WheelStep::Left], "positive horizontal pixels scroll left");
    assert_eq!(accumulator.push(pixels(-50.0, 0.0)), vec![WheelStep::Right], "negative horizontal pixels scroll right");
}
