use winit::event::MouseScrollDelta;

use super::{WheelAccumulator, WheelStep};
use crate::{MAX_WHEEL_STEPS_PER_PUSH, PIXELS_PER_WHEEL_STEP, axis_steps, take_whole_steps};

impl WheelAccumulator {
    pub fn push(&mut self, delta: MouseScrollDelta) -> Vec<WheelStep> {
        let (horizontal, vertical) = match delta {
            MouseScrollDelta::LineDelta(x, y) => (
                f64::from(x) * PIXELS_PER_WHEEL_STEP,
                f64::from(y) * PIXELS_PER_WHEEL_STEP,
            ),
            MouseScrollDelta::PixelDelta(position) => (position.x, position.y),
        };
        let vertical_steps = axis_steps(
            take_whole_steps(&mut self.vertical, vertical),
            WheelStep::Up,
            WheelStep::Down,
        );
        let horizontal_steps = axis_steps(
            take_whole_steps(&mut self.horizontal, horizontal),
            WheelStep::Left,
            WheelStep::Right,
        );
        vertical_steps
            .chain(horizontal_steps)
            .take(MAX_WHEEL_STEPS_PER_PUSH)
            .collect()
    }
}
