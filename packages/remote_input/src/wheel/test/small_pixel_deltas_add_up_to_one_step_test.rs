use winit::dpi::PhysicalPosition;
use winit::event::MouseScrollDelta;

use crate::{WheelAccumulator, WheelStep};

#[test]
fn test_small_pixel_deltas_add_up_to_one_step() {
    let mut accumulator = WheelAccumulator::new();
    let pixels = |y| MouseScrollDelta::PixelDelta(PhysicalPosition::new(0.0, y));
    assert!(accumulator.push(pixels(20.0)).is_empty(), "20 pixels are less than a step");
    assert!(accumulator.push(pixels(20.0)).is_empty(), "40 pixels are still less than a step");
    assert_eq!(accumulator.push(pixels(10.0)), vec![WheelStep::Up], "the third push reaches 50 pixels and releases one step");
    assert!(accumulator.push(pixels(10.0)).is_empty(), "the step used the pixels up, so the next small push starts from zero");
}
