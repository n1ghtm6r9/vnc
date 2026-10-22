use log::{info, warn};
use rfb_session::MediaState;

use super::ClientApp;

impl ClientApp {
    pub(super) fn on_high_performance(&mut self, state: MediaState) {
        match state {
            MediaState::Streaming { width, height, stripes } => {
                info!("[UI] High Performance shows a {width}x{height} picture in {stripes} stripes");
                self.show_media_picture(width, height);
            }
            MediaState::Failed(failure) => {
                warn!("[UI] High Performance ended: {failure}");
                self.media_picture = None;
                if failure.needs_reconnect() {
                    self.reconnect_without_high_performance();
                }
            }
        }
    }
}
