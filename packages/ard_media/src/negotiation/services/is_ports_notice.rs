use crate::KIND_STREAM_PORTS;

pub fn is_ports_notice(payload: &[u8]) -> bool {
    payload.get(2..4) == Some(KIND_STREAM_PORTS.to_be_bytes().as_slice())
}
