use rfb_session::ConnectOptions;
use winit::dpi::PhysicalSize;

use crate::window_display_size;

pub fn fit_display_to_window(options: &mut ConnectOptions, window: PhysicalSize<u32>) {
    let (Some(high_performance), Some(size)) = (&mut options.high_performance, window_display_size(window)) else {
        return;
    };
    high_performance.display_width = size.width;
    high_performance.display_height = size.height;
}
