#[cfg(windows)]
use windows::Win32::Foundation::{LPARAM, LRESULT, WPARAM};
#[cfg(windows)]
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, GetForegroundWindow, HC_ACTION, KBDLLHOOKSTRUCT, WM_KEYDOWN, WM_SYSKEYDOWN,
};

#[cfg(windows)]
use super::{GrabbedKey, KeyboardGrab};
#[cfg(windows)]
use crate::lock_active_grab;

#[cfg(windows)]
impl KeyboardGrab {
    pub(super) unsafe extern "system" fn hook_procedure(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        if code == HC_ACTION as i32 {
            let record = unsafe { &*(lparam.0 as *const KBDLLHOOKSTRUCT) };
            let message = wparam.0 as u32;
            let key = GrabbedKey {
                virtual_key: record.vkCode,
                down: message == WM_KEYDOWN || message == WM_SYSKEYDOWN,
            };
            let foreground = unsafe { GetForegroundWindow() }.0 as isize;
            let on_key = lock_active_grab().as_mut().and_then(|state| {
                let foreground_is_ours = foreground == state.window;
                state.decide(key, foreground_is_ours).then(|| state.on_key.clone())
            });
            if let Some(on_key) = on_key {
                on_key(key);
                return LRESULT(1);
            }
        }
        unsafe { CallNextHookEx(None, code, wparam, lparam) }
    }
}
