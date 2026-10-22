use std::io::Write;

use flate2::{Compression, write::ZlibEncoder};

pub(crate) fn zlib_deflate(bytes: &[u8]) -> Vec<u8> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder
        .write_all(bytes)
        .expect("writing into memory cannot fail");
    encoder.finish().expect("writing into memory cannot fail")
}
