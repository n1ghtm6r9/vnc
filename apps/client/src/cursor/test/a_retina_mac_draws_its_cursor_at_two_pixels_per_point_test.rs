use remote_input::ViewRect;
use rfb_session::AppleDisplayGeometry;
use winit::dpi::PhysicalSize;

use crate::mac_cursor_scale;

#[test]
fn test_a_retina_mac_draws_its_cursor_at_two_pixels_per_point() {
    let view = ViewRect {
        x: 0.0,
        y: 0.0,
        width: 2268.0,
        height: 1473.0,
    };
    let display = AppleDisplayGeometry {
        pixel_width: 3024,
        pixel_height: 1964,
        point_width: 1512,
        point_height: 982,
    };

    let scale = mac_cursor_scale(view, PhysicalSize::new(3024, 1964), Some(display), None);

    assert_eq!(
        scale, 1.5,
        "the Mac sends its cursor in points, so a desktop of two pixels per point shown at three quarters draws it at 1.5"
    );
}
