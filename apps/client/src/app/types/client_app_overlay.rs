use std::time::Instant;

use winit::window::Window;

use super::{ClientApp, LinkState};
use crate::{Overlay, ReconnectView, ToolbarView, seconds_left};

impl ClientApp {
    pub(super) fn overlay(&self, window: &Window) -> Overlay {
        let reason = self.link_error.clone().unwrap_or_default();
        match self.link {
            LinkState::Offline | LinkState::Connecting { .. } => Overlay::Form,
            LinkState::Online => Overlay::Toolbar(ToolbarView {
                scale_mode: self.settings.scale_mode,
                quality: self.settings.quality,
                sound: self.settings.sound,
                sound_streamed: self.media_picture.is_some(),
                high_performance_allowed: self.high_performance_allowed(),
                swap_control_and_command: self.settings.swap_control_and_command,
                fullscreen: window.fullscreen().is_some(),
                stats: self.stats.clone(),
            }),
            LinkState::Waiting { at, .. } => Overlay::Reconnect(ReconnectView {
                heading: self.link_heading,
                reason,
                seconds_left: Some(seconds_left(at, Instant::now())),
            }),
            LinkState::Reconnecting { .. } => Overlay::Reconnect(ReconnectView {
                heading: self.link_heading,
                reason,
                seconds_left: None,
            }),
        }
    }
}
