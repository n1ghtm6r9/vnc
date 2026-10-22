use flate2::write::ZlibEncoder;

pub struct ZlibFixture {
    pub(super) encoder: ZlibEncoder<Vec<u8>>,
}
