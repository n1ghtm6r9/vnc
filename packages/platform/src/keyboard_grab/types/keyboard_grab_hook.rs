#[cfg(windows)]
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
#[cfg(windows)]
use windows::Win32::UI::WindowsAndMessaging::{SetWindowsHookExW, WH_KEYBOARD_LL};
#[cfg(windows)]
use windows::core::PCWSTR;

use super::KeyboardGrab;
use crate::PlatformError;

impl KeyboardGrab {
    #[cfg(windows)]
    pub(super) fn hook() -> Result<isize, PlatformError> {
        let module = unsafe { GetModuleHandleW(PCWSTR::null()) }?;
        let hook = unsafe { SetWindowsHookExW(WH_KEYBOARD_LL, Some(KeyboardGrab::hook_procedure), Some(module.into()), 0) }?;
        Ok(hook.0 as isize)
    }

    #[cfg(not(windows))]
    pub(super) fn hook() -> Result<isize, PlatformError> {
        Ok(0)
    }
}
