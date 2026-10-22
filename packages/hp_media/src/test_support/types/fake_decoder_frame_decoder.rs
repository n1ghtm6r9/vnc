use nvdec::{DecodedFrame, NvdecError};

use super::FakeDecoder;
use crate::{
    FrameDecoder,
    test_support::{FAKE_FRAME_HEIGHT, FAKE_FRAME_WIDTH, fake_frame},
};

impl FrameDecoder for FakeDecoder {
    fn decode(&mut self, access_unit: &[u8]) -> Result<Vec<DecodedFrame>, NvdecError> {
        if self.broken {
            return Err(NvdecError::Unsupported("the test decoder is broken".into()));
        }
        if self.silent {
            return Ok(Vec::new());
        }
        let value = access_unit.last().copied().unwrap_or_default();
        Ok(vec![fake_frame(FAKE_FRAME_WIDTH, FAKE_FRAME_HEIGHT, value)])
    }
}
