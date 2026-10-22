use std::slice;

use ard_media::{virtual_display_for, virtual_display_message};
use log::info;

use super::DisplayControl;

impl DisplayControl {
    pub(crate) fn resize(&mut self, width: u32, height: u32) {
        let display = virtual_display_for(width, height, self.scale);
        if display == self.display {
            return;
        }
        if let Some(mode) = display.modes.first() {
            info!(
                "[SESSION] asking the Mac for a virtual display of {}x{} points, {}x{} pixels",
                mode.width, mode.height, mode.pixel_width, mode.pixel_height
            );
        }
        let message = virtual_display_message(slice::from_ref(&display));
        self.peer.resize(message);
        self.display = display;
    }
}
