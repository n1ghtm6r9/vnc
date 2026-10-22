use super::ZlibFixture;
use crate::test_support::length_prefixed;

impl ZlibFixture {
    pub fn payload(&mut self, data: &[u8]) -> Vec<u8> {
        length_prefixed(&self.compress(data))
    }
}
