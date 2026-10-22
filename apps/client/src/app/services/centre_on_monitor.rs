use winit::window::Window;

use crate::centred_position;

pub fn centre_on_monitor(window: &Window) {
    let Some(monitor) = window.current_monitor() else {
        return;
    };
    window.set_outer_position(centred_position(monitor.position(), monitor.size(), window.outer_size()));
}
