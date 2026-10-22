use winit::dpi::PhysicalPosition;
use winit::event::MouseScrollDelta;

use crate::{WheelAccumulator, WheelStep};

#[test]
fn test_a_huge_delta_is_capped() {
    let mut accumulator = WheelAccumulator::default();
    let lines = accumulator.push(MouseScrollDelta::LineDelta(0.0, 1000.0));
    assert_eq!(lines, vec![WheelStep::Up; 20], "a huge line scroll is cut to 20 steps");
    assert!(accumulator.push(MouseScrollDelta::LineDelta(0.0, 0.0)).is_empty(), "the cut-off steps are dropped, not replayed on the next push");
    let pixels = accumulator.push(MouseScrollDelta::PixelDelta(PhysicalPosition::new(0.0, -1.0e9)));
    assert_eq!(pixels, vec![WheelStep::Down; 20], "a huge pixel scroll is cut to 20 steps too");
    let extreme = accumulator.push(MouseScrollDelta::PixelDelta(PhysicalPosition::new(f64::MAX, f64::MAX)));
    assert_eq!(extreme.len(), 20, "the largest finite delta still gives no more than 20 steps");
}
