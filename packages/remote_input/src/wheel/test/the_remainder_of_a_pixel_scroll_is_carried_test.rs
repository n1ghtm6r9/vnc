use winit::dpi::PhysicalPosition;
use winit::event::MouseScrollDelta;

use crate::{WheelAccumulator, WheelStep};

#[test]
fn test_the_remainder_of_a_pixel_scroll_is_carried() {
    let mut accumulator = WheelAccumulator::new();
    let pixels = |y| MouseScrollDelta::PixelDelta(PhysicalPosition::new(0.0, y));
    assert_eq!(accumulator.push(pixels(75.0)), vec![WheelStep::Up], "75 pixels give one step");
    assert_eq!(accumulator.push(pixels(25.0)), vec![WheelStep::Up], "the 25 carried pixels and 25 new ones give the next step");
    assert!(accumulator.push(pixels(49.0)).is_empty(), "49 pixels are one short of a step");
    assert_eq!(accumulator.push(pixels(1.0)), vec![WheelStep::Up], "the missing pixel completes the step");
    assert_eq!(accumulator.push(pixels(-75.0)), vec![WheelStep::Down], "scrolling the other way carries its remainder the same way");
    assert_eq!(accumulator.push(pixels(-25.0)), vec![WheelStep::Down], "the carried negative pixels complete the next step");
}
