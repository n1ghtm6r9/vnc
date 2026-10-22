use super::SessionSlot;

impl SessionSlot {
    pub fn is_current(&self, generation: u64) -> bool {
        self.handle.is_some() && self.generation == generation
    }
}
