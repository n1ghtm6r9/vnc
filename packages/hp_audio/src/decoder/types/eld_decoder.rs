use fdk_aac::dec::Decoder;

pub(crate) struct EldDecoder {
    pub(crate) decoder: Decoder,
    pub(crate) pcm: Vec<i16>,
}
