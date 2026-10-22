use winit::window::{CursorIcon, Window};

use super::{ClientApp, LinkState};
use crate::RemoteCursor;

impl ClientApp {
    pub(super) fn show_pointer(&mut self, window: &Window, pointer_over_ui: bool) {
        if self.link != LinkState::Online || !self.pointer_inside {
            return;
        }
        if pointer_over_ui {
            if self.cursor_on_remote {
                self.cursor_on_remote = false;
                window.set_cursor_visible(true);
                window.set_cursor(CursorIcon::Default);
            }
            return;
        }
        self.cursor_on_remote = true;
        if self.mac_cursor_hidden {
            window.set_cursor_visible(false);
            return;
        }
        match &self.remote_cursor {
            Some(RemoteCursor::Shown(cursor)) => {
                window.set_cursor_visible(true);
                window.set_cursor(cursor.clone());
            }
            Some(RemoteCursor::System(icon)) => {
                window.set_cursor_visible(true);
                window.set_cursor(*icon);
            }
            Some(RemoteCursor::Hidden) => window.set_cursor_visible(false),
            None => window.set_cursor_visible(true),
        }
    }
}
