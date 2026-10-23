use super::MacInputSource;

impl MacInputSource {
    pub fn give_back(&mut self) -> Option<String> {
        self.own.take()
    }
}
