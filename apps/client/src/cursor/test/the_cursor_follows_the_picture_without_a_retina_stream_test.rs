use remote_input::ViewRect;
use winit::dpi::PhysicalSize;

use crate::mac_cursor_scale;

#[test]
fn test_the_cursor_follows_the_picture_without_a_retina_stream() {
    let view = ViewRect {
        x: 0.0,
        y: 0.0,
        width: 2880.0,
        height: 1620.0,
    };
    let remote = PhysicalSize::new(3840, 2160);

    assert_eq!(
        mac_cursor_scale(view, remote, None, None),
        0.75,
        "outside High Performance the Mac sends its cursor in desktop pixels, so it follows the picture"
    );
    assert_eq!(
        mac_cursor_scale(view, remote, None, Some(1)),
        0.75,
        "a stream of one pixel per point keeps the picture scale"
    );
    assert_eq!(
        mac_cursor_scale(view, remote, None, Some(0)),
        0.75,
        "a zero display scale counts as one pixel per point"
    );
}
