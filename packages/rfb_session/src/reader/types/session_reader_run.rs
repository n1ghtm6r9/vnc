use super::SessionReader;
use crate::{SessionError, SessionEvent};

impl SessionReader {
    pub(crate) fn run(&mut self, on_event: &mut dyn FnMut(SessionEvent)) -> SessionError {
        loop {
            if let Err(error) = self.read_message(on_event) {
                return error;
            }
        }
    }
}
