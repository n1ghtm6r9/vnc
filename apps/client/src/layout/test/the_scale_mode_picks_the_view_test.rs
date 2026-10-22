use winit::dpi::{PhysicalPosition, PhysicalSize};

use crate::{ScaleMode, actual_view, fit_view, view_for_mode};

#[test]
fn test_the_scale_mode_picks_the_view() {
    let window = PhysicalSize::new(2560, 1377);
    let remote = PhysicalSize::new(3456, 2234);
    let pointer = Some(PhysicalPosition::new(100.0, 100.0));
    assert_eq!(
        view_for_mode(ScaleMode::Fit, window, remote, pointer),
        fit_view(window, remote),
        "fit mode shrinks the screen into the window"
    );
    assert_eq!(
        view_for_mode(ScaleMode::Actual, window, remote, pointer),
        actual_view(window, remote, pointer),
        "1:1 mode pans with the pointer"
    );
}
