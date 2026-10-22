use super::be_u16;
use crate::{KIND_STREAM_PORTS, StreamPorts};

pub fn parse_stream_ports(payload: &[u8]) -> Option<StreamPorts> {
    let length = usize::from(be_u16(payload.get(..2)?, 0));
    if length < 20 || payload.len() < 2 + length || be_u16(payload, 2) != KIND_STREAM_PORTS {
        return None;
    }
    Some(StreamPorts {
        audio_port: be_u16(payload, 10),
        video_port: be_u16(payload, 16),
    })
}
