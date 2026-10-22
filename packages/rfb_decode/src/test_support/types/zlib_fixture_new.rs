use flate2::Compression;
use flate2::write::ZlibEncoder;

use super::ZlibFixture;

impl ZlibFixture {
    pub fn new() -> ZlibFixture {
        ZlibFixture {
            encoder: ZlibEncoder::new(Vec::new(), Compression::default()),
        }
    }
}
