use winit::dpi::{PhysicalPosition, PhysicalSize};

use crate::{ViewRect, remote_point};

#[test]
fn test_the_view_corners_map_to_the_remote_corners() {
    let view = ViewRect { x: 160.0, y: 20.0, width: 1280.0, height: 720.0 };
    let remote = PhysicalSize::new(2560, 1440);
    assert_eq!(remote_point(PhysicalPosition::new(160.0, 20.0), view, remote), (0, 0), "the top-left window pixel of the view is the first remote pixel");
    assert_eq!(remote_point(PhysicalPosition::new(1439.0, 739.0), view, remote), (2559, 1439), "the bottom-right window pixel of the view is the last remote pixel, so hot corners stay reachable");
    let fitted = ViewRect { x: 277.0, y: 0.0, width: 2005.0, height: 1296.0 };
    let retina = PhysicalSize::new(3456, 2234);
    assert_eq!(remote_point(PhysicalPosition::new(2281.0, 1295.0), fitted, retina), (3455, 2233), "the last pixel of a fitted Retina view reaches the last Mac pixel");
}
