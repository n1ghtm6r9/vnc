use rfb::Rect;

use super::SessionReader;

impl SessionReader {
    pub(super) fn screen(&self) -> Rect {
        Rect::new(0, 0, self.framebuffer.width(), self.framebuffer.height())
    }
}
