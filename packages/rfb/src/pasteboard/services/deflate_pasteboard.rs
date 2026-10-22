use std::io;

use flate2::{Compress, Compression, FlushCompress};

pub(crate) fn deflate_pasteboard(pasteboard: &[u8]) -> io::Result<Vec<u8>> {
    let mut compressor = Compress::new(Compression::default(), true);
    let mut compressed = Vec::with_capacity(pasteboard.len() / 2 + 64);
    loop {
        let consumed = usize::try_from(compressor.total_in()).unwrap_or(usize::MAX);
        compressor
            .compress_vec(&pasteboard[consumed.min(pasteboard.len())..], &mut compressed, FlushCompress::Full)
            .map_err(io::Error::other)?;
        let all_in = usize::try_from(compressor.total_in()).is_ok_and(|total| total >= pasteboard.len());
        if all_in && compressed.len() < compressed.capacity() {
            return Ok(compressed);
        }
        compressed.reserve(compressed.capacity().max(64));
    }
}
