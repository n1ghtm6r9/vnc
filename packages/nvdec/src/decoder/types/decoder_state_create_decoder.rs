use std::ptr;

use super::{DecoderState, StreamFormat};
use crate::{
    CODEC_HEVC, CREATE_PREFER_CUVID, CuvidDecodeCreateInfo, DEINTERLACE_WEAVE, NvdecError,
    OUTPUT_SURFACES, cuda_chroma_format, cuda_surface_format,
};

impl DecoderState {
    pub(crate) fn create_decoder(&mut self, format: &StreamFormat) -> Result<(), NvdecError> {
        self.check_capabilities(format)?;
        let (width, height) = (format.coded_width, format.coded_height);
        let mut info = CuvidDecodeCreateInfo {
            width,
            height,
            num_decode_surfaces: u32::from(format.decode_surfaces),
            codec_type: CODEC_HEVC,
            chroma_format: cuda_chroma_format(format.chroma),
            creation_flags: CREATE_PREFER_CUVID,
            bit_depth_minus8: 0,
            intra_decode_only: 0,
            max_width: width,
            max_height: height,
            reserved1: 0,
            display_area: [0; 4],
            output_format: cuda_surface_format(format.chroma),
            deinterlace_mode: DEINTERLACE_WEAVE,
            target_width: width,
            target_height: height,
            num_output_surfaces: OUTPUT_SURFACES,
            vid_lock: ptr::null_mut(),
            target_rect: [0; 4],
            enable_histogram: 0,
            enable_decode_features: 0,
            reserved2: [0; 3],
        };
        let mut decoder = ptr::null_mut();
        let cuda = self.cuda;
        cuda.with_context(self.primary.context, || {
            cuda.check("cuvidCreateDecoder", unsafe {
                (cuda.create_decoder)(&mut decoder, &mut info)
            })
        })?;
        self.decoder = decoder;
        log::info!(
            "[DECODE] NVDEC decodes {:?} {}x{} pictures shown at {}x{} through {} surfaces",
            format.chroma,
            width,
            height,
            format.display.width,
            format.display.height,
            format.decode_surfaces
        );
        Ok(())
    }
}
