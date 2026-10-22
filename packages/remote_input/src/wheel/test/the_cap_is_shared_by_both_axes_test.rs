use winit::event::MouseScrollDelta;

use crate::{WheelAccumulator, WheelStep};

#[test]
fn test_the_cap_is_shared_by_both_axes() {
    let mut accumulator = WheelAccumulator::new();
    let steps = accumulator.push(MouseScrollDelta::LineDelta(100.0, 15.0));
    let expected = [vec![WheelStep::Up; 15], vec![WheelStep::Left; 5]].concat();
    assert_eq!(steps, expected, "one push never gives more than 20 steps in total, vertical steps first");
}
