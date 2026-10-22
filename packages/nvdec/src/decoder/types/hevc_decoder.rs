use super::DecoderState;
use crate::{CuVideoParser, CudaLibraries};

pub struct HevcDecoder {
    pub(crate) parser: CuVideoParser,
    pub(crate) state: *mut DecoderState,
    pub(crate) _libraries: CudaLibraries,
}
