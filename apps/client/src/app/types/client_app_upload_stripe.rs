use rfb_session::StripeImage;

use super::ClientApp;
use crate::yuv_stripe;

impl ClientApp {
    pub(super) fn upload_stripe(&mut self, stripe: &StripeImage) {
        let (Some(gpu), Some(scaler), Some(window)) = (&self.gpu, &mut self.scaler, self.window.clone()) else {
            return;
        };
        scaler.upload_yuv(gpu.device(), gpu.queue(), yuv_stripe(stripe));
        self.request_frame(&window);
    }
}
