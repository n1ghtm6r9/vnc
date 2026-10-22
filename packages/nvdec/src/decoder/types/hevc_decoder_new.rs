use super::{DecoderState, HevcDecoder};
use crate::{CudaFunctions, CudaLibraries, NvdecError, create_parser};

impl HevcDecoder {
    pub fn new() -> Result<Self, NvdecError> {
        let libraries = CudaLibraries::open()?;
        let cuda = CudaFunctions::load(&libraries)?;
        let primary = cuda.retain_primary_context()?;
        let state = Box::into_raw(Box::new(DecoderState::new(cuda, primary)));
        match create_parser(&cuda, state) {
            Ok(parser) => Ok(Self {
                parser,
                state,
                _libraries: libraries,
            }),
            Err(error) => {
                drop(unsafe { Box::from_raw(state) });
                Err(error)
            }
        }
    }
}
