use std::net::IpAddr;

use super::MediaPorts;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MediaOptions {
    pub server: IpAddr,
    pub ports: MediaPorts,
    pub play_sound: bool,
}
