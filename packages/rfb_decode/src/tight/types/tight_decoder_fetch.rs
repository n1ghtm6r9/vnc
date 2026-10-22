use std::io::Read;

use super::TightDecoder;
use crate::{
    DecodeError, TIGHT_MIN_COMPRESSED, inflate_payload, read_compact_length,
    read_compressed_payload, read_scratch,
};

impl<R: Read> TightDecoder<'_, R> {
    pub(super) fn fetch(&mut self, stream: usize, expected: usize) -> Result<(), DecodeError> {
        if expected < TIGHT_MIN_COMPRESSED {
            read_scratch(self.reader, &mut self.state.inflated, expected)?;
            return Ok(());
        }
        let length = read_compact_length(self.reader)?;
        read_compressed_payload(self.reader, &mut self.state.compressed, length, expected)?;
        let zlib = self.state.tight_streams.get_mut(stream).ok_or_else(|| {
            DecodeError::Corrupt(format!("the Tight stream {stream} does not exist"))
        })?;
        let produced = inflate_payload(
            zlib,
            &self.state.compressed,
            &mut self.state.inflated,
            expected,
        )?;
        if produced != expected {
            return Err(DecodeError::Corrupt(format!(
                "the Tight data inflates to {produced} bytes where the rect needs {expected}"
            )));
        }
        Ok(())
    }
}
