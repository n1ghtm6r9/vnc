#[cfg(windows)]
use windows::Win32::Security::Credentials::{CRED_TYPE_GENERIC, CredDeleteW};
#[cfg(windows)]
use windows::core::PCWSTR;

#[cfg(windows)]
use super::{is_not_found, wide_string};
use crate::PlatformError;

#[cfg(windows)]
pub fn delete_password(target: &str) -> Result<(), PlatformError> {
    let target_name = wide_string(target);
    match unsafe { CredDeleteW(PCWSTR(target_name.as_ptr()), CRED_TYPE_GENERIC, None) } {
        Err(error) if is_not_found(&error) => Ok(()),
        result => result.map_err(PlatformError::from),
    }
}

#[cfg(not(windows))]
pub fn delete_password(_target: &str) -> Result<(), PlatformError> {
    Ok(())
}
