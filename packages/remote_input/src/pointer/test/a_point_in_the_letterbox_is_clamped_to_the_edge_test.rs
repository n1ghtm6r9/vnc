use winit::dpi::{PhysicalPosition, PhysicalSize};

use crate::{ViewRect, remote_point};

#[test]
fn test_a_point_in_the_letterbox_is_clamped_to_the_edge() {
    let view = ViewRect { x: 200.0, y: 0.0, width: 1200.0, height: 900.0 };
    let remote = PhysicalSize::new(1024, 768);
    let at = |x: f64, y: f64| remote_point(PhysicalPosition::new(x, y), view, remote);
    assert_eq!(at(50.0, 450.0), (0, 384), "the left bar clamps to the first column");
    assert_eq!(at(1550.0, 450.0), (1023, 384), "the right bar clamps to the last column");
    assert_eq!(at(800.0, -30.0), (512, 0), "above the view clamps to the first row");
    assert_eq!(at(800.0, 1200.0), (512, 767), "below the view clamps to the last row");
    assert_eq!(at(-100.0, -100.0), (0, 0), "the far top-left corner clamps to the first pixel");
    assert_eq!(at(5000.0, 5000.0), (1023, 767), "the far bottom-right corner clamps to the last pixel");
}
