#[cfg(windows)]
use log::warn;
#[cfg(windows)]
use windows::Win32::Globalization::{GetUserPreferredUILanguages, MUI_LANGUAGE_NAME};
#[cfg(windows)]
use windows::core::PWSTR;

#[cfg(windows)]
use super::names_from_multi_string;

#[cfg(windows)]
pub fn preferred_ui_languages() -> Vec<String> {
    let mut count = 0;
    let mut length = 0;
    let names = unsafe { GetUserPreferredUILanguages(MUI_LANGUAGE_NAME, &mut count, None, &mut length) }.and_then(|()| {
        let mut buffer = vec![0u16; length as usize];
        let list = PWSTR(buffer.as_mut_ptr());
        unsafe { GetUserPreferredUILanguages(MUI_LANGUAGE_NAME, &mut count, Some(list), &mut length) }
            .map(|()| names_from_multi_string(&buffer))
    });
    names.unwrap_or_else(|error| {
        warn!("[PLATFORM] the Windows display languages could not be read: {error}");
        Vec::new()
    })
}

#[cfg(not(windows))]
pub fn preferred_ui_languages() -> Vec<String> {
    Vec::new()
}
