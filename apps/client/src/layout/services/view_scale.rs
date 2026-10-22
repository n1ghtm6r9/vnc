use remote_input::ViewRect;
use winit::dpi::PhysicalSize;

pub fn view_scale(view: ViewRect, remote: PhysicalSize<u16>) -> f64 {
    view.width / f64::from(remote.width.max(1))
}
