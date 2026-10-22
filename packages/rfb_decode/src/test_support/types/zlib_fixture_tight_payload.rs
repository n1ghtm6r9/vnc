use super::ZlibFixture;
use crate::test_support::compact_length;

impl ZlibFixture {
    pub fn tight_payload(&mut self, data: &[u8]) -> Vec<u8> {
        let compressed = self.compress(data);
        let mut payload = compact_length(compressed.len());
        payload.extend_from_slice(&compressed);
        payload
    }
}
