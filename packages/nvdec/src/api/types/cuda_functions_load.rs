use super::{CudaFunctions, CudaLibraries};
use crate::{NvdecError, symbol};

impl CudaFunctions {
    pub(crate) fn load(libraries: &CudaLibraries) -> Result<Self, NvdecError> {
        let cuda = &libraries.cuda;
        let cuvid = &libraries.cuvid;
        Ok(Self {
            init: symbol(cuda, "cuInit")?,
            device_get: symbol(cuda, "cuDeviceGet")?,
            device_primary_ctx_retain: symbol(cuda, "cuDevicePrimaryCtxRetain")?,
            device_primary_ctx_release: symbol(cuda, "cuDevicePrimaryCtxRelease_v2")?,
            ctx_push_current: symbol(cuda, "cuCtxPushCurrent_v2")?,
            ctx_pop_current: symbol(cuda, "cuCtxPopCurrent_v2")?,
            memcpy_2d: symbol(cuda, "cuMemcpy2D_v2")?,
            get_error_name: symbol(cuda, "cuGetErrorName")?,
            get_decoder_caps: symbol(cuvid, "cuvidGetDecoderCaps")?,
            create_decoder: symbol(cuvid, "cuvidCreateDecoder")?,
            destroy_decoder: symbol(cuvid, "cuvidDestroyDecoder")?,
            decode_picture: symbol(cuvid, "cuvidDecodePicture")?,
            map_video_frame: symbol(cuvid, "cuvidMapVideoFrame64")?,
            unmap_video_frame: symbol(cuvid, "cuvidUnmapVideoFrame64")?,
            create_video_parser: symbol(cuvid, "cuvidCreateVideoParser")?,
            parse_video_data: symbol(cuvid, "cuvidParseVideoData")?,
            destroy_video_parser: symbol(cuvid, "cuvidDestroyVideoParser")?,
        })
    }
}
