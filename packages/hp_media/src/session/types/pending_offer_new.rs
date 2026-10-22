use std::{collections::VecDeque, time::Instant};

use ard_media::MediaSecrets;

use super::PendingOffer;
use crate::ANSWER_TIMEOUT;

impl PendingOffer {
    pub(crate) fn new(secrets: MediaSecrets, now: Instant) -> PendingOffer {
        PendingOffer {
            secrets,
            deadline: now + ANSWER_TIMEOUT,
            early: VecDeque::new(),
            resize: false,
        }
    }
}
