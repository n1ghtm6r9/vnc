use winit::event::MouseScrollDelta;

use crate::{WheelAccumulator, WheelStep};

#[test]
fn test_three_line_scroll_gives_three_up_steps() {
    let mut accumulator = WheelAccumulator::new();
    let steps = accumulator.push(MouseScrollDelta::LineDelta(0.0, 3.0));
    assert_eq!(steps, vec![WheelStep::Up; 3], "three whole lines of positive vertical scroll are three wheel-up steps");
}
