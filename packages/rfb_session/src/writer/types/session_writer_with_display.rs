use super::SessionWriter;
use crate::DisplayControl;

impl SessionWriter {
    pub(crate) fn with_display(mut self, display: Option<DisplayControl>) -> SessionWriter {
        self.display = display;
        self
    }
}
