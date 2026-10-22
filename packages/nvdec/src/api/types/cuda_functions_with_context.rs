use std::ptr;

use super::CudaFunctions;
use crate::{CuContext, NvdecError};

impl CudaFunctions {
    pub(crate) fn with_context<T>(
        &self,
        context: CuContext,
        work: impl FnOnce() -> Result<T, NvdecError>,
    ) -> Result<T, NvdecError> {
        self.check("cuCtxPushCurrent", unsafe {
            (self.ctx_push_current)(context)
        })?;
        let outcome = work();
        let mut popped = ptr::null_mut();
        let popped_cleanly = self.check("cuCtxPopCurrent", unsafe {
            (self.ctx_pop_current)(&mut popped)
        });
        let value = outcome?;
        popped_cleanly?;
        Ok(value)
    }
}
