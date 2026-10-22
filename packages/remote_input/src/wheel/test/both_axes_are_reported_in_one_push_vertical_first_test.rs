use winit::event::MouseScrollDelta;

use crate::{WheelAccumulator, WheelStep};

#[test]
fn test_both_axes_are_reported_in_one_push_vertical_first() {
    let mut accumulator = WheelAccumulator::new();
    assert_eq!(accumulator.push(MouseScrollDelta::LineDelta(1.0, 2.0)), vec![WheelStep::Up, WheelStep::Up, WheelStep::Left], "a diagonal scroll reports the vertical steps and then the horizontal ones");
    assert_eq!(accumulator.push(MouseScrollDelta::LineDelta(-1.0, -1.0)), vec![WheelStep::Down, WheelStep::Right], "a diagonal scroll the other way reports down and then right");
}
