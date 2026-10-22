use rfb::Rect;

use super::MvsState;
use crate::{
    DecodeError, Framebuffer, MVS_FULL_UPDATE, MVS_PARTIAL_UPDATE, MVS_QUANTIZATION_UPDATE,
};

impl MvsState {
    pub(crate) fn decode(
        &mut self,
        data: &[u8],
        rect: Rect,
        framebuffer: &mut Framebuffer,
    ) -> Rect {
        self.fit_screen(framebuffer.width(), framebuffer.height());
        let mut changed = Rect::new(rect.x, rect.y, 0, 0);
        let decoded = match data.first().copied() {
            Some(MVS_QUANTIZATION_UPDATE) => self.set_quantization(data),
            Some(MVS_FULL_UPDATE) => {
                changed = rect;
                self.full_update(data, rect, framebuffer)
            }
            Some(MVS_PARTIAL_UPDATE) => self.partial_update(data, rect, framebuffer, &mut changed),
            Some(kind) => Err(DecodeError::Corrupt(format!(
                "the MVS update type {kind:#04x} is unknown"
            ))),
            None => Err(DecodeError::Corrupt("the MVS update is empty".to_owned())),
        };
        if let Err(error) = decoded {
            log::warn!("[DECODE] an MVS update is only partly drawn: {error}");
        }
        changed
    }
}
