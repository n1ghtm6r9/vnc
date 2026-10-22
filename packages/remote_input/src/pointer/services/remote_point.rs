use winit::dpi::{PhysicalPosition, PhysicalSize};

use super::remote_coordinate;
use crate::ViewRect;

pub fn remote_point(position: PhysicalPosition<f64>, view: ViewRect, remote: PhysicalSize<u16>) -> (u16, u16) {
    (
        remote_coordinate(position.x, view.x, view.width, remote.width),
        remote_coordinate(position.y, view.y, view.height, remote.height),
    )
}
