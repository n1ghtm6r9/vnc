use std::ffi::c_ulong;

#[repr(C)]
pub(crate) struct CuvidSourceDataPacket {
    pub(crate) flags: c_ulong,
    pub(crate) payload_size: c_ulong,
    pub(crate) payload: *const u8,
    pub(crate) timestamp: i64,
}
