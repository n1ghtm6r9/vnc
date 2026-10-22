use winit::platform::windows::WindowExtWindows;
use winit::window::Window;

use crate::{BIG_ICON_SIDE, SMALL_ICON_SIDE, app_icon, icon_side};

pub fn show_app_icon(window: &Window, scale: f64) {
    window.set_window_icon(app_icon(icon_side(SMALL_ICON_SIDE, scale)));
    window.set_taskbar_icon(app_icon(icon_side(BIG_ICON_SIDE, scale)));
}
