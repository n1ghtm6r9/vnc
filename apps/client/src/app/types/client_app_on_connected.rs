use log::info;
use winit::dpi::PhysicalSize;
use winit::window::Fullscreen;

use super::{ClientApp, LinkState};
use crate::{WINDOW_TITLE, centre_on_monitor, initial_window_size, remember_password};

impl ClientApp {
    pub(super) fn on_connected(&mut self, remote: PhysicalSize<u16>, name: &str, apple_remote_desktop: bool) {
        info!(
            "[UI] connected to \"{name}\" with a {}x{} desktop, Apple Remote Desktop mode: {apple_remote_desktop}",
            remote.width, remote.height
        );
        self.router.set_apple_scroll(apple_remote_desktop);
        let first_connect = self.link == LinkState::Connecting;
        self.link = LinkState::Online;
        self.link_error = None;
        self.connect_form.busy = false;
        self.connect_form.error = None;
        if first_connect
            && self.settings.remember_password
            && let Some(options) = &self.last_options
        {
            remember_password(options);
        }
        self.update_keyboard_grab();
        self.send_clipboard();
        self.resize_remote(remote);
        if self.remote.is_none() {
            return;
        }
        let Some(window) = &self.window else {
            return;
        };
        window.set_title(if name.trim().is_empty() { WINDOW_TITLE } else { name });
        if !first_connect {
            return;
        }
        let remembered = self.settings.session_window;
        if remembered.is_some_and(|remembered| remembered.fullscreen) {
            window.set_fullscreen(Some(Fullscreen::Borderless(None)));
            return;
        }
        if remembered.is_some_and(|remembered| remembered.maximized) {
            window.set_maximized(true);
            return;
        }
        let display = self
            .last_options
            .as_ref()
            .and_then(|options| options.high_performance)
            .filter(|_| apple_remote_desktop);
        match (display, window.current_monitor()) {
            (Some(display), _) => {
                let _ = window.request_inner_size(PhysicalSize::new(display.display_width, display.display_height));
            }
            (None, Some(monitor)) => {
                let _ = window.request_inner_size(initial_window_size(monitor.size(), remote));
            }
            (None, None) => {}
        }
        centre_on_monitor(window);
    }
}
