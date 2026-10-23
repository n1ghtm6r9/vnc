use log::info;
use rfb_session::HighPerformanceOptions;

use super::ClientApp;
use crate::{MODE_SWITCH_HEADING, Notice, Phrase};

impl ClientApp {
    pub(super) fn reconnect_for_sound(&mut self, high_performance: Option<HighPerformanceOptions>, status: Phrase) {
        let Some(options) = &mut self.last_options else {
            return;
        };
        let mode = if high_performance.is_some() { "High Performance" } else { "standard mode" };
        info!("[SESSION] the sound moves, reconnecting to {}:{} in {mode}", options.host, options.port);
        options.high_performance = high_performance;
        self.wait_to_reconnect(0, Some(Notice::from(status)), MODE_SWITCH_HEADING);
    }
}
