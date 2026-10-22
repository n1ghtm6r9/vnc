use gpu_scaler::SourceRect;
use rfb_session::Patch;

use super::ClientApp;

impl ClientApp {
    pub(super) fn upload_patches(&mut self, patches: &[Patch]) {
        let (Some(gpu), Some(scaler), Some(window)) = (&self.gpu, &self.scaler, self.window.clone()) else {
            return;
        };
        for patch in patches {
            let rect = SourceRect::new(
                u32::from(patch.rect.x),
                u32::from(patch.rect.y),
                u32::from(patch.rect.width),
                u32::from(patch.rect.height),
            );
            scaler.upload(gpu.queue(), rect, bytemuck::cast_slice(&patch.pixels));
        }
        self.request_frame(&window);
    }
}
