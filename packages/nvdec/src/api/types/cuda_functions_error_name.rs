use std::{ffi::CStr, ptr};

use super::CudaFunctions;
use crate::{CUDA_SUCCESS, CuResult};

impl CudaFunctions {
    pub(crate) fn error_name(&self, code: CuResult) -> String {
        let mut name = ptr::null();
        let found = unsafe { (self.get_error_name)(code, &mut name) };
        if found != CUDA_SUCCESS || name.is_null() {
            return String::from("an unknown CUDA error");
        }
        unsafe { CStr::from_ptr(name) }
            .to_string_lossy()
            .into_owned()
    }
}
