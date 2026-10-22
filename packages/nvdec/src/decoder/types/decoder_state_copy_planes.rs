use std::ptr;

use super::{DecoderState, StreamFormat};
use crate::{
    CuDevicePtr, CudaMemcpy2d, MEMORY_TYPE_DEVICE, MEMORY_TYPE_HOST, NvdecError, PlaneBuffer,
};

impl DecoderState {
    pub(crate) fn copy_planes(
        &self,
        frame: CuDevicePtr,
        pitch: usize,
        format: &StreamFormat,
    ) -> Result<PlaneBuffer, NvdecError> {
        let planes = format.planes();
        let total = planes
            .iter()
            .map(|plane| plane.width_bytes * plane.rows)
            .sum();
        let mut buffer = self.pool.take(total);
        let plane_stride = pitch * (format.coded_height as usize).next_multiple_of(2);
        let mut offset = 0;
        for plane in planes {
            let copy = CudaMemcpy2d {
                src_x_in_bytes: plane.x_bytes,
                src_y: plane.y,
                src_memory_type: MEMORY_TYPE_DEVICE,
                src_host: ptr::null(),
                src_device: frame + (plane_stride * plane.index) as CuDevicePtr,
                src_array: ptr::null_mut(),
                src_pitch: pitch,
                dst_x_in_bytes: 0,
                dst_y: 0,
                dst_memory_type: MEMORY_TYPE_HOST,
                dst_host: buffer[offset..].as_mut_ptr().cast(),
                dst_device: 0,
                dst_array: ptr::null_mut(),
                dst_pitch: plane.width_bytes,
                width_in_bytes: plane.width_bytes,
                height: plane.rows,
            };
            self.cuda
                .check("cuMemcpy2D", unsafe { (self.cuda.memcpy_2d)(&copy) })?;
            offset += plane.width_bytes * plane.rows;
        }
        Ok(buffer)
    }
}
