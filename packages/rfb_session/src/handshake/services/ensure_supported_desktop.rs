use rfb_decode::{DecodeError, MAX_FRAMEBUFFER_PIXELS};

use crate::SessionError;

pub(crate) fn ensure_supported_desktop(width: u16, height: u16) -> Result<(), SessionError> {
    let area = usize::from(width) * usize::from(height);
    if area == 0 || area > MAX_FRAMEBUFFER_PIXELS {
        return Err(SessionError::Decode(DecodeError::Corrupt(format!(
            "a desktop of {width}x{height} is outside the supported range"
        ))));
    }
    Ok(())
}
