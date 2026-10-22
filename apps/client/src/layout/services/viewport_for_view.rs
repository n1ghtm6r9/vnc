use gpu_scaler::Viewport;
use remote_input::ViewRect;

pub fn viewport_for_view(view: ViewRect) -> Viewport {
    Viewport {
        x: view.x.round() as i32,
        y: view.y.round() as i32,
        width: view.width.round().max(0.0) as u32,
        height: view.height.round().max(0.0) as u32,
    }
}
