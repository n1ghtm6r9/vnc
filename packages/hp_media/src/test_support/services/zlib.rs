use std::io::Write;

use flate2::{Compression, write::ZlibEncoder};

pub fn zlib(bytes: &[u8]) -> Vec<u8> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder
        .write_all(bytes)
        .expect("bytes are compressed into memory");
    encoder
        .finish()
        .expect("the compressed bytes are finished in memory")
}
