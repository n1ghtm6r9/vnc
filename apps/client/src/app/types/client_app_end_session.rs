use super::{ClientApp, LinkState};
use crate::{FORM_WINDOW_SIZE, WINDOW_TITLE, centre_on_monitor};

impl ClientApp {
    pub(super) fn end_session(&mut self, message: Option<String>) {
        self.session.stop();
        self.router.release_all();
        self.restore_default_pointer();
        let was_shown = self.remote.take().is_some();
        self.media_picture = None;
        self.mac_display = None;
        self.link = LinkState::Offline;
        self.update_keyboard_grab();
        self.link_error = None;
        self.last_options = None;
        self.stats = None;
        self.connect_form.busy = false;
        self.connect_form.error = message;
        let Some(window) = &self.window else {
            return;
        };
        window.set_title(WINDOW_TITLE);
        if was_shown {
            window.set_fullscreen(None);
            let _ = window.request_inner_size(FORM_WINDOW_SIZE);
            centre_on_monitor(window);
        }
        window.request_redraw();
    }
}
