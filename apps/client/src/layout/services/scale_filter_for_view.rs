use gpu_scaler::ScaleFilter;
use remote_input::ViewRect;
use winit::dpi::PhysicalSize;

pub fn scale_filter_for_view(view: ViewRect, picture: PhysicalSize<u16>) -> ScaleFilter {
    if view.width == f64::from(picture.width) && view.height == f64::from(picture.height) {
        ScaleFilter::Nearest
    } else {
        ScaleFilter::Smooth
    }
}
