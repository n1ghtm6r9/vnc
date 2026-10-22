use std::{collections::VecDeque, time::Instant};

use ard_media::MediaSecrets;

use super::Datagram;

pub(crate) struct PendingOffer {
    pub(crate) secrets: MediaSecrets,
    pub(crate) deadline: Instant,
    pub(crate) early: VecDeque<Datagram>,
    pub(crate) resize: bool,
}
