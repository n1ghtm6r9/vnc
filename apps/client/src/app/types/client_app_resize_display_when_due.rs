use std::time::Instant;

use log::info;
use rfb_session::ClientCommand;

use super::{ClientApp, LinkState};
use crate::window_display_size;

impl ClientApp {
    pub(super) fn resize_display_when_due(&mut self) {
        if self.display_resize_at.is_none_or(|at| at > Instant::now()) {
            return;
        }
        self.display_resize_at = None;
        let high_performance = self.last_options.as_ref().is_some_and(|options| options.high_performance.is_some());
        if !high_performance || self.link != LinkState::Online {
            return;
        }
        let Some(size) = self.window.as_ref().and_then(|window| window_display_size(window.inner_size())) else {
            return;
        };
        info!("[UI] the window settled at {}x{}, the Mac display follows it", size.width, size.height);
        self.session.send(ClientCommand::ResizeDisplay {
            width: size.width,
            height: size.height,
        });
    }
}
