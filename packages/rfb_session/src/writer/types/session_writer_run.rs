use std::net::Shutdown;

use log::warn;

use super::SessionWriter;

impl SessionWriter {
    pub(crate) fn run(mut self) {
        if let Err(error) = self.serve() {
            warn!("[SESSION] the writer stopped, the session ends: {error}");
            let _ = self.output.get_ref().shutdown(Shutdown::Both);
        }
    }
}
