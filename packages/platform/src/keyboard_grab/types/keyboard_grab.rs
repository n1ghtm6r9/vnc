#[cfg(windows)]
use log::warn;
#[cfg(windows)]
use windows::Win32::UI::WindowsAndMessaging::{HHOOK, UnhookWindowsHookEx};

use crate::lock_active_grab;

pub struct KeyboardGrab {
    #[cfg_attr(not(windows), allow(dead_code))]
    pub(super) hook: isize,
}

impl Drop for KeyboardGrab {
    fn drop(&mut self) {
        #[cfg(windows)]
        if let Err(error) = unsafe { UnhookWindowsHookEx(HHOOK(self.hook as *mut core::ffi::c_void)) } {
            warn!("[PLATFORM] the keyboard hook could not be removed: {error}");
        }
        *lock_active_grab() = None;
    }
}
