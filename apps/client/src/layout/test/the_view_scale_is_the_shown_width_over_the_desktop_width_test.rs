use remote_input::ViewRect;
use winit::dpi::PhysicalSize;

use crate::view_scale;

#[test]
fn test_the_view_scale_is_the_shown_width_over_the_desktop_width() {
    let view = ViewRect {
        x: 10.0,
        y: 0.0,
        width: 1728.0,
        height: 1117.0,
    };

    let scale = view_scale(view, PhysicalSize::new(3456, 2234));
    let empty_desktop = view_scale(view, PhysicalSize::new(0, 0));

    assert_eq!(scale, 0.5, "a Retina desktop shown at half its width has a scale of one half");
    assert!(
        empty_desktop.is_finite(),
        "a desktop without pixels still gives a usable scale, got {empty_desktop}"
    );
}
