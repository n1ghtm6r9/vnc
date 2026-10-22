use remote_input::ViewRect;
use winit::dpi::PhysicalSize;

pub fn fit_view(window: PhysicalSize<u32>, remote: PhysicalSize<u16>) -> Option<ViewRect> {
    if window.width == 0 || window.height == 0 || remote.width == 0 || remote.height == 0 {
        return None;
    }
    let window_width = f64::from(window.width);
    let window_height = f64::from(window.height);
    let scale = (window_width / f64::from(remote.width)).min(window_height / f64::from(remote.height));
    let width = (f64::from(remote.width) * scale).round().clamp(1.0, window_width);
    let height = (f64::from(remote.height) * scale).round().clamp(1.0, window_height);
    Some(ViewRect {
        x: ((window_width - width) / 2.0).floor(),
        y: ((window_height - height) / 2.0).floor(),
        width,
        height,
    })
}
