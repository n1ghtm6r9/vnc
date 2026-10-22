use winit::dpi::PhysicalSize;

use crate::SessionWindow;

pub fn remembered_window(size: PhysicalSize<u32>, maximized: bool, fullscreen: bool) -> Option<SessionWindow> {
    (size.width > 0 && size.height > 0).then_some(SessionWindow {
        width: size.width,
        height: size.height,
        maximized,
        fullscreen,
    })
}
