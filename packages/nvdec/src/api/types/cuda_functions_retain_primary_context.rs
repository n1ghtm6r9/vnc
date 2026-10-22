use std::ptr;

use super::{CudaFunctions, PrimaryContext};
use crate::NvdecError;

impl CudaFunctions {
    pub(crate) fn retain_primary_context(&self) -> Result<PrimaryContext, NvdecError> {
        self.check("cuInit", unsafe { (self.init)(0) })?;
        let mut device = 0;
        self.check("cuDeviceGet", unsafe { (self.device_get)(&mut device, 0) })?;
        let mut context = ptr::null_mut();
        self.check("cuDevicePrimaryCtxRetain", unsafe {
            (self.device_primary_ctx_retain)(&mut context, device)
        })?;
        Ok(PrimaryContext { device, context })
    }
}
