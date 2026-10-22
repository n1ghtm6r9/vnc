use nvdec::{DecodedFrame, NvdecError};

pub(crate) trait FrameDecoder {
    fn decode(&mut self, access_unit: &[u8]) -> Result<Vec<DecodedFrame>, NvdecError>;
}
