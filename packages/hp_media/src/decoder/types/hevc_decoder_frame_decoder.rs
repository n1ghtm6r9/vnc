use nvdec::{DecodedFrame, HevcDecoder, NvdecError};

use super::FrameDecoder;

impl FrameDecoder for HevcDecoder {
    fn decode(&mut self, access_unit: &[u8]) -> Result<Vec<DecodedFrame>, NvdecError> {
        HevcDecoder::decode(self, access_unit)
    }
}
