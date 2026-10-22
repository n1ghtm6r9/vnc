use rfb_session::ConnectOptions;

use super::ClientApp;

impl ClientApp {
    pub(super) fn open_session(&mut self, options: ConnectOptions) {
        self.last_options = Some(options.clone());
        let proxy = self.proxy.clone();
        self.session.start(options, move |event| {
            let _ = proxy.send_event(event);
        });
    }
}
