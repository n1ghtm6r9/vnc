use libloading::Library;

use super::CudaLibraries;
use crate::{CUDA_LIBRARY, CUVID_LIBRARY, NvdecError};

impl CudaLibraries {
    pub(crate) fn open() -> Result<Self, NvdecError> {
        let cuda = unsafe { Library::new(CUDA_LIBRARY) }.map_err(NvdecError::Load)?;
        let cuvid = unsafe { Library::new(CUVID_LIBRARY) }.map_err(NvdecError::Load)?;
        Ok(Self { cuda, cuvid })
    }
}
