use super::{DisplayArea, StreamFormat};
use crate::{
    CHROMA_FORMAT_420, CHROMA_FORMAT_444, CODEC_HEVC, ChromaLayout, ColourMatrix, ColourPrimaries,
    ColourTransform, CuvidVideoFormat, NvdecError, VIDEO_FULL_RANGE_FLAG,
};

impl StreamFormat {
    pub(crate) fn read(video: &CuvidVideoFormat) -> Result<Self, NvdecError> {
        if video.codec != CODEC_HEVC {
            return Err(NvdecError::Unsupported(format!("codec {}", video.codec)));
        }
        let extra_bits = video
            .bit_depth_luma_minus8
            .max(video.bit_depth_chroma_minus8);
        if extra_bits != 0 {
            return Err(NvdecError::Unsupported(format!(
                "{}-bit video",
                8 + u32::from(extra_bits)
            )));
        }
        let chroma = match video.chroma_format {
            CHROMA_FORMAT_444 => ChromaLayout::Yuv444,
            CHROMA_FORMAT_420 => ChromaLayout::Nv12,
            other => return Err(NvdecError::Unsupported(format!("chroma format {other}"))),
        };
        if video.min_num_decode_surfaces == 0 {
            return Err(NvdecError::Unsupported(String::from(
                "a stream the driver did not size",
            )));
        }
        let full_range = video.video_signal_flags & VIDEO_FULL_RANGE_FLAG != 0;
        Ok(Self {
            coded_width: video.coded_width,
            coded_height: video.coded_height,
            chroma,
            decode_surfaces: video.min_num_decode_surfaces,
            display: DisplayArea::read(video),
            colour: ColourTransform::new(
                ColourMatrix::from_code(video.matrix_coefficients),
                full_range,
                ColourPrimaries::from_code(video.colour_primaries),
            ),
        })
    }
}
