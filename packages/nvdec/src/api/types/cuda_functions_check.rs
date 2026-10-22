use super::CudaFunctions;
use crate::{CUDA_SUCCESS, CuResult, NvdecError};

impl CudaFunctions {
    pub(crate) fn check(&self, call: &'static str, code: CuResult) -> Result<(), NvdecError> {
        if code == CUDA_SUCCESS {
            return Ok(());
        }
        Err(NvdecError::Cuda {
            call,
            code,
            name: self.error_name(code),
        })
    }
}
