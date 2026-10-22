use super::{DecodedFrame, StreamFormat};
use crate::{CuVideoDecoder, CudaFunctions, NvdecError, PlanePool, PrimaryContext};

pub(crate) struct DecoderState {
    pub(crate) cuda: CudaFunctions,
    pub(crate) primary: PrimaryContext,
    pub(crate) decoder: CuVideoDecoder,
    pub(crate) format: Option<StreamFormat>,
    pub(crate) pool: PlanePool,
    pub(crate) frames: Vec<DecodedFrame>,
    pub(crate) error: Option<NvdecError>,
}
