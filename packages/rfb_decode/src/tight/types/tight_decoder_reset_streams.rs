use std::io::Read;

use super::TightDecoder;

impl<R: Read> TightDecoder<'_, R> {
    pub(super) fn reset_streams(&mut self, mask: u8) {
        for (index, stream) in self.state.tight_streams.iter_mut().enumerate() {
            if mask & (1 << index) != 0 {
                stream.reset(true);
            }
        }
    }
}
