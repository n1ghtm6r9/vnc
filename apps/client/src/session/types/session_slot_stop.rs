use super::SessionSlot;

impl SessionSlot {
    pub fn stop(&mut self) {
        self.handle = None;
        self.generation += 1;
    }
}
