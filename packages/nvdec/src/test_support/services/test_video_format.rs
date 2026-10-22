use crate::{CHROMA_FORMAT_444, CODEC_HEVC, CuvidVideoFormat};

pub fn test_video_format() -> CuvidVideoFormat {
    CuvidVideoFormat {
        codec: CODEC_HEVC,
        frame_rate_numerator: 60,
        frame_rate_denominator: 1,
        progressive_sequence: 1,
        bit_depth_luma_minus8: 0,
        bit_depth_chroma_minus8: 0,
        min_num_decode_surfaces: 5,
        coded_width: 208,
        coded_height: 160,
        display_left: 0,
        display_top: 0,
        display_right: 200,
        display_bottom: 150,
        chroma_format: CHROMA_FORMAT_444,
        bitrate: 0,
        display_aspect_ratio_x: 4,
        display_aspect_ratio_y: 3,
        video_signal_flags: 0,
        colour_primaries: 1,
        transfer_characteristics: 1,
        matrix_coefficients: 1,
        seqhdr_data_length: 0,
    }
}
