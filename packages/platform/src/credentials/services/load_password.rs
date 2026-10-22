#[cfg(windows)]
use windows::Win32::Security::Credentials::{CRED_TYPE_GENERIC, CREDENTIALW, CredFree, CredReadW};
#[cfg(windows)]
use windows::core::PCWSTR;

#[cfg(windows)]
use super::{is_not_found, password_from_blob, wide_string};
use crate::PlatformError;

#[cfg(windows)]
pub fn load_password(target: &str) -> Result<Option<String>, PlatformError> {
    let target_name = wide_string(target);
    let mut credential: *mut CREDENTIALW = std::ptr::null_mut();
    match unsafe { CredReadW(PCWSTR(target_name.as_ptr()), CRED_TYPE_GENERIC, None, &mut credential) } {
        Err(error) if is_not_found(&error) => return Ok(None),
        Err(error) => return Err(error.into()),
        Ok(()) => {}
    }
    let password = unsafe {
        let record = &*credential;
        let blob = if record.CredentialBlob.is_null() {
            &[][..]
        } else {
            std::slice::from_raw_parts(record.CredentialBlob, record.CredentialBlobSize as usize)
        };
        password_from_blob(blob)
    };
    unsafe { CredFree(credential.cast()) };
    Ok(Some(password))
}

#[cfg(not(windows))]
pub fn load_password(_target: &str) -> Result<Option<String>, PlatformError> {
    Ok(None)
}
