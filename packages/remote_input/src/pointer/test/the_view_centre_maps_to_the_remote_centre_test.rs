use winit::dpi::{PhysicalPosition, PhysicalSize};

use crate::{ViewRect, remote_point};

#[test]
fn test_the_view_centre_maps_to_the_remote_centre() {
    let retina = ViewRect { x: 100.0, y: 50.0, width: 1728.0, height: 1117.0 };
    let (x, y) = remote_point(PhysicalPosition::new(964.0, 608.0), retina, PhysicalSize::new(3456, 2234));
    assert!(x.abs_diff(1728) <= 1 && y.abs_diff(1117) <= 1, "the middle of a half-size view lands within a pixel of the middle of a 3456x2234 screen, got ({x}, {y})");
    let letterboxed = ViewRect { x: 160.0, y: 0.0, width: 1280.0, height: 720.0 };
    let (x, y) = remote_point(PhysicalPosition::new(800.0, 360.0), letterboxed, PhysicalSize::new(2560, 1440));
    assert!(x.abs_diff(1280) <= 1 && y.abs_diff(720) <= 1, "the middle of a letterboxed view lands within a pixel of the middle of a 2560x1440 screen, got ({x}, {y})");
}
