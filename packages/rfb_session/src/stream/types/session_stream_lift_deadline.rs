use super::SessionStream;

impl SessionStream {
    pub(crate) fn lift_deadline(&mut self) {
        self.deadline = None;
    }
}
