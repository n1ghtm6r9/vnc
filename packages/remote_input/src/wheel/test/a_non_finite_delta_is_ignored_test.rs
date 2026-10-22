use winit::dpi::PhysicalPosition;
use winit::event::MouseScrollDelta;

use crate::{WheelAccumulator, WheelStep};

#[test]
fn test_a_non_finite_delta_is_ignored() {
    let mut accumulator = WheelAccumulator::new();
    assert!(accumulator.push(MouseScrollDelta::LineDelta(f32::NAN, f32::INFINITY)).is_empty(), "a line delta that is not finite gives no steps");
    let broken = MouseScrollDelta::PixelDelta(PhysicalPosition::new(f64::NAN, f64::NEG_INFINITY));
    assert!(accumulator.push(broken).is_empty(), "a pixel delta that is not finite gives no steps");
    assert_eq!(accumulator.push(MouseScrollDelta::LineDelta(0.0, 1.0)), vec![WheelStep::Up], "the accumulator is not poisoned by the broken deltas");
}
