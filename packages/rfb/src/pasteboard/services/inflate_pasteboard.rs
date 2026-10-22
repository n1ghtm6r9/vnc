use flate2::{Decompress, FlushDecompress};

pub(crate) fn inflate_pasteboard(compressed: &[u8], length: usize) -> Option<Vec<u8>> {
    let mut pasteboard = Vec::with_capacity(length);
    Decompress::new(true)
        .decompress_vec(compressed, &mut pasteboard, FlushDecompress::Sync)
        .ok()?;
    Some(pasteboard)
}
