use std::{ffi::c_int, thread};

use super::DecoderState;
use crate::NvdecError;

impl DecoderState {
    pub(crate) fn settle(&mut self, outcome: thread::Result<Result<c_int, NvdecError>>) -> c_int {
        let error = match outcome {
            Ok(Ok(answer)) => return answer,
            Ok(Err(error)) => error,
            Err(_) => NvdecError::CallbackPanicked,
        };
        self.error.get_or_insert(error);
        0
    }
}
