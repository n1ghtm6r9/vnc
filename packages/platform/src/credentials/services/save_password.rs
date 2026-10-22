#[cfg(windows)]
use windows::Win32::Security::Credentials::{CRED_PERSIST_LOCAL_MACHINE, CRED_TYPE_GENERIC, CREDENTIALW, CredWriteW};
#[cfg(windows)]
use windows::core::PWSTR;

#[cfg(windows)]
use super::wide_string;
use crate::PlatformError;

#[cfg(windows)]
pub fn save_password(target: &str, username: &str, password: &str) -> Result<(), PlatformError> {
    let mut target_name = wide_string(target);
    let mut user_name = wide_string(username);
    let mut blob: Vec<u8> = password.encode_utf16().flat_map(u16::to_le_bytes).collect();
    let blob_size = u32::try_from(blob.len()).map_err(|_| PlatformError::Os("the password is too long".to_string()))?;
    let credential = CREDENTIALW {
        Type: CRED_TYPE_GENERIC,
        TargetName: PWSTR(target_name.as_mut_ptr()),
        CredentialBlobSize: blob_size,
        CredentialBlob: blob.as_mut_ptr(),
        Persist: CRED_PERSIST_LOCAL_MACHINE,
        UserName: PWSTR(user_name.as_mut_ptr()),
        ..Default::default()
    };
    unsafe { CredWriteW(&credential, 0) }.map_err(PlatformError::from)
}

#[cfg(not(windows))]
pub fn save_password(_target: &str, _username: &str, _password: &str) -> Result<(), PlatformError> {
    Ok(())
}
