use super::{Control, TestServer};
use crate::Datagram;

impl TestServer {
    pub fn open_all(&self, datagrams: &[Datagram]) -> Vec<Control> {
        datagrams
            .iter()
            .map(|datagram| self.open(datagram))
            .collect()
    }
}
