use std::io::Write;

use flate2::Compression;
use flate2::write::ZlibEncoder;

pub fn finished_zlib_stream(data: &[u8]) -> Vec<u8> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data).expect("the test data compresses");
    encoder.finish().expect("the stream finishes")
}
