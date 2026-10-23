use log::error;
use winit::dpi::PhysicalSize;

use super::ClientApp;
use crate::{DESKTOP_TOO_LARGE_ERROR, Notice};

impl ClientApp {
    pub(super) fn size_picture(&mut self, picture: PhysicalSize<u16>) -> bool {
        let (Some(gpu), Some(scaler)) = (&self.gpu, &mut self.scaler) else {
            return false;
        };
        let size = (u32::from(picture.width), u32::from(picture.height));
        if scaler.source_size() != Some(size) {
            scaler.resize_source(gpu.device(), size.0, size.1);
        }
        if scaler.source_size().is_some() {
            return true;
        }
        error!("[GPU] a {}x{} picture does not fit into a texture", picture.width, picture.height);
        self.end_session(Some(Notice::from(DESKTOP_TOO_LARGE_ERROR)));
        false
    }
}
