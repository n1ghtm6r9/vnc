#[cfg(windows)]
use windows::Win32::Foundation::ERROR_NOT_FOUND;
#[cfg(windows)]
use windows::core::HRESULT;

#[cfg(windows)]
pub(crate) fn is_not_found(error: &windows::core::Error) -> bool {
    error.code() == HRESULT::from_win32(ERROR_NOT_FOUND.0)
}
