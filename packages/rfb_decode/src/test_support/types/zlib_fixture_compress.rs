use std::io::Write;

use super::ZlibFixture;

impl ZlibFixture {
    pub fn compress(&mut self, data: &[u8]) -> Vec<u8> {
        self.encoder
            .write_all(data)
            .expect("the test data compresses");
        self.encoder.flush().expect("the sync flush completes");
        std::mem::take(self.encoder.get_mut())
    }
}
