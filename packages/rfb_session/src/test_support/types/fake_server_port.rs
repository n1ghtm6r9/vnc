use super::FakeServer;

impl<T> FakeServer<T> {
    pub fn port(&self) -> u16 {
        self.port
    }
}
