use winit::event::MouseScrollDelta;

use crate::{WheelAccumulator, WheelStep};

#[test]
fn test_fractional_line_deltas_are_carried_to_the_next_push() {
    let mut accumulator = WheelAccumulator::new();
    assert!(accumulator.push(MouseScrollDelta::LineDelta(0.0, 0.5)).is_empty(), "half a line is not yet a step");
    assert_eq!(accumulator.push(MouseScrollDelta::LineDelta(0.0, 0.5)), vec![WheelStep::Up], "two halves make a line");
    assert_eq!(accumulator.push(MouseScrollDelta::LineDelta(0.0, 1.5)), vec![WheelStep::Up], "a line and a half gives one step and keeps half a line");
    assert_eq!(accumulator.push(MouseScrollDelta::LineDelta(0.0, 0.5)), vec![WheelStep::Up], "the carried half completes the next line");
}
