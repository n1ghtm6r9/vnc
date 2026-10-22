use super::{DecoderState, StreamFormat};
use crate::{CODEC_HEVC, CuvidDecodeCaps, NvdecError, cuda_chroma_format, cuda_surface_format};

impl DecoderState {
    pub(crate) fn check_capabilities(&self, format: &StreamFormat) -> Result<(), NvdecError> {
        let mut caps = CuvidDecodeCaps::new(CODEC_HEVC, cuda_chroma_format(format.chroma), 0);
        let cuda = self.cuda;
        cuda.with_context(self.primary.context, || {
            cuda.check("cuvidGetDecoderCaps", unsafe {
                (cuda.get_decoder_caps)(&mut caps)
            })
        })?;
        let (width, height) = (format.coded_width, format.coded_height);
        let macroblocks = (width / 16).saturating_mul(height / 16);
        let fits = caps.is_supported != 0
            && caps.output_format_mask & (1 << cuda_surface_format(format.chroma)) != 0
            && (u32::from(caps.min_width)..=caps.max_width).contains(&width)
            && (u32::from(caps.min_height)..=caps.max_height).contains(&height)
            && macroblocks <= caps.max_mb_count;
        if fits {
            return Ok(());
        }
        Err(NvdecError::Unsupported(format!(
            "HEVC {:?} at {width}x{height}",
            format.chroma
        )))
    }
}
