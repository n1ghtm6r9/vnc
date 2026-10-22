use std::ptr;

use super::DecoderState;
use crate::{CudaFunctions, PlanePool, PrimaryContext};

impl DecoderState {
    pub(crate) fn new(cuda: CudaFunctions, primary: PrimaryContext) -> Self {
        Self {
            cuda,
            primary,
            decoder: ptr::null_mut(),
            format: None,
            pool: PlanePool::default(),
            frames: Vec::new(),
            error: None,
        }
    }
}
