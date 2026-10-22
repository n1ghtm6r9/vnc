use std::sync::mpsc::Receiver;

pub struct FakeServer<T> {
    pub(super) port: u16,
    pub(super) outcome: Receiver<T>,
}
