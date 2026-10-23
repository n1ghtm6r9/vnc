use super::SessionReader;
use crate::SessionEvent;

impl SessionReader {
    pub(super) fn report_input_source(&mut self, source: String, on_event: &mut dyn FnMut(SessionEvent)) {
        if self.input_source.as_ref() == Some(&source) {
            return;
        }
        self.input_source = Some(source.clone());
        on_event(SessionEvent::InputSource(source));
    }
}
