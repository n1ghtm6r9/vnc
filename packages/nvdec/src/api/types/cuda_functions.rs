use std::ffi::{c_char, c_int, c_uint};

use crate::{
    CuContext, CuDevice, CuDevicePtr, CuResult, CuVideoDecoder, CuVideoParser, CudaMemcpy2d,
    CuvidDecodeCaps, CuvidDecodeCreateInfo, CuvidParserParams, CuvidPicParams, CuvidProcParams,
    CuvidSourceDataPacket,
};

#[derive(Clone, Copy)]
pub(crate) struct CudaFunctions {
    pub(crate) init: unsafe extern "system" fn(c_uint) -> CuResult,
    pub(crate) device_get: unsafe extern "system" fn(*mut CuDevice, c_int) -> CuResult,
    pub(crate) device_primary_ctx_retain:
        unsafe extern "system" fn(*mut CuContext, CuDevice) -> CuResult,
    pub(crate) device_primary_ctx_release: unsafe extern "system" fn(CuDevice) -> CuResult,
    pub(crate) ctx_push_current: unsafe extern "system" fn(CuContext) -> CuResult,
    pub(crate) ctx_pop_current: unsafe extern "system" fn(*mut CuContext) -> CuResult,
    pub(crate) memcpy_2d: unsafe extern "system" fn(*const CudaMemcpy2d) -> CuResult,
    pub(crate) get_error_name: unsafe extern "system" fn(CuResult, *mut *const c_char) -> CuResult,
    pub(crate) get_decoder_caps: unsafe extern "system" fn(*mut CuvidDecodeCaps) -> CuResult,
    pub(crate) create_decoder:
        unsafe extern "system" fn(*mut CuVideoDecoder, *mut CuvidDecodeCreateInfo) -> CuResult,
    pub(crate) destroy_decoder: unsafe extern "system" fn(CuVideoDecoder) -> CuResult,
    pub(crate) decode_picture:
        unsafe extern "system" fn(CuVideoDecoder, *mut CuvidPicParams) -> CuResult,
    pub(crate) map_video_frame: unsafe extern "system" fn(
        CuVideoDecoder,
        c_int,
        *mut CuDevicePtr,
        *mut c_uint,
        *mut CuvidProcParams,
    ) -> CuResult,
    pub(crate) unmap_video_frame:
        unsafe extern "system" fn(CuVideoDecoder, CuDevicePtr) -> CuResult,
    pub(crate) create_video_parser:
        unsafe extern "system" fn(*mut CuVideoParser, *mut CuvidParserParams) -> CuResult,
    pub(crate) parse_video_data:
        unsafe extern "system" fn(CuVideoParser, *mut CuvidSourceDataPacket) -> CuResult,
    pub(crate) destroy_video_parser: unsafe extern "system" fn(CuVideoParser) -> CuResult,
}
