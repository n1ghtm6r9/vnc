use nvdec::NvdecError;

use crate::test_support::FakeDecoder;

pub fn new_fake_decoder() -> Result<FakeDecoder, NvdecError> {
    Ok(FakeDecoder::default())
}
