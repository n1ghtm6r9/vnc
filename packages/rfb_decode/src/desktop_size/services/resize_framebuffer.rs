use crate::{DecodeError, Framebuffer, MAX_FRAMEBUFFER_PIXELS};

pub(crate) fn resize_framebuffer(
    framebuffer: &mut Framebuffer,
    width: u16,
    height: u16,
) -> Result<(), DecodeError> {
    let area = usize::from(width) * usize::from(height);
    if area == 0 || area > MAX_FRAMEBUFFER_PIXELS {
        return Err(DecodeError::Corrupt(format!(
            "a desktop of {width}x{height} is outside the supported range"
        )));
    }
    if (framebuffer.width(), framebuffer.height()) != (width, height) {
        log::debug!(
            "[DECODE] the desktop changes from {}x{} to {width}x{height}",
            framebuffer.width(),
            framebuffer.height()
        );
    }
    framebuffer.resize(width, height);
    Ok(())
}
