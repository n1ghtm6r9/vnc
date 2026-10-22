use log::{info, warn};
use platform::KeyboardGrab;
use winit::event_loop::EventLoopProxy;
use winit::window::Window;

use super::native_window_handle;
use crate::AppEvent;

pub fn install_keyboard_grab(window: &Window, proxy: EventLoopProxy<AppEvent>) -> Option<KeyboardGrab> {
    let Some(handle) = native_window_handle(window) else {
        warn!("[UI] the Windows key stays with Windows: the window has no Win32 handle");
        return None;
    };
    let on_key = move |key| {
        let _ = proxy.send_event(AppEvent::GrabbedKey(key));
    };
    KeyboardGrab::install(handle, Box::new(on_key))
        .inspect(|_| info!("[UI] the Windows key goes to the Mac as Cmd while connected"))
        .inspect_err(|error| warn!("[UI] the Windows key stays with Windows: {error}"))
        .ok()
}
