use gpu_scaler::ScaleFilter;
use remote_input::ViewRect;
use winit::dpi::PhysicalSize;

use crate::scale_filter_for_view;

#[test]
fn test_a_one_to_one_view_samples_the_nearest_pixel() {
    let remote = PhysicalSize::new(3456, 2234);
    let panned = ViewRect { x: -448.0, y: -397.0, width: 3456.0, height: 2234.0 };
    assert_eq!(scale_filter_for_view(panned, remote), ScaleFilter::Nearest, "a 1:1 view copies pixels without filtering");
    let fitted = ViewRect { x: 277.0, y: 0.0, width: 2005.0, height: 1296.0 };
    assert_eq!(scale_filter_for_view(fitted, remote), ScaleFilter::Smooth, "a shrunk view is filtered");
}
