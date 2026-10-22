use std::mem;

use super::{DecodedFrame, HevcDecoder};
use crate::{CuvidSourceDataPacket, NvdecError, PACKET_END_OF_PICTURE};

impl HevcDecoder {
    pub fn decode(&mut self, access_unit: &[u8]) -> Result<Vec<DecodedFrame>, NvdecError> {
        if access_unit.is_empty() {
            return Ok(Vec::new());
        }
        let payload_size = u32::try_from(access_unit.len()).map_err(|_| {
            NvdecError::Unsupported(format!("a picture of {} bytes", access_unit.len()))
        })?;
        let mut packet = CuvidSourceDataPacket {
            flags: PACKET_END_OF_PICTURE,
            payload_size,
            payload: access_unit.as_ptr(),
            timestamp: 0,
        };
        let (cuda, context) = unsafe { ((*self.state).cuda, (*self.state).primary.context) };
        let parser = self.parser;
        let parsed = cuda.with_context(context, || {
            cuda.check("cuvidParseVideoData", unsafe {
                (cuda.parse_video_data)(parser, &mut packet)
            })
        });
        let state = unsafe { &mut *self.state };
        let frames = mem::take(&mut state.frames);
        match state.error.take() {
            Some(error) => Err(error),
            None => parsed.map(|()| frames),
        }
    }
}
