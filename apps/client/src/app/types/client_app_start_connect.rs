use log::info;

use super::{ClientApp, LinkState};
use crate::{ConnectRequest, connect_options, forget_password};

impl ClientApp {
    pub(super) fn start_connect(&mut self, request: ConnectRequest) {
        info!("[UI] connecting to {}:{} as {}", request.host, request.port, request.username);
        if !request.remember_password {
            forget_password(&request);
        }
        let options = connect_options(&request, self.settings.quality, self.settings.sound, self.high_performance_display());
        self.settings.host = request.host;
        self.settings.port = request.port;
        self.settings.username = request.username;
        self.settings.remember_password = request.remember_password;
        self.settings.apple_remote_desktop = request.apple_remote_desktop;
        self.settings.high_performance = request.high_performance;
        self.settings.retina = request.retina;
        self.save_settings();
        self.link = LinkState::Connecting { attempt: 0 };
        self.connect_form.busy = true;
        self.open_session(options);
    }
}
