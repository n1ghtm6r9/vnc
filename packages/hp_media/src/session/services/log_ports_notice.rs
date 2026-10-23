use ard_media::parse_stream_ports;
use log::{debug, warn};

use super::logged_bytes;

pub(crate) fn log_ports_notice(payload: &[u8]) {
    match parse_stream_ports(payload) {
        Some(ports) => debug!(
            "[MEDIA] the Mac streams from ports {} and {}",
            ports.audio_port, ports.video_port
        ),
        None => warn!(
            "[MEDIA] the Mac sent a ports notice of another layout: {:02x?}",
            logged_bytes(payload)
        ),
    }
}
