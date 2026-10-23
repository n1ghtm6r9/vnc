use log::warn;

use super::ClientApp;
use crate::{HIGH_PERFORMANCE_FAILED_ERROR, LINK_LOST_HEADING};

impl ClientApp {
    pub(super) fn reconnect_without_high_performance(&mut self) {
        let Some(options) = &mut self.last_options else {
            return;
        };
        options.high_performance = None;
        warn!("[SESSION] reconnecting to {}:{} without High Performance", options.host, options.port);
        self.wait_to_reconnect(0, Some(HIGH_PERFORMANCE_FAILED_ERROR.to_owned()), LINK_LOST_HEADING);
    }
}
