use winit::dpi::PhysicalSize;

use crate::fit_view;

#[test]
fn test_fit_keeps_the_aspect_and_centres() {
    let view = fit_view(PhysicalSize::new(2560, 1377), PhysicalSize::new(3456, 2234)).unwrap();
    assert_eq!(view.height, 1377.0, "the taller side fills the window");
    assert_eq!(view.width, 2130.0, "the other side follows the Mac's aspect ratio");
    assert_eq!(view.x, 215.0, "the image is centred horizontally");
    assert_eq!(view.y, 0.0, "no vertical bars when the height is the limit");
    let window_aspect = view.width / view.height;
    let remote_aspect = 3456.0 / 2234.0;
    assert!((window_aspect - remote_aspect).abs() < 0.001, "aspect {window_aspect} stays at {remote_aspect}");
}
