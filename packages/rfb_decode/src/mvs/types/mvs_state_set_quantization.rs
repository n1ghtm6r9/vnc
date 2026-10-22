use super::MvsState;
use crate::{DecodeError, MVS_QUANTIZATION_BYTES};

impl MvsState {
    pub(super) fn set_quantization(&mut self, data: &[u8]) -> Result<(), DecodeError> {
        if data.len() != MVS_QUANTIZATION_BYTES {
            return Err(DecodeError::Corrupt(format!(
                "the MVS quantization update has {} bytes where {MVS_QUANTIZATION_BYTES} belong",
                data.len()
            )));
        }
        self.luma_quantization.copy_from_slice(&data[1..65]);
        self.chroma_quantization.copy_from_slice(&data[65..]);
        self.quantization_generation = self.quantization_generation.wrapping_add(1).max(1);
        Ok(())
    }
}
