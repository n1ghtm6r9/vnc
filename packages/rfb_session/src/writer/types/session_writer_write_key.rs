use std::io;

use rfb::{apple_key_block, write_apple_encrypted_event, write_key_event};

use super::SessionWriter;

impl SessionWriter {
    pub(super) fn write_key(&mut self, keysym: u32, down: bool) -> io::Result<()> {
        let Some(key) = &self.apple_key else {
            return write_key_event(&mut self.output, down, keysym);
        };
        let timestamp = self.started.elapsed().as_micros() as u32;
        let block = key.encrypt(&apple_key_block(keysym, down, timestamp));
        write_apple_encrypted_event(&mut self.output, &block)
    }
}
